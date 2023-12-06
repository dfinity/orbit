use super::{Create, CreateHook, Evaluate, Execute, ProposalExecuteStage, Validate};
use crate::{
    errors::{ProposalError, ProposalEvaluateError, ProposalExecuteError},
    models::{AddUserOperation, EvaluationStatus, Proposal},
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;

pub struct AddUserProposalCreate {}

impl Create<wallet_api::AddUserOperationInput> for AddUserProposalCreate {
    fn create(
        _proposal_id: UUID,
        _proposed_by_user: UUID,
        _input: wallet_api::CreateProposalInput,
        _operation_input: wallet_api::AddUserOperationInput,
    ) -> Result<Proposal, ProposalError> {
        todo!()
    }
}

pub struct AddUserProposalCreateHook<'p, 'o> {
    _proposal: &'p Proposal,
    _operation: &'o AddUserOperation,
}

impl<'p, 'o> AddUserProposalCreateHook<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o AddUserOperation) -> Self {
        Self {
            _proposal: proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl CreateHook for AddUserProposalCreateHook<'_, '_> {
    async fn on_created(&self) {
        // TODO: Add once policy design is ready
    }
}

pub struct AddUserProposalValidate<'p, 'o> {
    proposal: &'p Proposal,
    _operation: &'o AddUserOperation,
}

impl<'p, 'o> AddUserProposalValidate<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o AddUserOperation) -> Self {
        Self {
            proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl Validate for AddUserProposalValidate<'_, '_> {
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

pub struct AddUserProposalEvaluate<'p, 'o> {
    _proposal: &'p Proposal,
    _operation: &'o AddUserOperation,
}

impl<'p, 'o> AddUserProposalEvaluate<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o AddUserOperation) -> Self {
        Self {
            _proposal: proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl Evaluate for AddUserProposalEvaluate<'_, '_> {
    async fn evaluate(&self) -> Result<EvaluationStatus, ProposalEvaluateError> {
        // TODO: Add once final policy design is ready

        Ok(EvaluationStatus::Adopted)
    }
}

pub struct AddUserProposalExecute<'p, 'o> {
    _proposal: &'p Proposal,
    _operation: &'o AddUserOperation,
}

impl<'p, 'o> AddUserProposalExecute<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o AddUserOperation) -> Self {
        Self {
            _proposal: proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl Execute for AddUserProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        todo!()
    }
}
