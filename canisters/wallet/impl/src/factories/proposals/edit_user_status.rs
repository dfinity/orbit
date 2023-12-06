use super::{Create, CreateHook, Evaluate, Execute, ProposalExecuteStage, Validate};
use crate::{
    errors::{ProposalError, ProposalEvaluateError, ProposalExecuteError},
    models::{EditUserStatusOperation, EvaluationStatus, Proposal},
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;

pub struct EditUserStatusProposalCreate {}

impl Create<wallet_api::EditUserStatusOperationInput> for EditUserStatusProposalCreate {
    fn create(
        _proposal_id: UUID,
        _proposed_by_user: UUID,
        _input: wallet_api::CreateProposalInput,
        _operation_input: wallet_api::EditUserStatusOperationInput,
    ) -> Result<Proposal, ProposalError> {
        todo!()
    }
}

pub struct EditUserStatusProposalCreateHook<'p, 'o> {
    _proposal: &'p Proposal,
    _operation: &'o EditUserStatusOperation,
}

impl<'p, 'o> EditUserStatusProposalCreateHook<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o EditUserStatusOperation) -> Self {
        Self {
            _proposal: proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl CreateHook for EditUserStatusProposalCreateHook<'_, '_> {
    async fn on_created(&self) {
        // TODO: Add once policy design is ready
    }
}

pub struct EditUserStatusProposalValidate<'p, 'o> {
    proposal: &'p Proposal,
    _operation: &'o EditUserStatusOperation,
}

impl<'p, 'o> EditUserStatusProposalValidate<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o EditUserStatusOperation) -> Self {
        Self {
            proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl Validate for EditUserStatusProposalValidate<'_, '_> {
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

pub struct EditUserStatusProposalEvaluate<'p, 'o> {
    _proposal: &'p Proposal,
    _operation: &'o EditUserStatusOperation,
}

impl<'p, 'o> EditUserStatusProposalEvaluate<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o EditUserStatusOperation) -> Self {
        Self {
            _proposal: proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl Evaluate for EditUserStatusProposalEvaluate<'_, '_> {
    async fn evaluate(&self) -> Result<EvaluationStatus, ProposalEvaluateError> {
        // TODO: Add once final policy design is ready

        Ok(EvaluationStatus::Adopted)
    }
}

pub struct EditUserStatusProposalExecute<'p, 'o> {
    _proposal: &'p Proposal,
    _operation: &'o EditUserStatusOperation,
}

impl<'p, 'o> EditUserStatusProposalExecute<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o EditUserStatusOperation) -> Self {
        Self {
            _proposal: proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl Execute for EditUserStatusProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        todo!()
    }
}
