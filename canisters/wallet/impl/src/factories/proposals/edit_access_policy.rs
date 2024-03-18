use super::{Create, Execute, ProposalExecuteStage};
use crate::{
    errors::{ProposalError, ProposalExecuteError},
    models::{EditAccessPolicyOperation, Proposal, ProposalExecutionPlan, ProposalOperation},
    services::access_policy::AccessPolicyService,
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;
use std::sync::Arc;

pub struct EditAccessPolicyProposalCreate {}

impl Create<wallet_api::EditAccessPolicyOperationInput> for EditAccessPolicyProposalCreate {
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: wallet_api::CreateProposalInput,
        operation_input: wallet_api::EditAccessPolicyOperationInput,
    ) -> Result<Proposal, ProposalError> {
        let proposal = Proposal::new(
            proposal_id,
            proposed_by_user,
            Proposal::default_expiration_dt_ns(),
            ProposalOperation::EditAccessPolicy(EditAccessPolicyOperation {
                input: operation_input.into(),
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(ProposalExecutionPlan::Immediate),
            input
                .title
                .unwrap_or_else(|| "Access policy update".to_string()),
            input.summary,
        );

        Ok(proposal)
    }
}

pub struct EditAccessPolicyProposalExecute<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o EditAccessPolicyOperation,
    policy_service: Arc<AccessPolicyService>,
}

impl<'p, 'o> EditAccessPolicyProposalExecute<'p, 'o> {
    pub fn new(
        proposal: &'p Proposal,
        operation: &'o EditAccessPolicyOperation,
        policy_service: Arc<AccessPolicyService>,
    ) -> Self {
        Self {
            proposal,
            operation,
            policy_service,
        }
    }
}

#[async_trait]
impl Execute for EditAccessPolicyProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        self.policy_service
            .edit_access_policy(self.operation.input.to_owned())
            .await
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to update access policy: {}", e),
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
        models::access_policy::access_policy_test_utils::mock_access_policy,
        repositories::{access_policy::ACCESS_POLICY_REPOSITORY, PROPOSAL_REPOSITORY},
        services::access_policy::ACCESS_POLICY_SERVICE,
    };
    use ic_canister_core::{model::ModelKey, repository::Repository};

    #[test]
    fn test_create_proposal() {
        let proposal_id = [0u8; 16];
        let proposed_by_user = [1u8; 16];
        let operation_input = edit_access_policy_test_utils::mock_edit_access_policy_api_input();
        let mut proposal_input = edit_access_policy_test_utils::mock_proposal_api_input();
        proposal_input.operation =
            wallet_api::ProposalOperationInput::EditAccessPolicy(operation_input.clone());

        let proposal = EditAccessPolicyProposalCreate::create(
            proposal_id,
            proposed_by_user,
            proposal_input,
            operation_input,
        )
        .unwrap();

        assert_eq!(proposal.id, proposal_id);
        assert_eq!(proposal.proposed_by, proposed_by_user);
        assert_eq!(proposal.title, "Access policy update".to_string());
    }

    #[tokio::test]
    async fn test_execute_proposal_completed() {
        let proposal_id = [0u8; 16];
        let proposed_by_user = [1u8; 16];
        let operation_input = edit_access_policy_test_utils::mock_edit_access_policy_api_input();
        let mut proposal_input = edit_access_policy_test_utils::mock_proposal_api_input();
        proposal_input.operation =
            wallet_api::ProposalOperationInput::EditAccessPolicy(operation_input.clone());

        let proposal = EditAccessPolicyProposalCreate::create(
            proposal_id,
            proposed_by_user,
            proposal_input,
            operation_input,
        )
        .unwrap();

        PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.to_owned());

        if let ProposalOperation::EditAccessPolicy(operation) = &proposal.operation {
            let policy = mock_access_policy();
            ACCESS_POLICY_REPOSITORY.insert(policy.key(), policy.to_owned());

            let stage = EditAccessPolicyProposalExecute::new(
                &proposal,
                operation,
                Arc::clone(&ACCESS_POLICY_SERVICE),
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
                "Expected EditAccessPolicy operation, got {:?}",
                proposal.operation
            );
        }
    }
}

#[cfg(test)]
pub mod edit_access_policy_test_utils {
    use uuid::Uuid;

    pub fn mock_edit_access_policy_api_input() -> wallet_api::EditAccessPolicyOperationInput {
        wallet_api::EditAccessPolicyOperationInput {
            resource: wallet_api::ResourceDTO::AccessPolicy(
                wallet_api::AccessPolicyResourceActionDTO::Read,
            ),
            auth_scope: None,
            user_groups: None,
            users: Some(vec![Uuid::from_bytes([1u8; 16]).hyphenated().to_string()]),
        }
    }

    pub fn mock_proposal_api_input() -> wallet_api::CreateProposalInput {
        wallet_api::CreateProposalInput {
            operation: wallet_api::ProposalOperationInput::EditAccessPolicy(
                mock_edit_access_policy_api_input(),
            ),
            title: None,
            summary: None,
            execution_plan: None,
        }
    }
}
