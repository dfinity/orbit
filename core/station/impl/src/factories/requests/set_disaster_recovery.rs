use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{
        Request, RequestOperation, SetDisasterRecoveryOperation, SetDisasterRecoveryOperationInput,
    },
    services::SystemService,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;
use station_api::CreateRequestInput;

pub struct SetDisasterRecoveryRequestCreate;

#[async_trait]
impl Create<station_api::SetDisasterRecoveryOperationInput> for SetDisasterRecoveryRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: CreateRequestInput,
        operation_input: station_api::SetDisasterRecoveryOperationInput,
    ) -> Result<Request, RequestError> {
        let operation_input: SetDisasterRecoveryOperationInput = operation_input.into();

        SystemService::assert_disaster_recovery_committee(operation_input.committee.as_ref())
            .map_err(|e| RequestError::ValidationError {
                info: e.to_string(),
            })?;

        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::SetDisasterRecovery(SetDisasterRecoveryOperation {
                input: operation_input,
            }),
            "Configure disaster recovery".to_string(),
        );

        Ok(request)
    }
}

#[allow(dead_code)]
pub struct SetDisasterRecoveryRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o SetDisasterRecoveryOperation,
}

impl<'p, 'o> SetDisasterRecoveryRequestExecute<'p, 'o> {
    pub fn new(request: &'p Request, operation: &'o SetDisasterRecoveryOperation) -> Self {
        Self { request, operation }
    }
}

#[async_trait]
impl Execute for SetDisasterRecoveryRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        SystemService::set_disaster_recovery_committee(self.operation.input.committee.clone())
            .map_err(|e| RequestExecuteError::Failed {
                reason: format!("Failed to set the disaster recovery committee: {e}"),
            })?;

        Ok(RequestExecuteStage::Completed(
            self.request.operation.clone(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::{read_system_info, write_system_info},
        factories::requests::requests_test_utils::mock_request_api_input,
        models::{
            user_group_test_utils::add_group, user_test_utils::mock_user, SystemInfo, UserStatus,
        },
        repositories::USER_REPOSITORY,
    };
    use orbit_essentials::repository::Repository;
    use station_api::{DisasterRecoveryCommitteeDTO, RequestOperationInput};
    use uuid::Uuid;

    fn add_committee_group(members: &[UserStatus]) -> UUID {
        write_system_info(SystemInfo::default());
        let group = add_group("committee");
        for status in members {
            let mut user = mock_user();
            user.groups = vec![group.id];
            user.status = status.clone();
            USER_REPOSITORY.insert(user.to_key(), user);
        }

        group.id
    }

    fn operation_input(group_id: Option<UUID>) -> station_api::SetDisasterRecoveryOperationInput {
        station_api::SetDisasterRecoveryOperationInput {
            committee: group_id.map(|group_id| DisasterRecoveryCommitteeDTO {
                user_group_id: Uuid::from_bytes(group_id).hyphenated().to_string(),
                quorum: 1,
            }),
        }
    }

    async fn create_request(group_id: Option<UUID>) -> Result<Request, RequestError> {
        let operation_input = operation_input(group_id);
        SetDisasterRecoveryRequestCreate
            .create(
                [0; 16],
                [1; 16],
                mock_request_api_input(RequestOperationInput::SetDisasterRecovery(
                    operation_input.clone(),
                )),
                operation_input,
            )
            .await
    }

    async fn execute_request(
        request: &Request,
    ) -> Result<RequestExecuteStage, RequestExecuteError> {
        let RequestOperation::SetDisasterRecovery(operation) = &request.operation else {
            panic!("Expected a SetDisasterRecovery operation");
        };

        SetDisasterRecoveryRequestExecute::new(request, operation)
            .execute()
            .await
    }

    #[tokio::test]
    async fn create_refuses_missing_committee() {
        let error = create_request(None).await.unwrap_err();

        let RequestError::ValidationError { info } = error else {
            panic!("Expected a validation error, got {error:?}");
        };
        assert!(info.starts_with("MISSING_COMMITTEE"), "{info}");
    }

    #[tokio::test]
    async fn create_refuses_committee_without_active_members() {
        let group_id = add_committee_group(&[UserStatus::Inactive]);

        let error = create_request(Some(group_id)).await.unwrap_err();

        let RequestError::ValidationError { info } = error else {
            panic!("Expected a validation error, got {error:?}");
        };
        assert!(
            info.starts_with("COMMITTEE_WITHOUT_ACTIVE_MEMBERS"),
            "{info}"
        );
    }

    #[tokio::test]
    async fn execute_refuses_committee_whose_members_left_after_creation() {
        let group_id = add_committee_group(&[UserStatus::Active]);
        let request = create_request(Some(group_id)).await.unwrap();
        for mut user in USER_REPOSITORY.list() {
            user.status = UserStatus::Inactive;
            USER_REPOSITORY.insert(user.to_key(), user);
        }

        let error = execute_request(&request).await.unwrap_err();

        let RequestExecuteError::Failed { reason } = error else {
            panic!("Expected a failed execution, got {error:?}");
        };
        assert!(
            reason.contains("COMMITTEE_WITHOUT_ACTIVE_MEMBERS"),
            "{reason}"
        );
        assert!(read_system_info()
            .get_disaster_recovery_committee()
            .is_none());
    }

    #[tokio::test]
    async fn execute_refuses_missing_committee() {
        let mut request = create_request(Some(add_committee_group(&[UserStatus::Active])))
            .await
            .unwrap();
        request.operation = RequestOperation::SetDisasterRecovery(SetDisasterRecoveryOperation {
            input: SetDisasterRecoveryOperationInput { committee: None },
        });

        let error = execute_request(&request).await.unwrap_err();

        let RequestExecuteError::Failed { reason } = error else {
            panic!("Expected a failed execution, got {error:?}");
        };
        assert!(reason.contains("MISSING_COMMITTEE"), "{reason}");
    }

    #[tokio::test]
    async fn execute_sets_committee_with_active_members() {
        let group_id = add_committee_group(&[UserStatus::Active, UserStatus::Inactive]);
        let request = create_request(Some(group_id)).await.unwrap();

        let stage = execute_request(&request).await.unwrap();

        assert!(matches!(stage, RequestExecuteStage::Completed(_)));
        assert_eq!(
            read_system_info()
                .get_disaster_recovery_committee()
                .map(|committee| committee.user_group_id),
            Some(group_id)
        );
    }
}
