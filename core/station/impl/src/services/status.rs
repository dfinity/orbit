use crate::errors::StatusError;
use ic_cdk::api::management_canister::main::{
    self as mgmt, CanisterIdRecord, CanisterStatusResponse,
};
use lazy_static::lazy_static;
use orbit_essentials::api::ServiceResult;
use std::sync::Arc;

lazy_static! {
    pub static ref STATUS_SERVICE: Arc<StatusService> = Arc::new(StatusService::default());
}

#[derive(Default, Debug)]
pub struct StatusService {}

impl StatusService {
    pub async fn canister_status(
        &self,
        input: CanisterIdRecord,
    ) -> ServiceResult<CanisterStatusResponse> {
        let canister_status_arg = CanisterIdRecord {
            canister_id: input.canister_id,
        };

        let canister_status_response = mgmt::canister_status(canister_status_arg)
            .await
            .map_err(|(_, err)| StatusError::Failed {
                reason: err.to_string(),
            })?
            .0;

        Ok(canister_status_response)
    }
}
