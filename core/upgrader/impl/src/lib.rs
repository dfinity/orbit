use crate::model::{DisasterRecovery, DisasterRecoveryV0, LogEntry};
use crate::services::insert_logs;
use crate::upgrade::{
    CheckController, Upgrade, Upgrader, WithAuthorization, WithBackground, WithLogs, WithSnapshot,
    WithStart, WithStop,
};
use candid::Principal;
use ic_cdk::api::stable::{stable_size, stable_write};
use ic_cdk::{
    api::management_canister::main::CanisterInstallMode, init, post_upgrade, trap, update,
};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    storable::Bound,
    DefaultMemoryImpl, StableBTreeMap, Storable,
};
use lazy_static::lazy_static;
use orbit_essentials::storable;
use orbit_essentials::types::Timestamp;
use std::{borrow::Cow, cell::RefCell, collections::BTreeMap, sync::Arc};
use upgrade::{UpgradeError, UpgradeParams};
use upgrader_api::{InitArg, TriggerUpgradeError};

#[cfg(not(test))]
pub use orbit_essentials::cdk as upgrader_ic_cdk;
#[cfg(test)]
pub use orbit_essentials::cdk::mocks as upgrader_ic_cdk;

pub mod controllers;
pub mod errors;
pub mod mappers;
pub mod model;
pub mod services;
pub mod upgrade;
pub mod utils;

type Memory = VirtualMemory<DefaultMemoryImpl>;
type StableMap<K, V> = StableBTreeMap<K, V, Memory>;
type StableValue<T> = StableMap<(), T>;

/// Represents one mebibyte.
pub const MIB: u32 = 1 << 20;

/// Canisters use 64KiB pages for Wasm memory, more details in the PR that introduced this constant:
/// - https://github.com/WebAssembly/design/pull/442#issuecomment-153203031
pub const WASM_PAGE_SIZE: u32 = 65536;

/// The size of the stable memory bucket in WASM pages.
///
/// We use a bucket size of 1MiB to ensure that the default memory allocated to the canister is as small as possible,
/// this is due to the fact that this cansiter uses several MemoryIds to manage the stable memory similarly to to how
/// a database arranges data per table.
///
/// Currently a bucket size of 1MiB limits the canister to 32GiB of stable memory, which is more than enough for the
/// current use case, however, if the canister needs more memory in the future, `ic-stable-structures` will need to be
/// updated to support storing more buckets in a backwards compatible way.
pub const STABLE_MEMORY_BUCKET_SIZE: u16 = (MIB / WASM_PAGE_SIZE) as u16;

/// Current version of stable memory layout.
pub const STABLE_MEMORY_VERSION: u32 = 1;

const MEMORY_ID_STATE: u8 = 0;
const MEMORY_ID_LOGS: u8 = 1;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init_with_bucket_size(DefaultMemoryImpl::default(), STABLE_MEMORY_BUCKET_SIZE));
    static STATE: RefCell<StableValue<State>> = RefCell::new(
        StableValue::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(MEMORY_ID_STATE))),
        )
    );
}

#[storable]
struct State {
    target_canister: Principal,
    backup_snapshot_id: Option<Vec<u8>>,
    disaster_recovery: DisasterRecovery,
    stable_memory_version: u32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            target_canister: Principal::anonymous(),
            backup_snapshot_id: None,
            disaster_recovery: Default::default(),
            stable_memory_version: STABLE_MEMORY_VERSION,
        }
    }
}

fn get_state() -> State {
    STATE.with(|storage| storage.borrow().get(&()).unwrap_or_default())
}

fn set_state(state: State) {
    STATE.with(|storage| storage.borrow_mut().insert((), state));
}

pub fn get_target_canister() -> Principal {
    get_state().target_canister
}

fn set_target_canister(target_canister: Principal) {
    let mut state = get_state();
    state.target_canister = target_canister;
    set_state(state);
}

pub fn get_backup_snapshot_id() -> Option<Vec<u8>> {
    get_state().backup_snapshot_id
}

fn set_backup_snapshot_id(backup_snapshot_id: Vec<u8>) {
    let mut state = get_state();
    state.backup_snapshot_id = Some(backup_snapshot_id);
    set_state(state);
}

pub fn get_disaster_recovery() -> DisasterRecovery {
    get_state().disaster_recovery
}

pub fn set_disaster_recovery(value: DisasterRecovery) {
    let mut state = get_state();
    state.disaster_recovery = value;
    set_state(state);
}

#[init]
fn init_fn(InitArg { target_canister }: InitArg) {
    set_target_canister(target_canister);
}

#[post_upgrade]
fn post_upgrade() {
    pub struct RawBytes(pub Vec<u8>);
    impl Storable for RawBytes {
        fn to_bytes(&self) -> Cow<[u8]> {
            trap("RawBytes should never be serialized")
        }

        fn from_bytes(bytes: Cow<[u8]>) -> Self {
            Self(bytes.to_vec())
        }

        const BOUND: Bound = Bound::Unbounded;
    }

    const OLD_MEMORY_ID_TARGET_CANISTER_ID: u8 = 0;
    const OLD_MEMORY_ID_DISASTER_RECOVERY: u8 = 1;
    const OLD_MEMORY_ID_LOGS: u8 = 4;

    let old_memory_manager = MemoryManager::init(DefaultMemoryImpl::default());

    // determine stable memory layout by trying to parse the target canister from memory with OLD_MEMORY_ID_TARGET_CANISTER_ID
    let old_target_canister_bytes: StableValue<RawBytes> =
        StableValue::init(old_memory_manager.get(MemoryId::new(OLD_MEMORY_ID_TARGET_CANISTER_ID)));
    let target_canister_bytes = old_target_canister_bytes
        .get(&())
        .unwrap_or_else(|| trap("Could not determine stable memory layout."));
    // if a principal can be parsed out of memory with OLD_MEMORY_ID_TARGET_CANISTER_ID
    // then we need to perform stable memory migration
    if let Ok(target_canister) = serde_cbor::from_slice::<Principal>(&target_canister_bytes.0) {
        let old_disaster_recovery: StableValue<DisasterRecoveryV0> = StableValue::init(
            old_memory_manager.get(MemoryId::new(OLD_MEMORY_ID_DISASTER_RECOVERY)),
        );
        let disaster_recovery: DisasterRecoveryV0 =
            old_disaster_recovery.get(&()).unwrap_or_default();

        let old_logs: StableBTreeMap<Timestamp, LogEntry, Memory> =
            StableBTreeMap::init(old_memory_manager.get(MemoryId::new(OLD_MEMORY_ID_LOGS)));
        let logs: BTreeMap<Timestamp, LogEntry> = old_logs.iter().collect();

        // clear the stable memory
        let stable_memory_size_bytes = stable_size() * (WASM_PAGE_SIZE as u64);
        stable_write(0, &vec![0; stable_memory_size_bytes as usize]);

        let state = State {
            target_canister,
            backup_snapshot_id: None,
            disaster_recovery: disaster_recovery.into(),
            stable_memory_version: STABLE_MEMORY_VERSION,
        };
        set_state(state);
        insert_logs(logs);
    }
}

lazy_static! {
    static ref UPGRADER: Box<dyn Upgrade> = {
        let u = Upgrader {};
        let u = WithSnapshot(u);
        let u = WithStop(u);
        let u = WithStart(u);
        let u = WithLogs(u, "upgrade".to_string());
        let u = WithBackground(Arc::new(u));
        let u = CheckController(u);
        let u = WithAuthorization(u);
        let u = WithLogs(u, "trigger_upgrade".to_string());
        Box::new(u)
    };
}

#[update]
async fn trigger_upgrade(params: upgrader_api::UpgradeParams) -> Result<(), TriggerUpgradeError> {
    let input: UpgradeParams = UpgradeParams {
        module: params.module,
        module_extra_chunks: params.module_extra_chunks,
        arg: params.arg,
        install_mode: CanisterInstallMode::Upgrade(None),
        backup_snapshot: params.backup_snapshot.unwrap_or_default(),
    };
    UPGRADER.upgrade(input).await.map_err(|err| match err {
        UpgradeError::NotController => TriggerUpgradeError::NotController,
        UpgradeError::Unauthorized => TriggerUpgradeError::Unauthorized,
        UpgradeError::UnexpectedError(err) => TriggerUpgradeError::UnexpectedError(err.to_string()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbit_essentials::api::ApiResult;

    #[test]
    fn check_candid_interface() {
        use candid_parser::utils::{service_equal, CandidSource};

        candid::export_service!();
        let new_interface = __export_service();

        service_equal(
            CandidSource::Text(&new_interface),
            CandidSource::Text(include_str!("../../api/spec.did")),
        )
        .unwrap();
    }
}
