use crate::model::DisasterRecovery;
use crate::upgrade::{
    CheckController, Upgrade, Upgrader, WithAuthorization, WithBackground, WithLogs, WithSnapshot,
    WithStart, WithStop,
};
use candid::Principal;
use ic_cdk::api::management_canister::main::CanisterInstallMode;
use ic_cdk::{init, post_upgrade, update};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};
use lazy_static::lazy_static;
use orbit_essentials::backup_snapshots::BackupSnapshots;
use orbit_essentials::storable;
use std::{cell::RefCell, sync::Arc};
use upgrade::{ChangeParams, RestoreParams, UpgradeError, UpgradeParams};
use upgrader_api::{InitArg, TriggerRestoreError, TriggerUpgradeError};

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
    #[serde(default)]
    backup_snapshots: BackupSnapshots,
    disaster_recovery: DisasterRecovery,
    stable_memory_version: u32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            target_canister: Principal::anonymous(),
            backup_snapshots: BackupSnapshots::new(1),
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

pub fn get_backup_snapshot_to_replace() -> Option<Vec<u8>> {
    get_state().backup_snapshots.get_snapshot_to_replace()
}

fn insert_backup_snapshot(snapshot_id: Vec<u8>) {
    let mut state = get_state();
    state.backup_snapshots.insert_snapshot(snapshot_id);
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
    // basic health check
    let _ = get_state();
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
        take_backup_snapshot: params.take_backup_snapshot.unwrap_or_default(),
    };
    UPGRADER
        .upgrade(ChangeParams::Upgrade(input))
        .await
        .map_err(|err| match err {
            UpgradeError::NotController => TriggerUpgradeError::NotController,
            UpgradeError::Unauthorized => TriggerUpgradeError::Unauthorized,
            UpgradeError::UnexpectedError(err) => {
                TriggerUpgradeError::UnexpectedError(err.to_string())
            }
        })
}

#[update]
async fn trigger_restore(params: upgrader_api::RestoreParams) -> Result<(), TriggerRestoreError> {
    let input: RestoreParams = RestoreParams {
        snapshot_id: params.snapshot_id,
    };
    UPGRADER
        .upgrade(ChangeParams::Restore(input))
        .await
        .map_err(|err| match err {
            UpgradeError::NotController => TriggerRestoreError::NotController,
            UpgradeError::Unauthorized => TriggerRestoreError::Unauthorized,
            UpgradeError::UnexpectedError(err) => {
                TriggerRestoreError::UnexpectedError(err.to_string())
            }
        })
}

#[update]
async fn set_max_backup_snapshots(max_backup_snapshots: u64) -> Result<(), String> {
    let id = get_target_canister();
    if ic_cdk::caller() != id {
        return Err(format!(
            "Only the target canister {} is authorized to call `set_max_backup_snapshots`.",
            id
        ));
    }

    let mut state = get_state();
    let res = state
        .backup_snapshots
        .set_max_backup_snapshots(max_backup_snapshots, id)
        .await;
    set_state(state);
    res
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
