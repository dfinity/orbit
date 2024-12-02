use super::{Create, Execute, RequestExecuteStage};
use crate::models::{MonitorExternalCanisterOperation, MonitorExternalCanisterOperationKind};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{Request, RequestOperation},
    services::ExternalCanisterService,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;
use std::sync::Arc;

pub struct MonitorExternalCanisterRequestCreate;

#[async_trait]
impl Create<station_api::MonitorExternalCanisterOperationInput>
    for MonitorExternalCanisterRequestCreate
{
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::MonitorExternalCanisterOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::MonitorExternalCanister(operation_input.into()),
            "Monitor canister".to_string(),
        );

        Ok(request)
    }
}

pub struct MonitorExternalCanisterRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o MonitorExternalCanisterOperation,
    external_canister_service: Arc<ExternalCanisterService>,
}

impl<'p, 'o> MonitorExternalCanisterRequestExecute<'p, 'o> {
    pub fn new(
        request: &'p Request,
        operation: &'o MonitorExternalCanisterOperation,
        external_canister_service: Arc<ExternalCanisterService>,
    ) -> Self {
        Self {
            request,
            operation,
            external_canister_service,
        }
    }
}

#[async_trait]
impl Execute for MonitorExternalCanisterRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        match &self.operation.kind {
            MonitorExternalCanisterOperationKind::Start(input) => {
                self.external_canister_service
                    .canister_monitor_start(
                        self.operation.canister_id,
                        input.funding_strategy.clone(),
                        input.cycle_obtain_strategy,
                    )
                    .map_err(|e| RequestExecuteError::Failed {
                        reason: format!("Failed to monitor canister: {}", e),
                    })?;
            }
            MonitorExternalCanisterOperationKind::Stop => {
                self.external_canister_service
                    .canister_monitor_stop(self.operation.canister_id)
                    .map_err(|e| RequestExecuteError::Failed {
                        reason: format!("Failed to stop monitoring canister: {}", e),
                    })?;
            }
        }

        Ok(RequestExecuteStage::Completed(
            self.request.operation.clone(),
        ))
    }
}
