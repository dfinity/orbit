use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{AddUserGroupOperation, Request, RequestOperation},
    services::USER_GROUP_SERVICE,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

pub struct AddUserGroupRequestCreate {}

#[async_trait]
impl Create<station_api::AddUserGroupOperationInput> for AddUserGroupRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::AddUserGroupOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::AddUserGroup(operation_input.into()),
            "Create user group".to_string(),
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
