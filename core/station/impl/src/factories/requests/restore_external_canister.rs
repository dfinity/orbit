use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{Request, RequestOperation, RestoreExternalCanisterOperation},
    services::ChangeCanisterService,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;
use station_api::{CreateRequestInput, RestoreExternalCanisterOperationInput};
use std::sync::Arc;

pub struct RestoreExternalCanisterRequestCreate;

#[async_trait]
impl Create<RestoreExternalCanisterOperationInput> for RestoreExternalCanisterRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: CreateRequestInput,
        operation_input: RestoreExternalCanisterOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::RestoreExternalCanister(RestoreExternalCanisterOperation {
                input: operation_input.into(),
            }),
            "Restore canister".to_string(),
        );

        Ok(request)
    }
}

pub struct RestoreExternalCanisterRequestExecute<'o> {
    operation: &'o RestoreExternalCanisterOperation,
    change_canister_service: Arc<ChangeCanisterService>,
}

impl<'o> RestoreExternalCanisterRequestExecute<'o> {
    pub fn new(
        operation: &'o RestoreExternalCanisterOperation,
        change_canister_service: Arc<ChangeCanisterService>,
    ) -> Self {
        Self {
            operation,
            change_canister_service,
        }
    }
}

#[async_trait]
impl Execute for RestoreExternalCanisterRequestExecute<'_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        self.change_canister_service
            .restore_canister(
                self.operation.input.canister_id,
                self.operation.input.snapshot_id.clone(),
            )
            .await
            .map_err(|err| RequestExecuteError::Failed {
                reason: format!(
                    "failed to restore external canister {} from snapshot {}: {}",
                    self.operation.input.canister_id,
                    hex::encode(&self.operation.input.snapshot_id),
                    err
                ),
            })?;

        Ok(RequestExecuteStage::Completed(
            RequestOperation::RestoreExternalCanister(self.operation.clone()),
        ))
    }
}
