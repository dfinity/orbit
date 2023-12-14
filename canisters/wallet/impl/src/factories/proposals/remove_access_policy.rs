use std::sync::Arc;

use super::{Create, CreateHook, Execute, ProposalExecuteStage};
use crate::{
    errors::{ProposalError, ProposalExecuteError},
    models::{Proposal, ProposalExecutionPlan, ProposalOperation, RemoveAccessPolicyOperation},
    services::PolicyService,
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;

pub struct RemoveAccessPolicyProposalCreate {}

impl Create<wallet_api::RemoveAccessPolicyOperationInput> for RemoveAccessPolicyProposalCreate {
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: wallet_api::CreateProposalInput,
        operation_input: wallet_api::RemoveAccessPolicyOperationInput,
    ) -> Result<Proposal, ProposalError> {
        let proposal = Proposal::new(
            proposal_id,
            proposed_by_user,
            Proposal::default_expiration_dt_ns(),
            ProposalOperation::RemoveAccessPolicy(RemoveAccessPolicyOperation {
                input: operation_input.into(),
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(ProposalExecutionPlan::Immediate),
            input
                .title
                .unwrap_or_else(|| "Access policy remove".to_string()),
            input.summary,
        );

        Ok(proposal)
    }
}

pub struct RemoveAccessPolicyProposalCreateHook<'p, 'o> {
    _proposal: &'p Proposal,
    _operation: &'o RemoveAccessPolicyOperation,
}

impl<'p, 'o> RemoveAccessPolicyProposalCreateHook<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o RemoveAccessPolicyOperation) -> Self {
        Self {
            _proposal: proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl CreateHook for RemoveAccessPolicyProposalCreateHook<'_, '_> {
    async fn on_created(&self) {
        // TODO: Add once policy design is ready
    }
}

pub struct RemoveAccessPolicyProposalExecute<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o RemoveAccessPolicyOperation,
    policy_service: Arc<PolicyService>,
}

impl<'p, 'o> RemoveAccessPolicyProposalExecute<'p, 'o> {
    pub fn new(
        proposal: &'p Proposal,
        operation: &'o RemoveAccessPolicyOperation,
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
impl Execute for RemoveAccessPolicyProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        self.policy_service
            .remove_access_policy(&self.operation.input.policy_id)
            .await
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to remove access policy: {}", e),
            })?;

        Ok(ProposalExecuteStage::Completed(
            self.proposal.operation.to_owned(),
        ))
    }
}
