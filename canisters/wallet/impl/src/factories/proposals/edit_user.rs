use super::{Create, CreateHook, Evaluate, Execute, ProposalExecuteStage, Validate};
use crate::{
    errors::{ProposalError, ProposalEvaluateError, ProposalExecuteError},
    models::{EditUserOperation, PolicyStatus, Proposal},
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;

pub struct EditUserProposalCreate {}

impl Create<wallet_api::EditUserOperationInput> for EditUserProposalCreate {
    fn create(
        _proposal_id: UUID,
        _proposed_by_user: UUID,
        _input: wallet_api::CreateProposalInput,
        _operation_input: wallet_api::EditUserOperationInput,
    ) -> Result<Proposal, ProposalError> {
        todo!()
    }
}

pub struct EditUserProposalCreateHook<'p, 'o> {
    _proposal: &'p Proposal,
    _operation: &'o EditUserOperation,
}

impl<'p, 'o> EditUserProposalCreateHook<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o EditUserOperation) -> Self {
        Self {
            _proposal: proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl CreateHook for EditUserProposalCreateHook<'_, '_> {
    async fn on_created(&self) {
        // TODO: Add once policy design is ready
    }
}

pub struct EditUserProposalValidate<'p, 'o> {
    proposal: &'p Proposal,
    _operation: &'o EditUserOperation,
}

impl<'p, 'o> EditUserProposalValidate<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o EditUserOperation) -> Self {
        Self {
            proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl Validate for EditUserProposalValidate<'_, '_> {
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

pub struct EditUserProposalEvaluate<'p, 'o> {
    _proposal: &'p Proposal,
    _operation: &'o EditUserOperation,
}

impl<'p, 'o> EditUserProposalEvaluate<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o EditUserOperation) -> Self {
        Self {
            _proposal: proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl Evaluate for EditUserProposalEvaluate<'_, '_> {
    async fn evaluate(&self) -> Result<PolicyStatus, ProposalEvaluateError> {
        // TODO: Add once final policy design is ready

        Ok(PolicyStatus::Accepted)
    }
}

pub struct EditUserProposalExecute<'p, 'o> {
    _proposal: &'p Proposal,
    _operation: &'o EditUserOperation,
}

impl<'p, 'o> EditUserProposalExecute<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o EditUserOperation) -> Self {
        Self {
            _proposal: proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl Execute for EditUserProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        todo!()
    }
}
