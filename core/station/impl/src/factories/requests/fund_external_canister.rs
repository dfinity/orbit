use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{
        FundExternalCanisterOperation, FundExternalCanisterOperationKind, Request,
        RequestExecutionPlan, RequestOperation,
    },
    services::ExternalCanisterService,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;
use std::sync::Arc;

pub struct FundExternalCanisterRequestCreate;

#[async_trait]
impl Create<station_api::FundExternalCanisterOperationInput> for FundExternalCanisterRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::FundExternalCanisterOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::new(
            request_id,
            requested_by_user,
            Request::default_expiration_dt_ns(),
            RequestOperation::FundExternalCanister(operation_input.into()),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(RequestExecutionPlan::Immediate),
            input.title.unwrap_or_else(|| "Fund canister".to_string()),
            input.summary,
        );

        Ok(request)
    }
}

pub struct FundExternalCanisterRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o FundExternalCanisterOperation,
    external_canister_service: Arc<ExternalCanisterService>,
}

impl<'p, 'o> FundExternalCanisterRequestExecute<'p, 'o> {
    pub fn new(
        request: &'p Request,
        operation: &'o FundExternalCanisterOperation,
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
impl Execute for FundExternalCanisterRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        match &self.operation.kind {
            FundExternalCanisterOperationKind::Send(input) => {
                self.external_canister_service
                    .top_up_canister(self.operation.canister_id, input.cycles as u128)
                    .await
                    .map_err(|e| RequestExecuteError::Failed {
                        reason: format!("Failed to fund canister: {}", e),
                    })?;
            }
        }

        Ok(RequestExecuteStage::Completed(
            self.request.operation.clone(),
        ))
    }
}
