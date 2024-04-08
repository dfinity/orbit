use super::{Create, Execute, ProposalExecuteStage};
use crate::{
    errors::{ProposalError, ProposalExecuteError},
    models::{
        EditProposalPolicyOperation, EditProposalPolicyOperationInput, Proposal,
        ProposalExecutionPlan, ProposalOperation,
    },
    services::{ProposalPolicyService, PROPOSAL_POLICY_SERVICE},
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;
use std::sync::Arc;
use uuid::Uuid;

pub struct EditProposalPolicyProposalCreate {}

impl Create<wallet_api::EditProposalPolicyOperationInput> for EditProposalPolicyProposalCreate {
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: wallet_api::CreateProposalInput,
        operation_input: wallet_api::EditProposalPolicyOperationInput,
    ) -> Result<Proposal, ProposalError> {
        let operation_input = EditProposalPolicyOperationInput::from(operation_input);
        PROPOSAL_POLICY_SERVICE
            .get_proposal_policy(&operation_input.policy_id)
            .map_err(|_| ProposalError::ValidationError {
                info: format!(
                    "Proposal policy with id {} does not exist",
                    Uuid::from_bytes(operation_input.policy_id).hyphenated()
                ),
            })?;

        let proposal = Proposal::new(
            proposal_id,
            proposed_by_user,
            Proposal::default_expiration_dt_ns(),
            ProposalOperation::EditProposalPolicy(EditProposalPolicyOperation {
                input: operation_input,
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(ProposalExecutionPlan::Immediate),
            input
                .title
                .unwrap_or_else(|| "Proposal policy update".to_string()),
            input.summary,
        );

        Ok(proposal)
    }
}

pub struct EditProposalPolicyProposalExecute<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o EditProposalPolicyOperation,
    policy_service: Arc<ProposalPolicyService>,
}

impl<'p, 'o> EditProposalPolicyProposalExecute<'p, 'o> {
    pub fn new(
        proposal: &'p Proposal,
        operation: &'o EditProposalPolicyOperation,
        policy_service: Arc<ProposalPolicyService>,
    ) -> Self {
        Self {
            proposal,
            operation,
            policy_service,
        }
    }
}

#[async_trait]
impl Execute for EditProposalPolicyProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        self.policy_service
            .edit_proposal_policy(self.operation.input.to_owned())
            .await
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to update proposal policy: {}", e),
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
        models::proposal_policy_test_utils::mock_proposal_policy,
        repositories::{policy::PROPOSAL_POLICY_REPOSITORY, PROPOSAL_REPOSITORY},
    };
    use ic_canister_core::repository::Repository;
    use std::str::FromStr;

    #[test]
    fn test_create_proposal() {
        let proposal_id = [0u8; 16];
        let proposed_by_user = [1u8; 16];
        let operation_input =
            edit_proposal_policy_test_utils::mock_edit_proposal_policy_api_input();
        let mut proposal_input = edit_proposal_policy_test_utils::mock_proposal_api_input();
        proposal_input.operation =
            wallet_api::ProposalOperationInput::EditProposalPolicy(operation_input.clone());

        let mut policy = mock_proposal_policy();
        policy.id = *Uuid::from_str(&operation_input.policy_id)
            .unwrap()
            .as_bytes();
        PROPOSAL_POLICY_REPOSITORY.insert(policy.id, policy.to_owned());

        let proposal = EditProposalPolicyProposalCreate::create(
            proposal_id,
            proposed_by_user,
            proposal_input,
            operation_input,
        )
        .unwrap();

        assert_eq!(proposal.id, proposal_id);
        assert_eq!(proposal.proposed_by, proposed_by_user);
        assert_eq!(proposal.title, "Proposal policy update".to_string());
    }

    #[tokio::test]
    async fn test_execute_proposal_completed() {
        let proposal_id = [0u8; 16];
        let proposed_by_user = [1u8; 16];
        let operation_input =
            edit_proposal_policy_test_utils::mock_edit_proposal_policy_api_input();
        let mut proposal_input = edit_proposal_policy_test_utils::mock_proposal_api_input();
        proposal_input.operation =
            wallet_api::ProposalOperationInput::EditProposalPolicy(operation_input.clone());

        let mut policy = mock_proposal_policy();
        policy.id = *Uuid::from_str(&operation_input.policy_id)
            .unwrap()
            .as_bytes();
        PROPOSAL_POLICY_REPOSITORY.insert(policy.id, policy.to_owned());

        let proposal = EditProposalPolicyProposalCreate::create(
            proposal_id,
            proposed_by_user,
            proposal_input,
            operation_input,
        )
        .unwrap();

        PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.to_owned());

        if let ProposalOperation::EditProposalPolicy(operation) = &proposal.operation {
            let mut policy = mock_proposal_policy();
            policy.id = operation.input.policy_id;
            PROPOSAL_POLICY_REPOSITORY.insert(policy.id, policy.to_owned());

            let stage = EditProposalPolicyProposalExecute::new(
                &proposal,
                operation,
                Arc::clone(&PROPOSAL_POLICY_SERVICE),
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
                "Expected EditProposalPolicy operation, got {:?}",
                proposal.operation
            );
        }
    }

    #[tokio::test]
    async fn test_execute_proposal_should_fail_non_existant_policy() {
        let proposal_id = [0u8; 16];
        let proposed_by_user = [1u8; 16];
        let operation_input =
            edit_proposal_policy_test_utils::mock_edit_proposal_policy_api_input();
        let mut proposal_input = edit_proposal_policy_test_utils::mock_proposal_api_input();
        proposal_input.operation =
            wallet_api::ProposalOperationInput::EditProposalPolicy(operation_input.clone());

        let mut policy = mock_proposal_policy();
        policy.id = *Uuid::from_str(&operation_input.policy_id)
            .unwrap()
            .as_bytes();
        PROPOSAL_POLICY_REPOSITORY.insert(policy.id, policy.to_owned());

        let proposal = EditProposalPolicyProposalCreate::create(
            proposal_id,
            proposed_by_user,
            proposal_input,
            operation_input,
        )
        .unwrap();

        PROPOSAL_POLICY_REPOSITORY.remove(&policy.id);

        PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.to_owned());

        if let ProposalOperation::EditProposalPolicy(operation) = &proposal.operation {
            let stage = EditProposalPolicyProposalExecute::new(
                &proposal,
                operation,
                Arc::clone(&PROPOSAL_POLICY_SERVICE),
            )
            .execute()
            .await;

            assert!(stage.is_err());
        } else {
            panic!(
                "Expected EditProposalPolicy operation, got {:?}",
                proposal.operation
            );
        }
    }
}

#[cfg(test)]
pub mod edit_proposal_policy_test_utils {
    use uuid::Uuid;

    pub fn mock_edit_proposal_policy_api_input() -> wallet_api::EditProposalPolicyOperationInput {
        wallet_api::EditProposalPolicyOperationInput {
            policy_id: Uuid::from_bytes([0u8; 16]).hyphenated().to_string(),
            criteria: Some(wallet_api::CriteriaDTO::AutoAdopted),
            specifier: Some(wallet_api::ProposalSpecifierDTO::EditProposalPolicy(
                wallet_api::ResourceIdsDTO::Any,
            )),
        }
    }

    pub fn mock_proposal_api_input() -> wallet_api::CreateProposalInput {
        wallet_api::CreateProposalInput {
            operation: wallet_api::ProposalOperationInput::EditProposalPolicy(
                mock_edit_proposal_policy_api_input(),
            ),
            title: None,
            summary: None,
            execution_plan: None,
        }
    }
}
