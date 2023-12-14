use std::sync::Arc;

use super::{Create, CreateHook, Execute, ProposalExecuteStage};
use crate::{
    errors::{ProposalError, ProposalExecuteError},
    models::{AddAccessPolicyOperation, Proposal, ProposalExecutionPlan, ProposalOperation},
    services::PolicyService,
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;

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

pub struct AddAccessPolicyProposalCreateHook<'p, 'o> {
    _proposal: &'p Proposal,
    _operation: &'o AddAccessPolicyOperation,
}

impl<'p, 'o> AddAccessPolicyProposalCreateHook<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o AddAccessPolicyOperation) -> Self {
        Self {
            _proposal: proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl CreateHook for AddAccessPolicyProposalCreateHook<'_, '_> {
    async fn on_created(&self) {
        // TODO: Add once policy design is ready
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
