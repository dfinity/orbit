use std::sync::Arc;

use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{AddRequestPolicyOperation, Request, RequestExecutionPlan, RequestOperation},
    services::RequestPolicyService,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

pub struct AddRequestPolicyRequestCreate {}

#[async_trait]
impl Create<station_api::AddRequestPolicyOperationInput> for AddRequestPolicyRequestCreate {
    async fn create(
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::AddRequestPolicyOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::new(
            request_id,
            requested_by_user,
            Request::default_expiration_dt_ns(),
            RequestOperation::AddRequestPolicy(AddRequestPolicyOperation {
                policy_id: None,
                input: operation_input.into(),
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(RequestExecutionPlan::Immediate),
            input
                .title
                .unwrap_or_else(|| "Request policy creation".to_string()),
            input.summary,
        );

        Ok(request)
    }
}

pub struct AddRequestPolicyRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o AddRequestPolicyOperation,
    policy_service: Arc<RequestPolicyService>,
}

impl<'p, 'o> AddRequestPolicyRequestExecute<'p, 'o> {
    pub fn new(
        request: &'p Request,
        operation: &'o AddRequestPolicyOperation,
        policy_service: Arc<RequestPolicyService>,
    ) -> Self {
        Self {
            request,
            operation,
            policy_service,
        }
    }
}

#[async_trait]
impl Execute for AddRequestPolicyRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        let policy = self
            .policy_service
            .add_request_policy(self.operation.input.to_owned())
            .await
            .map_err(|e| RequestExecuteError::Failed {
                reason: format!("Failed to create request policy: {}", e),
            })?;

        let mut operation = self.request.operation.clone();

        if let RequestOperation::AddRequestPolicy(ref mut operation) = operation {
            operation.policy_id = Some(policy.id);
        }

        Ok(RequestExecuteStage::Completed(operation))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{repositories::REQUEST_REPOSITORY, services::REQUEST_POLICY_SERVICE};
    use orbit_essentials::repository::Repository;

    #[tokio::test]
    async fn test_create_request() {
        let request_id = [0u8; 16];
        let requested_by_user = [1u8; 16];
        let operation_input = add_request_policy_test_utils::mock_add_request_policy_api_input();
        let mut request_input = add_request_policy_test_utils::mock_request_api_input();
        request_input.operation =
            station_api::RequestOperationInput::AddRequestPolicy(operation_input.clone());

        let request = AddRequestPolicyRequestCreate::create(
            request_id,
            requested_by_user,
            request_input,
            operation_input,
        )
        .await
        .unwrap();

        assert_eq!(request.id, request_id);
        assert_eq!(request.requested_by, requested_by_user);
        assert_eq!(request.title, "Request policy creation".to_string());
    }

    #[tokio::test]
    async fn test_execute_request_completed() {
        let request_id = [0u8; 16];
        let requested_by_user = [1u8; 16];
        let operation_input = add_request_policy_test_utils::mock_add_request_policy_api_input();
        let mut request_input = add_request_policy_test_utils::mock_request_api_input();
        request_input.operation =
            station_api::RequestOperationInput::AddRequestPolicy(operation_input.clone());

        let request = AddRequestPolicyRequestCreate::create(
            request_id,
            requested_by_user,
            request_input,
            operation_input,
        )
        .await
        .unwrap();

        REQUEST_REPOSITORY.insert(request.to_key(), request.to_owned());

        if let RequestOperation::AddRequestPolicy(operation) = &request.operation {
            let stage = AddRequestPolicyRequestExecute::new(
                &request,
                operation,
                Arc::clone(&REQUEST_POLICY_SERVICE),
            )
            .execute()
            .await
            .unwrap();

            match stage {
                RequestExecuteStage::Completed(_) => (),
                _ => panic!("Expected RequestExecuteStage::Completed, got {:?}", stage),
            }
        } else {
            panic!(
                "Expected AddRequestPolicy operation, got {:?}",
                request.operation
            );
        }
    }
}

#[cfg(test)]
pub mod add_request_policy_test_utils {
    pub fn mock_add_request_policy_api_input() -> station_api::AddRequestPolicyOperationInput {
        station_api::AddRequestPolicyOperationInput {
            rule: station_api::RequestPolicyRuleDTO::AutoApproved,
            specifier: station_api::RequestSpecifierDTO::AddRequestPolicy,
        }
    }

    pub fn mock_request_api_input() -> station_api::CreateRequestInput {
        station_api::CreateRequestInput {
            operation: station_api::RequestOperationInput::AddRequestPolicy(
                mock_add_request_policy_api_input(),
            ),
            title: None,
            summary: None,
            execution_plan: None,
        }
    }
}
