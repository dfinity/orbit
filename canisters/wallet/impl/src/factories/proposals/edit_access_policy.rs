use std::sync::Arc;

use super::{Create, CreateHook, Execute, ProposalExecuteStage};
use crate::{
    errors::{ProposalError, ProposalExecuteError},
    models::{EditAccessPolicyOperation, Proposal, ProposalExecutionPlan, ProposalOperation},
    services::PolicyService,
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;

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

pub struct EditAccessPolicyProposalCreateHook<'p, 'o> {
    _proposal: &'p Proposal,
    _operation: &'o EditAccessPolicyOperation,
}

impl<'p, 'o> EditAccessPolicyProposalCreateHook<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o EditAccessPolicyOperation) -> Self {
        Self {
            _proposal: proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl CreateHook for EditAccessPolicyProposalCreateHook<'_, '_> {
    async fn on_created(&self) {
        // TODO: Add once policy design is ready
    }
}

pub struct EditAccessPolicyProposalExecute<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o EditAccessPolicyOperation,
    policy_service: Arc<PolicyService>,
}

impl<'p, 'o> EditAccessPolicyProposalExecute<'p, 'o> {
    pub fn new(
        proposal: &'p Proposal,
        operation: &'o EditAccessPolicyOperation,
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
