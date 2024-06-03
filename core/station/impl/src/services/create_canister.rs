use crate::errors::ChangeCanisterError;
use candid::Principal;
use ic_cdk::api::management_canister::main::{self as mgmt, CreateCanisterArgument};
use lazy_static::lazy_static;
use orbit_essentials::api::ServiceResult;
use std::sync::Arc;

lazy_static! {
    pub static ref CREATE_CANISTER_SERVICE: Arc<CreateCanisterService> =
        Arc::new(CreateCanisterService::default());
}

const CREATE_CANISTER_CYCLES: u128 = 100_000_000_000; // the default fee of 100 B cycles

#[derive(Default, Debug)]
pub struct CreateCanisterService {}

impl CreateCanisterService {
    /// Execute an install or upgrade of a canister.
    pub async fn create_canister(&self) -> ServiceResult<Principal, ChangeCanisterError> {
        let create_canister_arg = CreateCanisterArgument { settings: None };
        Ok(
            mgmt::create_canister(create_canister_arg, CREATE_CANISTER_CYCLES)
                .await
                .map_err(|(_, err)| ChangeCanisterError::Failed {
                    reason: err.to_string(),
                })?
                .0
                .canister_id,
        )
    }
}
