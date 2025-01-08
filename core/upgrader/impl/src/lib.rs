use crate::upgrade::{
    CheckController, Upgrade, Upgrader, WithAuthorization, WithBackground, WithLogs, WithStart,
    WithStop,
};
use candid::Principal;
use ic_cdk::{api::management_canister::main::CanisterInstallMode, init, update};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};
use lazy_static::lazy_static;
use orbit_essentials::storable;
use std::{cell::RefCell, sync::Arc, thread::LocalKey};
use upgrade::{UpgradeError, UpgradeParams};
use upgrader_api::{InitArg, TriggerUpgradeError};

#[cfg(not(test))]
pub use orbit_essentials::cdk as upgrader_ic_cdk;
#[cfg(test)]
pub use orbit_essentials::cdk::mocks as upgrader_ic_cdk;

pub mod controllers;
pub mod errors;
pub mod model;
pub mod services;
pub mod upgrade;
pub mod utils;

type Memory = VirtualMemory<DefaultMemoryImpl>;
type StableMap<K, V> = StableBTreeMap<K, V, Memory>;
type StableValue<T> = StableMap<(), T>;
type LocalRef<T> = &'static LocalKey<RefCell<T>>;

const MEMORY_ID_TARGET_CANISTER_ID: u8 = 0;
const MEMORY_ID_DISASTER_RECOVERY: u8 = 1;
const MEMORY_ID_LOGS: u8 = 4;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

#[storable]
pub struct StorablePrincipal(Principal);

thread_local! {
    static TARGET_CANISTER_ID: RefCell<StableValue<StorablePrincipal>> = RefCell::new(
        StableValue::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(MEMORY_ID_TARGET_CANISTER_ID))),
        )
    );
}

#[init]
fn init_fn(InitArg { target_canister }: InitArg) {
    TARGET_CANISTER_ID.with(|id| {
        let mut id = id.borrow_mut();
        id.insert((), StorablePrincipal(target_canister));
    });
}

lazy_static! {
    static ref UPGRADER: Box<dyn Upgrade> = {
        let u = Upgrader::new(&TARGET_CANISTER_ID);
        let u = WithStop(u, &TARGET_CANISTER_ID);
        let u = WithStart(u, &TARGET_CANISTER_ID);
        let u = WithLogs(u, "upgrade".to_string());
        let u = WithBackground(Arc::new(u), &TARGET_CANISTER_ID);
        let u = CheckController(u, &TARGET_CANISTER_ID);
        let u = WithAuthorization(u, &TARGET_CANISTER_ID);
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
