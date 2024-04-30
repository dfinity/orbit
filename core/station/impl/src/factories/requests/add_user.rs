use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{AddUserOperation, Request, RequestExecutionPlan, RequestOperation},
    services::USER_SERVICE,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

pub struct AddUserRequestCreate {}

impl Create<station_api::AddUserOperationInput> for AddUserRequestCreate {
    fn create(
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::AddUserOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::new(
            request_id,
            requested_by_user,
            Request::default_expiration_dt_ns(),
            RequestOperation::AddUser(AddUserOperation {
                user_id: None,
                input: operation_input.into(),
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(RequestExecutionPlan::Immediate),
            input.title.unwrap_or_else(|| "User creation".to_string()),
            input.summary,
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
