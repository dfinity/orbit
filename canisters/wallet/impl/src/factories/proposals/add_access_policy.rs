use super::{Create, Execute, ProposalExecuteStage};
use crate::{
    errors::{ProposalError, ProposalExecuteError},
    models::{AddAccessPolicyOperation, Proposal, ProposalExecutionPlan, ProposalOperation},
    services::PolicyService,
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;
use std::sync::Arc;

pub struct AddAccessPolicyProposalCreate {}

impl Create<wallet_api::AddAccessPolicyOperationInput> for AddAccessPolicyProposalCreate {
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: wallet_api::CreateProposalInput,
        operation_input: wallet_api::AddAccessPolicyOperationInput,
    ) -> Result<Proposal, ProposalError> {
        let proposal = Proposal::new(
            proposal_id,
            proposed_by_user,
            Proposal::default_expiration_dt_ns(),
            ProposalOperation::AddAccessPolicy(AddAccessPolicyOperation {
                policy_id: None,
                input: operation_input.into(),
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(ProposalExecutionPlan::Immediate),
            input
                .title
                .unwrap_or_else(|| "Access policy creation".to_string()),
            input.summary,
        );

        Ok(proposal)
    }
}

pub struct AddAccessPolicyProposalExecute<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o AddAccessPolicyOperation,
    policy_service: Arc<PolicyService>,
}

impl<'p, 'o> AddAccessPolicyProposalExecute<'p, 'o> {
    pub fn new(
        proposal: &'p Proposal,
        operation: &'o AddAccessPolicyOperation,
        policy_service: Arc<PolicyService>,
    ) -> Self {
        Self {
            proposal,
            operation,
            policy_service,
        }
    }
}

#[async_trait]
impl Execute for AddAccessPolicyProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        let policy = self
            .policy_service
            .add_access_policy(self.operation.input.to_owned())
            .await
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to create access policy: {}", e),
            })?;

        let mut operation = self.proposal.operation.clone();

        if let ProposalOperation::AddAccessPolicy(ref mut operation) = operation {
            operation.policy_id = Some(policy.id);
        }

        Ok(ProposalExecuteStage::Completed(operation))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{repositories::PROPOSAL_REPOSITORY, services::POLICY_SERVICE};
    use ic_canister_core::repository::Repository;

    #[test]
    fn test_create_proposal() {
        let proposal_id = [0u8; 16];
        let proposed_by_user = [1u8; 16];
        let operation_input = add_access_policy_test_utils::mock_add_access_policy_api_input();
        let mut proposal_input = add_access_policy_test_utils::mock_proposal_api_input();
        proposal_input.operation =
            wallet_api::ProposalOperationInput::AddAccessPolicy(operation_input.clone());

        let proposal = AddAccessPolicyProposalCreate::create(
            proposal_id,
            proposed_by_user,
            proposal_input,
            operation_input,
        )
        .unwrap();

        assert_eq!(proposal.id, proposal_id);
        assert_eq!(proposal.proposed_by, proposed_by_user);
        assert_eq!(proposal.title, "Access policy creation".to_string());
    }

    #[tokio::test]
    async fn test_execute_proposal_completed() {
        let proposal_id = [0u8; 16];
        let proposed_by_user = [1u8; 16];
        let operation_input = add_access_policy_test_utils::mock_add_access_policy_api_input();
        let mut proposal_input = add_access_policy_test_utils::mock_proposal_api_input();
        proposal_input.operation =
            wallet_api::ProposalOperationInput::AddAccessPolicy(operation_input.clone());

        let proposal = AddAccessPolicyProposalCreate::create(
            proposal_id,
            proposed_by_user,
            proposal_input,
            operation_input,
        )
        .unwrap();

        PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.to_owned());

        if let ProposalOperation::AddAccessPolicy(operation) = &proposal.operation {
            let stage = AddAccessPolicyProposalExecute::new(
                &proposal,
                operation,
                Arc::clone(&POLICY_SERVICE),
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
                "Expected AddAccessPolicy operation, got {:?}",
                proposal.operation
            );
        }
    }
}

#[cfg(test)]
pub mod add_access_policy_test_utils {
    use uuid::Uuid;

    pub fn mock_add_access_policy_api_input() -> wallet_api::AddAccessPolicyOperationInput {
        wallet_api::AddAccessPolicyOperationInput {
            resource: wallet_api::ResourceSpecifierDTO::AccessPolicy(
                wallet_api::CommonActionSpecifierDTO::Create,
            ),
            user: wallet_api::AccessControlUserSpecifierDTO::Id(vec![Uuid::from_bytes([1u8; 16])
                .hyphenated()
                .to_string()]),
        }
    }

    pub fn mock_proposal_api_input() -> wallet_api::CreateProposalInput {
        wallet_api::CreateProposalInput {
            operation: wallet_api::ProposalOperationInput::AddAccessPolicy(
                mock_add_access_policy_api_input(),
            ),
            title: None,
            summary: None,
            execution_plan: None,
        }
    }
}
