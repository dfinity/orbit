use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{RemoveNamedRuleOperation, Request, RequestOperation},
    services::NamedRuleService,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

pub struct RemoveNamedRuleRequestCreate {}

#[async_trait]
impl Create<station_api::RemoveNamedRuleOperationInput> for RemoveNamedRuleRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::RemoveNamedRuleOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::RemoveNamedRule(RemoveNamedRuleOperation {
                input: operation_input.into(),
            }),
            "Remove named rule".to_string(),
        );

        Ok(request)
    }
}

pub struct RemoveNamedRuleRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o RemoveNamedRuleOperation,
    named_rule_service: NamedRuleService,
}

impl<'p, 'o> RemoveNamedRuleRequestExecute<'p, 'o> {
    pub fn new(request: &'p Request, operation: &'o RemoveNamedRuleOperation) -> Self {
        Self {
            request,
            operation,
            named_rule_service: NamedRuleService::default(),
        }
    }
}

#[async_trait]
impl Execute for RemoveNamedRuleRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        self.named_rule_service
            .remove(self.operation.input.clone())
            .map_err(|e| RequestExecuteError::Failed {
                reason: format!("Failed to remove named rule: {}", e),
            })?;

        Ok(RequestExecuteStage::Completed(
            self.request.operation.clone(),
        ))
    }
}
