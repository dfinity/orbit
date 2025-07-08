use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{EditPermissionOperation, Request, RequestOperation},
    services::permission::PermissionService,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;
use std::sync::Arc;

pub struct EditPermissionRequestCreate {}

#[async_trait]
impl Create<station_api::EditPermissionOperationInput> for EditPermissionRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::EditPermissionOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::EditPermission(EditPermissionOperation {
                input: operation_input.into(),
            }),
            "Edit permission".to_string(),
        );

        Ok(request)
    }
}

pub struct EditPermissionRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o EditPermissionOperation,
    policy_service: Arc<PermissionService>,
}

impl<'p, 'o> EditPermissionRequestExecute<'p, 'o> {
    pub fn new(
        request: &'p Request,
        operation: &'o EditPermissionOperation,
        policy_service: Arc<PermissionService>,
    ) -> Self {
        Self {
            request,
            operation,
            policy_service,
        }
    }
}

#[async_trait]
impl Execute for EditPermissionRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        self.policy_service
            .edit_permission(self.operation.input.to_owned())
            .map_err(|e| RequestExecuteError::Failed {
                reason: format!("Failed to update permission: {}", e),
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
        models::permission::permission_test_utils::mock_permission,
        repositories::{permission::PERMISSION_REPOSITORY, REQUEST_REPOSITORY},
        services::permission::PERMISSION_SERVICE,
    };
    use orbit_essentials::{model::ModelKey, repository::Repository};

    #[tokio::test]
    async fn test_create_request() {
        let request_id = [0u8; 16];
        let requested_by_user = [1u8; 16];
        let operation_input = edit_permission_test_utils::mock_edit_permission_api_input();
        let mut request_input = edit_permission_test_utils::mock_request_api_input();
        request_input.operation =
            station_api::RequestOperationInput::EditPermission(operation_input.clone());

        let creator = Box::new(EditPermissionRequestCreate {});
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
        assert_eq!(request.title, "Edit permission".to_string());
    }

    #[tokio::test]
    async fn test_execute_request_completed() {
        let request_id = [0u8; 16];
        let requested_by_user = [1u8; 16];
        let operation_input = edit_permission_test_utils::mock_edit_permission_api_input();
        let mut request_input = edit_permission_test_utils::mock_request_api_input();
        request_input.operation =
            station_api::RequestOperationInput::EditPermission(operation_input.clone());

        let creator = Box::new(EditPermissionRequestCreate {});
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

        if let RequestOperation::EditPermission(operation) = &request.operation {
            let policy = mock_permission();
            PERMISSION_REPOSITORY.insert(policy.key(), policy.to_owned());

            let stage = EditPermissionRequestExecute::new(
                &request,
                operation,
                Arc::clone(&PERMISSION_SERVICE),
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
                "Expected EditPermission operation, got {:?}",
                request.operation
            );
        }
    }
}

#[cfg(test)]
pub mod edit_permission_test_utils {
    use uuid::Uuid;

    pub fn mock_edit_permission_api_input() -> station_api::EditPermissionOperationInput {
        station_api::EditPermissionOperationInput {
            resource: station_api::ResourceDTO::Permission(
                station_api::PermissionResourceActionDTO::Read,
            ),
            auth_scope: None,
            user_groups: None,
            users: Some(vec![Uuid::from_bytes([1u8; 16]).hyphenated().to_string()]),
        }
    }

    pub fn mock_request_api_input() -> station_api::CreateRequestInput {
        station_api::CreateRequestInput {
            operation: station_api::RequestOperationInput::EditPermission(
                mock_edit_permission_api_input(),
            ),
            title: None,
            summary: None,
            execution_plan: None,
            expiration_dt: None,
            tags: None,
        }
    }
}
