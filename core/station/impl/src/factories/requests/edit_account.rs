use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{EditAccountOperation, Request, RequestExecutionPlan, RequestOperation},
    services::ACCOUNT_SERVICE,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

pub struct EditAccountRequestCreate {}

impl Create<station_api::EditAccountOperationInput> for EditAccountRequestCreate {
    fn create(
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::EditAccountOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::new(
            request_id,
            requested_by_user,
            Request::default_expiration_dt_ns(),
            RequestOperation::EditAccount(EditAccountOperation {
                input: operation_input.into(),
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(RequestExecutionPlan::Immediate),
            input.title.unwrap_or_else(|| "Account edit".to_string()),
            input.summary,
        );

        Ok(request)
    }
}

pub struct EditAccountRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o EditAccountOperation,
}

impl<'p, 'o> EditAccountRequestExecute<'p, 'o> {
    pub fn new(request: &'p Request, operation: &'o EditAccountOperation) -> Self {
        Self { request, operation }
    }
}

#[async_trait]
impl Execute for EditAccountRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        ACCOUNT_SERVICE
            .edit_account(self.operation.input.to_owned())
            .await
            .map_err(|e| RequestExecuteError::Failed {
                reason: format!("Failed to update account: {}", e),
            })?;

        Ok(RequestExecuteStage::Completed(
            self.request.operation.clone(),
        ))
    }
}
