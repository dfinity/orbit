use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{
        EditRequestPolicyOperation, EditRequestPolicyOperationInput, Request, RequestOperation,
    },
    services::{RequestPolicyService, REQUEST_POLICY_SERVICE},
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;
use std::sync::Arc;
use uuid::Uuid;

pub struct EditRequestPolicyRequestCreate {}

#[async_trait]
impl Create<station_api::EditRequestPolicyOperationInput> for EditRequestPolicyRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::EditRequestPolicyOperationInput,
    ) -> Result<Request, RequestError> {
        let operation_input = EditRequestPolicyOperationInput::from(operation_input);
        REQUEST_POLICY_SERVICE
            .get_request_policy(&operation_input.policy_id)
            .map_err(|_| RequestError::ValidationError {
                info: format!(
                    "Request policy with id {} does not exist",
                    Uuid::from_bytes(operation_input.policy_id).hyphenated()
                ),
            })?;

        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::EditRequestPolicy(EditRequestPolicyOperation {
                input: operation_input,
            }),
            "Edit approval policy".to_string(),
        );

        Ok(request)
    }
}

pub struct EditRequestPolicyRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o EditRequestPolicyOperation,
    policy_service: Arc<RequestPolicyService>,
}

impl<'p, 'o> EditRequestPolicyRequestExecute<'p, 'o> {
    pub fn new(
        request: &'p Request,
        operation: &'o EditRequestPolicyOperation,
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
impl Execute for EditRequestPolicyRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        self.policy_service
            .edit_request_policy(self.operation.input.to_owned())
            .map_err(|e| RequestExecuteError::Failed {
                reason: format!("Failed to update request policy: {}", e),
            })?;

        Ok(RequestExecuteStage::Completed(
            self.request.operation.to_owned(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::request_policy_test_utils::mock_request_policy,
        repositories::{request_policy::REQUEST_POLICY_REPOSITORY, REQUEST_REPOSITORY},
    };
    use orbit_essentials::repository::Repository;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_create_request() {
        let request_id = [0u8; 16];
        let requested_by_user = [1u8; 16];
        let operation_input = edit_request_policy_test_utils::mock_edit_request_policy_api_input();
        let mut request_input = edit_request_policy_test_utils::mock_request_api_input();
        request_input.operation =
            station_api::RequestOperationInput::EditRequestPolicy(operation_input.clone());

        let mut policy = mock_request_policy();
        policy.id = *Uuid::from_str(&operation_input.policy_id)
            .unwrap()
            .as_bytes();
        REQUEST_POLICY_REPOSITORY.insert(policy.id, policy.to_owned());

        let creator = Box::new(EditRequestPolicyRequestCreate {});
        let request = creator
            .create(
                request_id,
                requested_by_user,
                request_input,
                operation_input,
            )
            .await
            .unwrap();

        assert_eq!(request.id, request_id);
        assert_eq!(request.requested_by, requested_by_user);
        assert_eq!(request.title, "Edit approval policy".to_string());
    }

    #[tokio::test]
    async fn test_execute_request_completed() {
        let request_id = [0u8; 16];
        let requested_by_user = [1u8; 16];
        let operation_input = edit_request_policy_test_utils::mock_edit_request_policy_api_input();
        let mut request_input = edit_request_policy_test_utils::mock_request_api_input();
        request_input.operation =
            station_api::RequestOperationInput::EditRequestPolicy(operation_input.clone());

        let mut policy = mock_request_policy();
        policy.id = *Uuid::from_str(&operation_input.policy_id)
            .unwrap()
            .as_bytes();
        REQUEST_POLICY_REPOSITORY.insert(policy.id, policy.to_owned());

        let creator = Box::new(EditRequestPolicyRequestCreate {});
        let request = creator
            .create(
                request_id,
                requested_by_user,
                request_input,
                operation_input,
            )
            .await
            .unwrap();

        REQUEST_REPOSITORY.insert(request.to_key(), request.to_owned());

        if let RequestOperation::EditRequestPolicy(operation) = &request.operation {
            let mut policy = mock_request_policy();
            policy.id = operation.input.policy_id;
            REQUEST_POLICY_REPOSITORY.insert(policy.id, policy.to_owned());

            let stage = EditRequestPolicyRequestExecute::new(
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
                "Expected EditRequestPolicy operation, got {:?}",
                request.operation
            );
        }
    }

    #[tokio::test]
    async fn test_execute_request_should_fail_non_existant_policy() {
        let request_id = [0u8; 16];
        let requested_by_user = [1u8; 16];
        let operation_input = edit_request_policy_test_utils::mock_edit_request_policy_api_input();
        let mut request_input = edit_request_policy_test_utils::mock_request_api_input();
        request_input.operation =
            station_api::RequestOperationInput::EditRequestPolicy(operation_input.clone());

        let mut policy = mock_request_policy();
        policy.id = *Uuid::from_str(&operation_input.policy_id)
            .unwrap()
            .as_bytes();
        REQUEST_POLICY_REPOSITORY.insert(policy.id, policy.to_owned());

        let creator = Box::new(EditRequestPolicyRequestCreate {});
        let request = creator
            .create(
                request_id,
                requested_by_user,
                request_input,
                operation_input,
            )
            .await
            .unwrap();

        REQUEST_POLICY_REPOSITORY.remove(&policy.id);

        REQUEST_REPOSITORY.insert(request.to_key(), request.to_owned());

        if let RequestOperation::EditRequestPolicy(operation) = &request.operation {
            let stage = EditRequestPolicyRequestExecute::new(
                &request,
                operation,
                Arc::clone(&REQUEST_POLICY_SERVICE),
            )
            .execute()
            .await;

            assert!(stage.is_err());
        } else {
            panic!(
                "Expected EditRequestPolicy operation, got {:?}",
                request.operation
            );
        }
    }
}

#[cfg(test)]
pub mod edit_request_policy_test_utils {
    use uuid::Uuid;

    pub fn mock_edit_request_policy_api_input() -> station_api::EditRequestPolicyOperationInput {
        station_api::EditRequestPolicyOperationInput {
            policy_id: Uuid::from_bytes([0u8; 16]).hyphenated().to_string(),
            rule: Some(station_api::RequestPolicyRuleDTO::AutoApproved),
            specifier: Some(station_api::RequestSpecifierDTO::EditRequestPolicy(
                station_api::ResourceIdsDTO::Any,
            )),
        }
    }

    pub fn mock_request_api_input() -> station_api::CreateRequestInput {
        station_api::CreateRequestInput {
            operation: station_api::RequestOperationInput::EditRequestPolicy(
                mock_edit_request_policy_api_input(),
            ),
            title: None,
            summary: None,
            execution_plan: None,
            expiration_dt: None,
            deduplication_key: None,
            tags: None,
        }
    }
}
