use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{EditUserGroupOperation, Request, RequestOperation},
    services::USER_GROUP_SERVICE,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

pub struct EditUserGroupRequestCreate {}

#[async_trait]
impl Create<station_api::EditUserGroupOperationInput> for EditUserGroupRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::EditUserGroupOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::EditUserGroup(operation_input.into()),
            "Edit user group".to_string(),
        );

        Ok(request)
    }
}

pub struct EditUserGroupRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o EditUserGroupOperation,
}

impl<'p, 'o> EditUserGroupRequestExecute<'p, 'o> {
    pub fn new(request: &'p Request, operation: &'o EditUserGroupOperation) -> Self {
        Self { request, operation }
    }
}

#[async_trait]
impl Execute for EditUserGroupRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        USER_GROUP_SERVICE
            .edit(self.operation.input.clone())
            .await
            .map_err(|e| RequestExecuteError::Failed {
                reason: format!("Failed to edit user group: {}", e),
            })?;

        Ok(RequestExecuteStage::Completed(
            self.request.operation.clone(),
        ))
    }
}
