use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{Request, RequestOperation, SystemRestoreOperation, SystemRestoreTarget},
    services::SystemService,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;
use station_api::{CreateRequestInput, SystemRestoreOperationInput};
use std::sync::Arc;

pub struct SystemRestoreRequestCreate;

#[async_trait]
impl Create<SystemRestoreOperationInput> for SystemRestoreRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: CreateRequestInput,
        operation_input: SystemRestoreOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::SystemRestore(SystemRestoreOperation {
                input: operation_input.into(),
            }),
            "Restore System".to_string(),
        );

        Ok(request)
    }
}

pub struct SystemRestoreRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o SystemRestoreOperation,
    system_service: Arc<SystemService>,
}

impl<'p, 'o> SystemRestoreRequestExecute<'p, 'o> {
    pub fn new(
        request: &'p Request,
        operation: &'o SystemRestoreOperation,
        system_service: Arc<SystemService>,
    ) -> Self {
        Self {
            request,
            operation,
            system_service,
        }
    }
}

#[async_trait]
impl Execute for SystemRestoreRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        match self.operation.input.target {
            SystemRestoreTarget::RestoreStation => {
                self.system_service
                    .set_self_upgrade_request(self.request.id);

                let out = self
                    .system_service
                    .restore_station(self.operation.input.snapshot_id.clone())
                    .await
                    .map_err(|err| RequestExecuteError::Failed {
                        reason: err
                            .details
                            .as_ref()
                            .and_then(|details| details.get("reason").cloned())
                            .unwrap_or(err.to_string()),
                    });

                if out.is_err() {
                    self.system_service.clear_self_upgrade_request();
                }

                out?;

                Ok(RequestExecuteStage::Processing(
                    self.request.operation.clone(),
                ))
            }

            SystemRestoreTarget::RestoreUpgrader => {
                self.system_service
                    .restore_upgrader(self.operation.input.snapshot_id.clone())
                    .await
                    .map_err(|err| RequestExecuteError::Failed {
                        reason: err
                            .details
                            .as_ref()
                            .and_then(|details| details.get("reason").cloned())
                            .unwrap_or(err.to_string()),
                    })?;

                Ok(RequestExecuteStage::Completed(
                    self.request.operation.clone(),
                ))
            }
        }
    }
}
