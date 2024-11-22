use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{PruneExternalCanisterOperation, Request, RequestOperation},
    services::ChangeCanisterService,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;
use station_api::{CreateRequestInput, PruneExternalCanisterOperationInput};
use std::sync::Arc;

pub struct PruneExternalCanisterRequestCreate;

#[async_trait]
impl Create<PruneExternalCanisterOperationInput> for PruneExternalCanisterRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: CreateRequestInput,
        operation_input: PruneExternalCanisterOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::PruneExternalCanister(PruneExternalCanisterOperation {
                input: operation_input.into(),
            }),
            "Prune canister".to_string(),
        );

        Ok(request)
    }
}

pub struct PruneExternalCanisterRequestExecute<'o> {
    operation: &'o PruneExternalCanisterOperation,
    change_canister_service: Arc<ChangeCanisterService>,
}

impl<'o> PruneExternalCanisterRequestExecute<'o> {
    pub fn new(
        operation: &'o PruneExternalCanisterOperation,
        change_canister_service: Arc<ChangeCanisterService>,
    ) -> Self {
        Self {
            operation,
            change_canister_service,
        }
    }
}

#[async_trait]
impl Execute for PruneExternalCanisterRequestExecute<'_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        self.change_canister_service
            .prune_canister(
                self.operation.input.canister_id,
                self.operation.input.prune.clone(),
            )
            .await
            .map_err(|err| RequestExecuteError::Failed {
                reason: format!(
                    "failed to prune {} on external canister {}: {}",
                    self.operation.input.canister_id, self.operation.input.prune, err
                ),
            })?;

        Ok(RequestExecuteStage::Completed(
            RequestOperation::PruneExternalCanister(self.operation.clone()),
        ))
    }
}
