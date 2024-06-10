use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{CreateExternalCanisterOperation, Request, RequestExecutionPlan, RequestOperation},
    services::ExternalCanisterService,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;
use station_api::{CreateExternalCanisterOperationInput, CreateRequestInput};
use std::sync::Arc;

pub struct CreateExternalCanisterRequestCreate;

#[async_trait]
impl Create<CreateExternalCanisterOperationInput> for CreateExternalCanisterRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: CreateRequestInput,
        _operation_input: CreateExternalCanisterOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::new(
            request_id,
            requested_by_user,
            Request::default_expiration_dt_ns(),
            RequestOperation::CreateExternalCanister(CreateExternalCanisterOperation {
                canister_id: None,
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(RequestExecutionPlan::Immediate),
            input
                .title
                .unwrap_or_else(|| "CreateExternalCanister".to_string()),
            input.summary,
        );

        Ok(request)
    }
}

pub struct CreateExternalCanisterRequestExecute<'p, 'o> {
    _request: &'p Request,
    operation: &'o CreateExternalCanisterOperation,
    create_canister_service: Arc<ExternalCanisterService>,
}

impl<'p, 'o> CreateExternalCanisterRequestExecute<'p, 'o> {
    pub fn new(
        request: &'p Request,
        operation: &'o CreateExternalCanisterOperation,
        create_canister_service: Arc<ExternalCanisterService>,
    ) -> Self {
        Self {
            _request: request,
            operation,
            create_canister_service,
        }
    }
}

#[async_trait]
impl Execute for CreateExternalCanisterRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        let canister_id = self
            .create_canister_service
            .create_canister()
            .await
            .map_err(|err| RequestExecuteError::Failed {
                reason: format!("failed to create external canister: {}", err),
            })?;
        let mut create_operation = self.operation.clone();
        create_operation.canister_id = Some(canister_id);

        Ok(RequestExecuteStage::Completed(
            RequestOperation::CreateExternalCanister(create_operation),
        ))
    }
}
