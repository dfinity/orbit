use super::{Create, CreateHook, Execute, ProposalExecuteStage, Validate};
use crate::{
    errors::{ProposalError, ProposalExecuteError},
    models::{Proposal, ProposalExecutionPlan, ProposalOperation, UpgradeOperation},
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;
use wallet_api::{CreateProposalInput, UpgradeOperationInput};

pub struct UpgradeProposalCreate;

impl Create<UpgradeOperationInput> for UpgradeProposalCreate {
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: CreateProposalInput,
        operation_input: UpgradeOperationInput,
    ) -> Result<Proposal, ProposalError> {
        let proposal = Proposal::new(
            proposal_id,
            proposed_by_user,
            Proposal::default_expiration_dt_ns(),
            ProposalOperation::Upgrade(UpgradeOperation {
                input: operation_input.into(),
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(ProposalExecutionPlan::Immediate),
            input.title.unwrap_or_else(|| "Upgrade".to_string()),
            input.summary,
        );

        Ok(proposal)
    }
}

pub struct UpgradeProposalCreateHook<'p, 'o> {
    _proposal: &'p Proposal,
    _operation: &'o UpgradeOperation,
}

impl<'p, 'o> UpgradeProposalCreateHook<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o UpgradeOperation) -> Self {
        Self {
            _proposal: proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl CreateHook for UpgradeProposalCreateHook<'_, '_> {
    async fn on_created(&self) {}
}

pub struct UpgradeProposalValidate<'p, 'o> {
    proposal: &'p Proposal,
    _operation: &'o UpgradeOperation,
}

impl<'p, 'o> UpgradeProposalValidate<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o UpgradeOperation) -> Self {
        Self {
            proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl Validate for UpgradeProposalValidate<'_, '_> {
    fn can_vote(&self, _user_id: &UUID) -> bool {
        false
    }

    fn can_view(&self, user_id: &UUID) -> bool {
        self.can_vote(user_id)
            || self.proposal.voters().contains(user_id)
            || self.proposal.proposed_by == *user_id
    }
}

pub struct UpgradeProposalExecute<'p, 'o> {
    _proposal: &'p Proposal,
    _operation: &'o UpgradeOperation,
}

impl<'p, 'o> UpgradeProposalExecute<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o UpgradeOperation) -> Self {
        Self {
            _proposal: proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl Execute for UpgradeProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        todo!()
    }
}
