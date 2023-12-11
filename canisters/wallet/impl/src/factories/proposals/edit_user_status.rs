use super::{Create, CreateHook, Execute, ProposalExecuteStage};
use crate::{
    errors::{ProposalError, ProposalExecuteError},
    models::{EditUserStatusOperation, Proposal},
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
