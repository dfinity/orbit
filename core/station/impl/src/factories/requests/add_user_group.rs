use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{AddUserGroupOperation, Request, RequestExecutionPlan, RequestOperation},
    services::USER_GROUP_SERVICE,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

pub struct AddUserGroupRequestCreate {}

impl Create<station_api::AddUserGroupOperationInput> for AddUserGroupRequestCreate {
    fn create(
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::AddUserGroupOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::new(
            request_id,
            requested_by_user,
            Request::default_expiration_dt_ns(),
            RequestOperation::AddUserGroup(operation_input.into()),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(RequestExecutionPlan::Immediate),
            input
                .title
                .unwrap_or_else(|| "User group creation".to_string()),
            input.summary,
        );

        Ok(request)
    }
}

pub struct AddUserGroupRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o AddUserGroupOperation,
}

impl<'p, 'o> AddUserGroupRequestExecute<'p, 'o> {
    pub fn new(request: &'p Request, operation: &'o AddUserGroupOperation) -> Self {
        Self { request, operation }
    }
}

#[async_trait]
impl Execute for AddUserGroupRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        let user_group = USER_GROUP_SERVICE
            .create(self.operation.input.clone())
            .await
            .map_err(|e| RequestExecuteError::Failed {
                reason: format!("Failed to create user group: {}", e),
            })?;

        let mut operation = self.request.operation.clone();

        if let RequestOperation::AddUserGroup(ref mut op) = operation {
            op.user_group_id = Some(user_group.id);
        }

        Ok(RequestExecuteStage::Completed(operation))
    }
}
