use super::{Create, CreateHook, Evaluate, Execute, ProposalExecuteStage, Validate};
use crate::{
    errors::{ProposalError, ProposalEvaluateError, ProposalExecuteError},
    models::{
        EditUserGroupOperation, PolicyStatus, Proposal, ProposalExecutionPlan, ProposalOperation,
    },
    services::USER_GROUP_SERVICE,
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;

pub struct EditUserGroupProposalCreate {}

impl Create<wallet_api::EditUserGroupOperationInput> for EditUserGroupProposalCreate {
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: wallet_api::CreateProposalInput,
        operation_input: wallet_api::EditUserGroupOperationInput,
    ) -> Result<Proposal, ProposalError> {
        let proposal = Proposal::new(
            proposal_id,
            proposed_by_user,
            Proposal::default_expiration_dt_ns(),
            ProposalOperation::EditUserGroup(operation_input.into()),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(ProposalExecutionPlan::Immediate),
            input.title.unwrap_or_else(|| "User group edit".to_string()),
            input.summary,
        );

        Ok(proposal)
    }
}

pub struct EditUserGroupProposalCreateHook<'p, 'o> {
    _proposal: &'p Proposal,
    _operation: &'o EditUserGroupOperation,
}

impl<'p, 'o> EditUserGroupProposalCreateHook<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o EditUserGroupOperation) -> Self {
        Self {
            _proposal: proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl CreateHook for EditUserGroupProposalCreateHook<'_, '_> {
    async fn on_created(&self) {
        // TODO: Add once policy design is ready
    }
}

pub struct EditUserGroupProposalValidate<'p, 'o> {
    proposal: &'p Proposal,
    _operation: &'o EditUserGroupOperation,
}

impl<'p, 'o> EditUserGroupProposalValidate<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o EditUserGroupOperation) -> Self {
        Self {
            proposal,
            _operation: operation,
        }
    }
}

impl Validate for EditUserGroupProposalValidate<'_, '_> {
    fn can_vote(&self, _user_id: &UUID) -> bool {
        // TODO: Add once policy design is ready

        false
    }

    fn can_view(&self, user_id: &UUID) -> bool {
        self.can_vote(user_id)
            || self.proposal.voters().contains(user_id)
            || self.proposal.proposed_by == *user_id
    }
}

pub struct EditUserGroupProposalEvaluate<'p, 'o> {
    _proposal: &'p Proposal,
    _operation: &'o EditUserGroupOperation,
}

impl<'p, 'o> EditUserGroupProposalEvaluate<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o EditUserGroupOperation) -> Self {
        Self {
            _proposal: proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl Evaluate for EditUserGroupProposalEvaluate<'_, '_> {
    async fn evaluate(&self) -> Result<PolicyStatus, ProposalEvaluateError> {
        // TODO: Add once final policy design is ready

        Ok(PolicyStatus::Accepted)
    }
}

pub struct EditUserGroupProposalExecute<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o EditUserGroupOperation,
}

impl<'p, 'o> EditUserGroupProposalExecute<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o EditUserGroupOperation) -> Self {
        Self {
            proposal,
            operation,
        }
    }
}

#[async_trait]
impl Execute for EditUserGroupProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        USER_GROUP_SERVICE
            .edit(self.operation.input.clone())
            .await
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to edit user group: {}", e),
            })?;

        Ok(ProposalExecuteStage::Completed(
            self.proposal.operation.clone(),
        ))
    }
}
