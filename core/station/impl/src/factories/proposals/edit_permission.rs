use super::{Create, Execute, ProposalExecuteStage};
use crate::{
    errors::{ProposalError, ProposalExecuteError},
    models::{EditPermissionOperation, Proposal, ProposalExecutionPlan, ProposalOperation},
    services::permission::PermissionService,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;
use std::sync::Arc;

pub struct EditPermissionProposalCreate {}

impl Create<station_api::EditPermissionOperationInput> for EditPermissionProposalCreate {
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: station_api::CreateProposalInput,
        operation_input: station_api::EditPermissionOperationInput,
    ) -> Result<Proposal, ProposalError> {
        let proposal = Proposal::new(
            proposal_id,
            proposed_by_user,
            Proposal::default_expiration_dt_ns(),
            ProposalOperation::EditPermission(EditPermissionOperation {
                input: operation_input.into(),
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(ProposalExecutionPlan::Immediate),
            input
                .title
                .unwrap_or_else(|| "Permission update".to_string()),
            input.summary,
        );

        Ok(proposal)
    }
}

pub struct EditPermissionProposalExecute<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o EditPermissionOperation,
    policy_service: Arc<PermissionService>,
}

impl<'p, 'o> EditPermissionProposalExecute<'p, 'o> {
    pub fn new(
        proposal: &'p Proposal,
        operation: &'o EditPermissionOperation,
        policy_service: Arc<PermissionService>,
    ) -> Self {
        Self {
            proposal,
            operation,
            policy_service,
        }
    }
}

#[async_trait]
impl Execute for EditPermissionProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        self.policy_service
            .edit_permission(self.operation.input.to_owned())
            .await
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to update permission: {}", e),
            })?;

        Ok(ProposalExecuteStage::Completed(
            self.proposal.operation.to_owned(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::permission::permission_test_utils::mock_permission,
        repositories::{permission::PERMISSION_REPOSITORY, PROPOSAL_REPOSITORY},
        services::permission::PERMISSION_SERVICE,
    };
    use orbit_essentials::{model::ModelKey, repository::Repository};

    #[test]
    fn test_create_proposal() {
        let proposal_id = [0u8; 16];
        let proposed_by_user = [1u8; 16];
        let operation_input = edit_permission_test_utils::mock_edit_permission_api_input();
        let mut proposal_input = edit_permission_test_utils::mock_proposal_api_input();
        proposal_input.operation =
            station_api::ProposalOperationInput::EditPermission(operation_input.clone());

        let proposal = EditPermissionProposalCreate::create(
            proposal_id,
            proposed_by_user,
            proposal_input,
            operation_input,
        )
        .unwrap();

        assert_eq!(proposal.id, proposal_id);
        assert_eq!(proposal.proposed_by, proposed_by_user);
        assert_eq!(proposal.title, "Permission update".to_string());
    }

    #[tokio::test]
    async fn test_execute_proposal_completed() {
        let proposal_id = [0u8; 16];
        let proposed_by_user = [1u8; 16];
        let operation_input = edit_permission_test_utils::mock_edit_permission_api_input();
        let mut proposal_input = edit_permission_test_utils::mock_proposal_api_input();
        proposal_input.operation =
            station_api::ProposalOperationInput::EditPermission(operation_input.clone());

        let proposal = EditPermissionProposalCreate::create(
            proposal_id,
            proposed_by_user,
            proposal_input,
            operation_input,
        )
        .unwrap();

        PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.to_owned());

        if let ProposalOperation::EditPermission(operation) = &proposal.operation {
            let policy = mock_permission();
            PERMISSION_REPOSITORY.insert(policy.key(), policy.to_owned());

            let stage = EditPermissionProposalExecute::new(
                &proposal,
                operation,
                Arc::clone(&PERMISSION_SERVICE),
            )
            .execute()
            .await
            .unwrap();

            match stage {
                ProposalExecuteStage::Completed(_) => (),
                _ => panic!("Expected ProposalExecuteStage::Completed, got {:?}", stage),
            }
        } else {
            panic!(
                "Expected EditPermission operation, got {:?}",
                proposal.operation
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

    pub fn mock_proposal_api_input() -> station_api::CreateProposalInput {
        station_api::CreateProposalInput {
            operation: station_api::ProposalOperationInput::EditPermission(
                mock_edit_permission_api_input(),
            ),
            title: None,
            summary: None,
            execution_plan: None,
        }
    }
}
