use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{RemoveUserGroupOperation, Request, RequestExecutionPlan, RequestOperation},
    services::USER_GROUP_SERVICE,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

pub struct RemoveUserGroupRequestCreate {}

#[async_trait]
impl Create<station_api::RemoveUserGroupOperationInput> for RemoveUserGroupRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::RemoveUserGroupOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::new(
            request_id,
            requested_by_user,
            Request::default_expiration_dt_ns(),
            RequestOperation::RemoveUserGroup(operation_input.into()),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(RequestExecutionPlan::Immediate),
            input
                .title
                .unwrap_or_else(|| "User group removal".to_string()),
            input.summary,
        );

        Ok(request)
    }
}

pub struct RemoveUserGroupRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o RemoveUserGroupOperation,
}

impl<'p, 'o> RemoveUserGroupRequestExecute<'p, 'o> {
    pub fn new(request: &'p Request, operation: &'o RemoveUserGroupOperation) -> Self {
        Self { request, operation }
    }
}

#[async_trait]
impl Execute for RemoveUserGroupRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        USER_GROUP_SERVICE
            .remove(&self.operation.input.user_group_id)
            .await
            .map_err(|e| RequestExecuteError::Failed {
                reason: format!("Failed to remove user group: {}", e),
            })?;

        Ok(RequestExecuteStage::Completed(
            self.request.operation.clone(),
        ))
    }
}
