use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{EditUserGroupOperation, Request, RequestExecutionPlan, RequestOperation},
    services::USER_GROUP_SERVICE,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

pub struct EditUserGroupRequestCreate {}

impl Create<station_api::EditUserGroupOperationInput> for EditUserGroupRequestCreate {
    fn create(
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::EditUserGroupOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::new(
            request_id,
            requested_by_user,
            Request::default_expiration_dt_ns(),
            RequestOperation::EditUserGroup(operation_input.into()),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(RequestExecutionPlan::Immediate),
            input.title.unwrap_or_else(|| "User group edit".to_string()),
            input.summary,
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
