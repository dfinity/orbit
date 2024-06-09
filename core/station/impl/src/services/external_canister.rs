use crate::core::validation::EnsureExternalCanister;
use crate::errors::ExternalCanisterError;
use candid::Principal;
use ic_cdk::api::call::call_raw;
use ic_cdk::api::management_canister::main::{
    self as mgmt, CanisterIdRecord, CanisterStatusResponse, CreateCanisterArgument,
};
use lazy_static::lazy_static;
use orbit_essentials::api::ServiceResult;
use std::sync::Arc;

lazy_static! {
    pub static ref EXTERNAL_CANISTER_SERVICE: Arc<ExternalCanisterService> =
        Arc::new(ExternalCanisterService::default());
}

const CREATE_CANISTER_CYCLES: u128 = 100_000_000_000; // the default fee of 100 B cycles

#[derive(Default, Debug)]
pub struct ExternalCanisterService {}

impl ExternalCanisterService {
    pub async fn create_canister(&self) -> ServiceResult<Principal, ExternalCanisterError> {
        let create_canister_arg = CreateCanisterArgument { settings: None };

        let canister_id = mgmt::create_canister(create_canister_arg, CREATE_CANISTER_CYCLES)
            .await
            .map_err(|(_, err)| ExternalCanisterError::Failed {
                reason: err.to_string(),
            })?
            .0
            .canister_id;

        Ok(canister_id)
    }

    pub async fn canister_status(
        &self,
        input: CanisterIdRecord,
    ) -> ServiceResult<CanisterStatusResponse> {
        let canister_status_arg = CanisterIdRecord {
            canister_id: input.canister_id,
        };

        let canister_status_response = mgmt::canister_status(canister_status_arg)
            .await
            .map_err(|(_, err)| ExternalCanisterError::Failed {
                reason: err.to_string(),
            })?
            .0;

        Ok(canister_status_response)
    }

    pub async fn call_canister(
        &self,
        canister_id: Principal,
        method_name: String,
        arg: Vec<u8>,
        cycles: Option<u64>,
    ) -> ServiceResult<Vec<u8>, ExternalCanisterError> {
        EnsureExternalCanister::ensure_external_canister(canister_id)?;

        call_raw(canister_id, &method_name, arg, cycles.unwrap_or_default())
            .await
            .map_err(|(_, err)| ExternalCanisterError::Failed {
                reason: err.to_string(),
            })
    }
}
