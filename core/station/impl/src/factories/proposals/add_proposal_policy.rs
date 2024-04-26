use std::sync::Arc;

use super::{Create, Execute, ProposalExecuteStage};
use crate::{
    errors::{ProposalError, ProposalExecuteError},
    models::{AddProposalPolicyOperation, Proposal, ProposalExecutionPlan, ProposalOperation},
    services::ProposalPolicyService,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

pub struct AddProposalPolicyProposalCreate {}

impl Create<station_api::AddProposalPolicyOperationInput> for AddProposalPolicyProposalCreate {
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: station_api::CreateProposalInput,
        operation_input: station_api::AddProposalPolicyOperationInput,
    ) -> Result<Proposal, ProposalError> {
        let proposal = Proposal::new(
            proposal_id,
            proposed_by_user,
            Proposal::default_expiration_dt_ns(),
            ProposalOperation::AddProposalPolicy(AddProposalPolicyOperation {
                policy_id: None,
                input: operation_input.into(),
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(ProposalExecutionPlan::Immediate),
            input
                .title
                .unwrap_or_else(|| "Proposal policy creation".to_string()),
            input.summary,
        );

        Ok(proposal)
    }
}

pub struct AddProposalPolicyProposalExecute<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o AddProposalPolicyOperation,
    policy_service: Arc<ProposalPolicyService>,
}

impl<'p, 'o> AddProposalPolicyProposalExecute<'p, 'o> {
    pub fn new(
        proposal: &'p Proposal,
        operation: &'o AddProposalPolicyOperation,
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
impl Execute for AddProposalPolicyProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        let policy = self
            .policy_service
            .add_proposal_policy(self.operation.input.to_owned())
            .await
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to create proposal policy: {}", e),
            })?;

        let mut operation = self.proposal.operation.clone();

        if let ProposalOperation::AddProposalPolicy(ref mut operation) = operation {
            operation.policy_id = Some(policy.id);
        }

        Ok(ProposalExecuteStage::Completed(operation))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{repositories::PROPOSAL_REPOSITORY, services::PROPOSAL_POLICY_SERVICE};
    use orbit_essentials::repository::Repository;

    #[test]
    fn test_create_proposal() {
        let proposal_id = [0u8; 16];
        let proposed_by_user = [1u8; 16];
        let operation_input = add_proposal_policy_test_utils::mock_add_proposal_policy_api_input();
        let mut proposal_input = add_proposal_policy_test_utils::mock_proposal_api_input();
        proposal_input.operation =
            station_api::ProposalOperationInput::AddProposalPolicy(operation_input.clone());

        let proposal = AddProposalPolicyProposalCreate::create(
            proposal_id,
            proposed_by_user,
            proposal_input,
            operation_input,
        )
        .unwrap();

        assert_eq!(proposal.id, proposal_id);
        assert_eq!(proposal.proposed_by, proposed_by_user);
        assert_eq!(proposal.title, "Proposal policy creation".to_string());
    }

    #[tokio::test]
    async fn test_execute_proposal_completed() {
        let proposal_id = [0u8; 16];
        let proposed_by_user = [1u8; 16];
        let operation_input = add_proposal_policy_test_utils::mock_add_proposal_policy_api_input();
        let mut proposal_input = add_proposal_policy_test_utils::mock_proposal_api_input();
        proposal_input.operation =
            station_api::ProposalOperationInput::AddProposalPolicy(operation_input.clone());

        let proposal = AddProposalPolicyProposalCreate::create(
            proposal_id,
            proposed_by_user,
            proposal_input,
            operation_input,
        )
        .unwrap();

        PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.to_owned());

        if let ProposalOperation::AddProposalPolicy(operation) = &proposal.operation {
            let stage = AddProposalPolicyProposalExecute::new(
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
                "Expected AddProposalPolicy operation, got {:?}",
                proposal.operation
            );
        }
    }
}

#[cfg(test)]
pub mod add_proposal_policy_test_utils {
    pub fn mock_add_proposal_policy_api_input() -> station_api::AddProposalPolicyOperationInput {
        station_api::AddProposalPolicyOperationInput {
            criteria: station_api::CriteriaDTO::AutoAdopted,
            specifier: station_api::ProposalSpecifierDTO::AddProposalPolicy,
        }
    }

    pub fn mock_proposal_api_input() -> station_api::CreateProposalInput {
        station_api::CreateProposalInput {
            operation: station_api::ProposalOperationInput::AddProposalPolicy(
                mock_add_proposal_policy_api_input(),
            ),
            title: None,
            summary: None,
            execution_plan: None,
        }
    }
}
