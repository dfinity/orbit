use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{CreateExternalCanisterOperation, Request, RequestOperation},
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
        operation_input: CreateExternalCanisterOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::CreateExternalCanister(CreateExternalCanisterOperation {
                canister_id: None,
                input: operation_input.into(),
            }),
            "Create canister".to_string(),
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
        let external_canister = self
            .create_canister_service
            .add_external_canister(self.operation.input.clone())
            .await
            .map_err(|err| RequestExecuteError::Failed {
                reason: format!("failed to add external canister: {}", err),
            })?;

        let mut create_operation = self.operation.clone();
        create_operation.canister_id = Some(external_canister.canister_id);

        Ok(RequestExecuteStage::Completed(
            RequestOperation::CreateExternalCanister(create_operation),
        ))
    }
}
