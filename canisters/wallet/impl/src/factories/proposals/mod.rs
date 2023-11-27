use crate::{
    core::{generate_uuid_v4, ic_cdk::api::trap},
    errors::{ProposalError, ProposalExecuteError},
    models::{Policy, PolicyStatus, Proposal, ProposalExecutionPlan, ProposalOperation},
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;
use uuid::Uuid;
use wallet_api::{CreateProposalInput, ProposalOperationInput};

mod add_account;
mod edit_account;
mod transfer;

use add_account::AddAccountProposal;
use edit_account::EditAccountProposal;
use transfer::TransferProposal;

#[derive(Debug)]
pub enum ProposalExecuteStage {
    Completed(ProposalOperation),
    Processing(ProposalOperation),
}

#[async_trait]
pub trait ProposalHandler: Send + Sync {
    /// Reevaluates the status of the associated policies.
    fn evaluate_policies(&self) -> Vec<(Policy, PolicyStatus)>;

    /// Returns true if the user can vote on the proposal.
    fn can_vote(&self, user_id: &UUID) -> bool;

    /// Checks if the user has access to the proposal.
    fn has_access(&self, user_id: &UUID) -> bool;

    /// Executes the proposal.
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError>;

    /// The post create hook is called after the proposal is created and can be used
    /// for additional processing (e.g. sending notifications)
    ///
    /// Should panic if the post create hook fails to rollback state changes.
    async fn on_created(&self) {
        // noop by default
    }

    /// Creates a new proposal for the operation but does not save it.
    fn new_proposal(
        id: Uuid,
        proposed_by_user: UUID,
        title: Option<String>,
        summary: Option<String>,
        execution_plan: Option<ProposalExecutionPlan>,
        operation: ProposalOperationInput,
    ) -> Result<Proposal, ProposalError>
    where
        Self: Sized;
}

fn create_proposal<Handler: ProposalHandler>(
    proposal_id: Uuid,
    proposed_by_user: UUID,
    input: CreateProposalInput,
) -> Result<Proposal, ProposalError> {
    Handler::new_proposal(
        proposal_id,
        proposed_by_user,
        input.title,
        input.summary,
        input.execution_plan.map(Into::into),
        input.operation,
    )
}

#[derive(Debug)]
pub struct ProposalFactory {}

impl ProposalFactory {
    pub async fn create_proposal(
        proposed_by_user: UUID,
        input: CreateProposalInput,
    ) -> Result<Proposal, ProposalError> {
        let id = generate_uuid_v4().await;

        match input.operation {
            ProposalOperationInput::Transfer(_) => {
                create_proposal::<TransferProposal>(id, proposed_by_user, input)
            }
            ProposalOperationInput::EditAccount(_) => {
                create_proposal::<EditAccountProposal>(id, proposed_by_user, input)
            }
            ProposalOperationInput::AddAccount(_) => {
                create_proposal::<AddAccountProposal>(id, proposed_by_user, input)
            }
            _ => trap(&format!(
                "Unsupported proposal operation: {:?}",
                input.operation
            )),
        }
    }

    pub fn build_handler<'proposal>(
        proposal: &'proposal Proposal,
    ) -> Box<dyn ProposalHandler + 'proposal> {
        match &proposal.operation {
            ProposalOperation::Transfer(_) => Box::new(TransferProposal::new(proposal)),
            ProposalOperation::EditAccount(_) => Box::new(EditAccountProposal::new(proposal)),
            ProposalOperation::AddAccount(_) => Box::new(AddAccountProposal::new(proposal)),
            _ => trap(&format!(
                "Unsupported proposal operation: {:?}",
                proposal.operation
            )),
        }
    }
}
