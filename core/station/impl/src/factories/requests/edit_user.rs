use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{EditUserOperation, Request, RequestOperation},
    services::USER_SERVICE,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

pub struct EditUserRequestCreate {}

#[async_trait]
impl Create<station_api::EditUserOperationInput> for EditUserRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::EditUserOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::EditUser(EditUserOperation {
                input: operation_input.into(),
            }),
            "Edit user".to_string(),
        );

        Ok(request)
    }
}

pub struct EditUserRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o EditUserOperation,
}

impl<'p, 'o> EditUserRequestExecute<'p, 'o> {
    pub fn new(request: &'p Request, operation: &'o EditUserOperation) -> Self {
        Self { request, operation }
    }
}

#[async_trait]
impl Execute for EditUserRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        USER_SERVICE
            .edit_user(self.operation.input.clone())
            .await
            .map_err(|e| RequestExecuteError::Failed {
                reason: format!("Failed to edit user: {}", e),
            })?;

        Ok(RequestExecuteStage::Completed(
            self.request.operation.clone(),
        ))
    }
}
