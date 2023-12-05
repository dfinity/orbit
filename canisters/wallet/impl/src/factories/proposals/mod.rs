use crate::{
    core::{generate_uuid_v4, ic_cdk::api::trap},
    errors::{ProposalError, ProposalExecuteError},
    models::{Policy, PolicyStatus, Proposal, ProposalOperation},
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;
use wallet_api::{CreateProposalInput, ProposalOperationInput};

// mod add_account;
// mod add_user_group;
// mod edit_account;
// mod edit_user_group;
// mod remove_user_group;
mod transfer;

use self::transfer::{
    TransferProposalCreate, TransferProposalCreateHook, TransferProposalEvaluate,
    TransferProposalExecute, TransferProposalValidate,
};

#[derive(Debug)]
pub enum ProposalExecuteStage {
    Completed(ProposalOperation),
    Processing(ProposalOperation),
}

#[async_trait]
pub trait Execute: Send + Sync {
    /// Executes the proposal and returns the operation that was executed with the stage that the execution is in.
    ///
    /// The stage is used to indicate if the operation was completed or if it is still processing.
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError>;
}

#[async_trait]
pub trait Evaluate: Send + Sync {
    /// Reevaluates the status of the associated policies.
    async fn evaluate(&self) -> Vec<(Policy, PolicyStatus)>;
}

#[async_trait]
pub trait Validate: Send + Sync {
    /// Returns true if the user can vote on the proposal.
    ///
    /// Votes are only allowed if the proposal is still open and has policies that
    /// include voting such as approval threshold, minimun votes, veto votes, etc...
    fn can_vote(&self, user_id: &UUID) -> bool;

    /// Checks if the user has access to view the proposal details.
    fn can_view(&self, user_id: &UUID) -> bool;
}

#[async_trait]
pub trait Create<T>: Send + Sync {
    /// Creates a new proposal for the operation but does not save it.
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: CreateProposalInput,
        operation_input: T,
    ) -> Result<Proposal, ProposalError>
    where
        Self: Sized;
}

#[async_trait]
pub trait CreateHook: Send + Sync {
    /// The post create hook is called after the proposal is created and can be used
    /// for additional processing (e.g. sending notifications).
    async fn on_created(&self) {
        // noop by default
    }
}

fn create_proposal<OperationInput, Creator: Create<OperationInput>>(
    proposal_id: UUID,
    proposed_by_user: UUID,
    input: CreateProposalInput,
    operation_input: OperationInput,
) -> Result<Proposal, ProposalError> {
    Creator::create(proposal_id, proposed_by_user, input, operation_input)
}

#[derive(Debug)]
pub struct ProposalFactory {}

impl ProposalFactory {
    pub async fn create_proposal(
        proposed_by_user: UUID,
        input: CreateProposalInput,
    ) -> Result<Proposal, ProposalError> {
        let id = *generate_uuid_v4().await.as_bytes();

        match &input.operation {
            ProposalOperationInput::Transfer(operation) => {
                create_proposal::<wallet_api::TransferOperationInput, TransferProposalCreate>(
                    id,
                    proposed_by_user,
                    input.clone(),
                    operation.clone(),
                )
            }
            _ => trap(&format!("Not yet supported: {:?}", input.operation)),
        }
    }

    pub fn create_hook<'p>(proposal: &'p Proposal) -> Box<dyn CreateHook + 'p> {
        match &proposal.operation {
            ProposalOperation::Transfer(operation) => {
                Box::new(TransferProposalCreateHook::new(proposal, operation))
            }
            _ => trap(&format!("Not yet supported: {:?}", proposal.operation)),
        }
    }

    pub fn validator<'p>(proposal: &'p Proposal) -> Box<dyn Validate + 'p> {
        match &proposal.operation {
            ProposalOperation::Transfer(operation) => {
                Box::new(TransferProposalValidate::new(proposal, operation))
            }
            _ => trap(&format!("Not yet supported: {:?}", proposal.operation)),
        }
    }

    pub fn evaluator<'p>(proposal: &'p Proposal) -> Box<dyn Evaluate + 'p> {
        match &proposal.operation {
            ProposalOperation::Transfer(operation) => {
                Box::new(TransferProposalEvaluate::new(proposal, operation))
            }
            _ => trap(&format!("Not yet supported: {:?}", proposal.operation)),
        }
    }

    pub fn executor<'p>(proposal: &'p Proposal) -> Box<dyn Execute + 'p> {
        match &proposal.operation {
            ProposalOperation::Transfer(operation) => {
                Box::new(TransferProposalExecute::new(proposal, operation))
            }
            _ => trap(&format!("Not yet supported: {:?}", proposal.operation)),
        }
    }
}
