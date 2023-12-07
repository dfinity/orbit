use super::{Create, CreateHook, Execute, ProposalExecuteStage, Validate};
use crate::{
    core::ic_cdk::api::id as self_canister_id,
    core::CallContext,
    errors::{ProposalError, ProposalExecuteError},
    models::{AddUserOperation, Proposal, ProposalExecutionPlan, ProposalOperation},
    services::USER_SERVICE,
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;

pub struct AddUserProposalCreate {}

impl Create<wallet_api::AddUserOperationInput> for AddUserProposalCreate {
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: wallet_api::CreateProposalInput,
        operation_input: wallet_api::AddUserOperationInput,
    ) -> Result<Proposal, ProposalError> {
        let proposal = Proposal::new(
            proposal_id,
            proposed_by_user,
            Proposal::default_expiration_dt_ns(),
            ProposalOperation::AddUser(AddUserOperation {
                user_id: None,
                input: operation_input.into(),
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(ProposalExecutionPlan::Immediate),
            input.title.unwrap_or_else(|| "User creation".to_string()),
            input.summary,
        );

        Ok(proposal)
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

pub struct AddUserProposalExecute<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o AddUserOperation,
}

impl<'p, 'o> AddUserProposalExecute<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o AddUserOperation) -> Self {
        Self {
            proposal,
            operation,
        }
    }
}

#[async_trait]
impl Execute for AddUserProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        let user = USER_SERVICE
            .add_user(
                self.operation.input.clone(),
                &CallContext::new(self_canister_id()),
            )
            .await
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to create user: {}", e),
            })?;

        let mut operation = self.proposal.operation.clone();

        if let ProposalOperation::AddUser(ref mut operation) = operation {
            operation.user_id = Some(user.id);
        }

        Ok(ProposalExecuteStage::Completed(operation))
    }
}
