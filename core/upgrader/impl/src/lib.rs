use crate::upgrade::{
    CheckController, Upgrade, Upgrader, WithAuthorization, WithBackground, WithLogs, WithStart,
    WithStop,
};
use candid::Principal;
use disaster_recovery::DisasterRecovery;
use ic_cdk::{init, query, update};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};
use lazy_static::lazy_static;
use orbit_essentials::{
    api::{ApiError, ApiResult},
    cdk::{api::is_controller, caller},
    storable,
};
use std::{cell::RefCell, sync::Arc, thread::LocalKey};
use upgrade::UpgradeError;
use upgrader_api::{
    GetDisasterRecoveryAccountsResponse, GetDisasterRecoveryCommitteeResponse, InitArg,
    IsCommitteeMemberResponse, TriggerUpgradeError, UpgradeParams,
};

mod disaster_recovery;
mod hash;
mod helper;
mod upgrade;

type Memory = VirtualMemory<DefaultMemoryImpl>;
type StableMap<K, V> = StableBTreeMap<K, V, Memory>;
type StableValue<T> = StableMap<(), T>;
type LocalRef<T> = &'static LocalKey<RefCell<T>>;

const MEMORY_ID_TARGET_CANISTER_ID: u8 = 0;
const MEMORY_ID_DISASTER_RECOVERY_ID: u8 = 1;

enum UpgraderApiError {
    NotController,
    Unauthorized,
}

impl From<UpgraderApiError> for ApiError {
    fn from(err: UpgraderApiError) -> Self {
        match err {
            UpgraderApiError::NotController => ApiError {
                code: "NOT_CONTROLLER".to_owned(),
                message: Some("Caller is not the controller.".to_owned()),
                details: None,
            },
            UpgraderApiError::Unauthorized => ApiError {
                code: "UNAUTHORIZED".to_owned(),
                message: Some("Caller is not authorized.".to_owned()),
                details: None,
            },
        }
    }
}

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
        let u = WithBackground(Arc::new(u));
        let u = CheckController(u, &TARGET_CANISTER_ID);
        let u = WithAuthorization(u, &TARGET_CANISTER_ID);
        let u = WithLogs(u, "trigger_upgrade".to_string());
        Box::new(u)
    };
}

#[update]
async fn trigger_upgrade(params: UpgradeParams) -> Result<(), TriggerUpgradeError> {
    UPGRADER.upgrade(params).await.map_err(|err| match err {
        UpgradeError::NotController => TriggerUpgradeError::NotController,
        UpgradeError::Unauthorized => TriggerUpgradeError::Unauthorized,
        UpgradeError::UnexpectedError(err) => TriggerUpgradeError::UnexpectedError(err.to_string()),
    })
}

#[update]
async fn set_disaster_recovery_committee(
    input: upgrader_api::SetDisasterRecoveryCommitteeInput,
) -> ApiResult {
    let caller = caller();
    if !is_controller(&caller) {
        Err(UpgraderApiError::NotController)?
    } else {
        DisasterRecovery::set_committee(input.committee.into());
        Ok(())
    }
}

#[update]
async fn set_disaster_recovery_accounts(
    input: upgrader_api::SetDisasterRecoveryAccountsInput,
) -> ApiResult {
    let caller = caller();
    if !is_controller(&caller) {
        Err(UpgraderApiError::NotController)?
    } else {
        DisasterRecovery::set_accounts(input.accounts.into_iter().map(Into::into).collect());
        Ok(())
    }
}

#[query]
async fn is_committee_member() -> ApiResult<IsCommitteeMemberResponse> {
    let caller = ic_cdk::caller();

    if caller == Principal::anonymous() {
        Err(UpgraderApiError::Unauthorized)?
    } else {
        Ok(IsCommitteeMemberResponse {
            is_committee_member: DisasterRecovery::is_committee_member(&caller),
        })
    }
}

#[query]
async fn get_disaster_recovery_accounts() -> ApiResult<GetDisasterRecoveryAccountsResponse> {
    let caller = caller();
    if !is_controller(&caller) {
        Err(UpgraderApiError::NotController)?
    } else {
        Ok(GetDisasterRecoveryAccountsResponse {
            accounts: DisasterRecovery::get()
                .accounts
                .into_iter()
                .map(Into::into)
                .collect(),
        })
    }
}

#[query]
async fn get_disaster_recovery_committee() -> ApiResult<GetDisasterRecoveryCommitteeResponse> {
    let caller = caller();
    if !is_controller(&caller) {
        Err(UpgraderApiError::NotController)?
    } else {
        Ok(GetDisasterRecoveryCommitteeResponse {
            committee: DisasterRecovery::get().committee.map(Into::into),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
