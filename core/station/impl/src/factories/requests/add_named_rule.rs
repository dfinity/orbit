use super::{Create, Execute, RequestExecuteStage};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{AddNamedRuleOperation, Request, RequestOperation},
    services::NamedRuleService,
};

pub struct AddNamedRuleRequestCreate {}

#[async_trait]
impl Create<station_api::AddNamedRuleOperationInput> for AddNamedRuleRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::AddNamedRuleOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::AddNamedRule(AddNamedRuleOperation {
                named_rule_id: None,
                input: operation_input.into(),
            }),
            "Add named rule".to_string(),
        );

        Ok(request)
    }
}

pub struct AddNamedRuleRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o AddNamedRuleOperation,
    named_rule_service: NamedRuleService,
}

impl<'p, 'o> AddNamedRuleRequestExecute<'p, 'o> {
    pub fn new(request: &'p Request, operation: &'o AddNamedRuleOperation) -> Self {
        Self {
            request,
            operation,
            named_rule_service: NamedRuleService::default(),
        }
    }
}

#[async_trait]
impl Execute for AddNamedRuleRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        let named_rule = self
            .named_rule_service
            .create(self.operation.input.clone())
            .map_err(|e| RequestExecuteError::Failed {
                reason: format!("Failed to create named rule: {}", e),
            })?;

        let mut operation = self.request.operation.clone();

        if let RequestOperation::AddNamedRule(ref mut operation) = operation {
            operation.named_rule_id = Some(named_rule.id);
        }

        Ok(RequestExecuteStage::Completed(operation))
    }
}
