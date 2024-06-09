use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{AddAccountOperation, Request, RequestExecutionPlan, RequestOperation},
    services::AccountService,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

pub struct AddAccountRequestCreate {}

#[async_trait]
impl Create<station_api::AddAccountOperationInput> for AddAccountRequestCreate {
    async fn create(
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::AddAccountOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::new(
            request_id,
            requested_by_user,
            Request::default_expiration_dt_ns(),
            RequestOperation::AddAccount(AddAccountOperation {
                account_id: None,
                input: operation_input.into(),
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(RequestExecutionPlan::Immediate),
            input
                .title
                .unwrap_or_else(|| "Account creation".to_string()),
            input.summary,
        );

        Ok(request)
    }
}

pub struct AddAccountRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o AddAccountOperation,
    account_service: AccountService,
}

impl<'p, 'o> AddAccountRequestExecute<'p, 'o> {
    pub fn new(request: &'p Request, operation: &'o AddAccountOperation) -> Self {
        Self {
            request,
            operation,
            account_service: AccountService::default(),
        }
    }
}

#[async_trait]
impl Execute for AddAccountRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        let account = self
            .account_service
            .create_account(self.operation.input.to_owned())
            .await
            .map_err(|e| RequestExecuteError::Failed {
                reason: format!("Failed to create account: {}", e),
            })?;

        let mut operation = self.request.operation.clone();

        if let RequestOperation::AddAccount(ref mut operation) = operation {
            operation.account_id = Some(account.id);
        }

        Ok(RequestExecuteStage::Completed(operation))
    }
}
