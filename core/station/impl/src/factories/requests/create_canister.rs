use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{CreateManagedCanisterOperation, Request, RequestExecutionPlan, RequestOperation},
    services::ExternalCanisterService,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;
use station_api::{CreateManagedCanisterOperationInput, CreateRequestInput};
use std::sync::Arc;

pub struct CreateManagedCanisterRequestCreate;

#[async_trait]
impl Create<CreateManagedCanisterOperationInput> for CreateManagedCanisterRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: CreateRequestInput,
        _operation_input: CreateManagedCanisterOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::new(
            request_id,
            requested_by_user,
            Request::default_expiration_dt_ns(),
            RequestOperation::CreateManagedCanister(CreateManagedCanisterOperation {
                canister_id: None,
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(RequestExecutionPlan::Immediate),
            input
                .title
                .unwrap_or_else(|| "CreateManagedCanister".to_string()),
            input.summary,
        );

        Ok(request)
    }
}

pub struct CreateManagedCanisterRequestExecute<'p, 'o> {
    _request: &'p Request,
    operation: &'o CreateManagedCanisterOperation,
    create_canister_service: Arc<ExternalCanisterService>,
}

impl<'p, 'o> CreateManagedCanisterRequestExecute<'p, 'o> {
    pub fn new(
        request: &'p Request,
        operation: &'o CreateManagedCanisterOperation,
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
impl Execute for CreateManagedCanisterRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        let canister_id = self
            .create_canister_service
            .create_canister()
            .await
            .map_err(|err| RequestExecuteError::Failed {
                reason: format!("failed to create managed canister: {}", err),
            })?;
        let mut create_operation = self.operation.clone();
        create_operation.canister_id = Some(canister_id);

        Ok(RequestExecuteStage::Completed(
            RequestOperation::CreateManagedCanister(create_operation),
        ))
    }
}
