use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{EditUserOperation, Request, RequestExecutionPlan, RequestOperation},
    services::USER_SERVICE,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

pub struct EditUserRequestCreate {}

impl Create<station_api::EditUserOperationInput> for EditUserRequestCreate {
    fn create(
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::EditUserOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::new(
            request_id,
            requested_by_user,
            Request::default_expiration_dt_ns(),
            RequestOperation::EditUser(EditUserOperation {
                input: operation_input.into(),
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(RequestExecutionPlan::Immediate),
            input.title.unwrap_or_else(|| "User edit".to_string()),
            input.summary,
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
