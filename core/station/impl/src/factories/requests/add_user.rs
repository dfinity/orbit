use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{AddUserOperation, Request, RequestOperation},
    services::USER_SERVICE,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

pub struct AddUserRequestCreate {}

#[async_trait]
impl Create<station_api::AddUserOperationInput> for AddUserRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::AddUserOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::AddUser(AddUserOperation {
                user_id: None,
                input: operation_input.into(),
            }),
            "Add user".to_string(),
        );

        Ok(request)
    }
}

pub struct AddUserRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o AddUserOperation,
}

impl<'p, 'o> AddUserRequestExecute<'p, 'o> {
    pub fn new(request: &'p Request, operation: &'o AddUserOperation) -> Self {
        Self { request, operation }
    }
}

#[async_trait]
impl Execute for AddUserRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        let user = USER_SERVICE
            .add_user(self.operation.input.clone())
            .map_err(|e| RequestExecuteError::Failed {
                reason: format!("Failed to create user: {}", e),
            })?;

        let mut operation = self.request.operation.clone();

        if let RequestOperation::AddUser(ref mut operation) = operation {
            operation.user_id = Some(user.id);
        }

        Ok(RequestExecuteStage::Completed(operation))
    }
}
