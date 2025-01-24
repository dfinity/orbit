use super::{Create, Execute, RequestExecuteStage};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{EditNamedRuleOperation, Request, RequestOperation},
    services::NamedRuleService,
};

pub struct EditNamedRuleRequestCreate {}

#[async_trait]
impl Create<station_api::EditNamedRuleOperationInput> for EditNamedRuleRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::EditNamedRuleOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::EditNamedRule(EditNamedRuleOperation {
                input: operation_input.into(),
            }),
            "Edit named rule".to_string(),
        );

        Ok(request)
    }
}

pub struct EditNamedRuleRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o EditNamedRuleOperation,
    named_rule_service: NamedRuleService,
}

impl<'p, 'o> EditNamedRuleRequestExecute<'p, 'o> {
    pub fn new(request: &'p Request, operation: &'o EditNamedRuleOperation) -> Self {
        Self {
            request,
            operation,
            named_rule_service: NamedRuleService::default(),
        }
    }
}

#[async_trait]
impl Execute for EditNamedRuleRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        self.named_rule_service
            .edit(self.operation.input.clone())
            .map_err(|e| RequestExecuteError::Failed {
                reason: format!("Failed to edit named rule: {}", e),
            })?;

        Ok(RequestExecuteStage::Completed(
            self.request.operation.clone(),
        ))
    }
}
