use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{Request, RequestOperation, SnapshotExternalCanisterOperation},
    services::ChangeCanisterService,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;
use station_api::{CreateRequestInput, SnapshotExternalCanisterOperationInput};
use std::sync::Arc;

pub struct SnapshotExternalCanisterRequestCreate;

#[async_trait]
impl Create<SnapshotExternalCanisterOperationInput> for SnapshotExternalCanisterRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: CreateRequestInput,
        operation_input: SnapshotExternalCanisterOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::SnapshotExternalCanister(SnapshotExternalCanisterOperation {
                input: operation_input.into(),
                snapshot_id: None,
            }),
            "Snapshot canister".to_string(),
        );

        Ok(request)
    }
}

pub struct SnapshotExternalCanisterRequestExecute<'o> {
    operation: &'o SnapshotExternalCanisterOperation,
    change_canister_service: Arc<ChangeCanisterService>,
}

impl<'o> SnapshotExternalCanisterRequestExecute<'o> {
    pub fn new(
        operation: &'o SnapshotExternalCanisterOperation,
        change_canister_service: Arc<ChangeCanisterService>,
    ) -> Self {
        Self {
            operation,
            change_canister_service,
        }
    }
}

#[async_trait]
impl Execute for SnapshotExternalCanisterRequestExecute<'_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        let snapshot_id = self
            .change_canister_service
            .snapshot_canister(
                self.operation.input.canister_id,
                self.operation.input.replace_snapshot.clone(),
            )
            .await
            .map_err(|err| RequestExecuteError::Failed {
                reason: format!(
                    "failed to snapshot external canister {}: {}",
                    self.operation.input.canister_id, err
                ),
            })?;

        let mut snapshot_operation = self.operation.clone();
        snapshot_operation.snapshot_id = Some(snapshot_id);

        Ok(RequestExecuteStage::Completed(
            RequestOperation::SnapshotExternalCanister(snapshot_operation),
        ))
    }
}
