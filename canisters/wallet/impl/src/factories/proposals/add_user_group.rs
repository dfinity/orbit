use super::{Create, CreateHook, Execute, ProposalExecuteStage, Validate};
use crate::{
    errors::{ProposalError, ProposalExecuteError},
    models::{AddUserGroupOperation, Proposal, ProposalExecutionPlan, ProposalOperation},
    services::USER_GROUP_SERVICE,
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;

pub struct AddUserGroupProposalCreate {}

impl Create<wallet_api::AddUserGroupOperationInput> for AddUserGroupProposalCreate {
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: wallet_api::CreateProposalInput,
        operation_input: wallet_api::AddUserGroupOperationInput,
    ) -> Result<Proposal, ProposalError> {
        let proposal = Proposal::new(
            proposal_id,
            proposed_by_user,
            Proposal::default_expiration_dt_ns(),
            ProposalOperation::AddUserGroup(operation_input.into()),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(ProposalExecutionPlan::Immediate),
            input
                .title
                .unwrap_or_else(|| "User group creation".to_string()),
            input.summary,
        );

        Ok(proposal)
    }
}

pub struct AddUserGroupProposalCreateHook<'p, 'o> {
    _proposal: &'p Proposal,
    _operation: &'o AddUserGroupOperation,
}

impl<'p, 'o> AddUserGroupProposalCreateHook<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o AddUserGroupOperation) -> Self {
        Self {
            _proposal: proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl CreateHook for AddUserGroupProposalCreateHook<'_, '_> {
    async fn on_created(&self) {
        // TODO: Add once policy design is ready
    }
}

pub struct AddUserGroupProposalValidate<'p, 'o> {
    proposal: &'p Proposal,
    _operation: &'o AddUserGroupOperation,
}

impl<'p, 'o> AddUserGroupProposalValidate<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o AddUserGroupOperation) -> Self {
        Self {
            proposal,
            _operation: operation,
        }
    }
}

impl Validate for AddUserGroupProposalValidate<'_, '_> {
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

pub struct AddUserGroupProposalExecute<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o AddUserGroupOperation,
}

impl<'p, 'o> AddUserGroupProposalExecute<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o AddUserGroupOperation) -> Self {
        Self {
            proposal,
            operation,
        }
    }
}

#[async_trait]
impl Execute for AddUserGroupProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        let user_group = USER_GROUP_SERVICE
            .create(self.operation.input.clone())
            .await
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to create user group: {}", e),
            })?;

        let mut operation = self.proposal.operation.clone();

        if let ProposalOperation::AddUserGroup(ref mut op) = operation {
            op.user_group_id = Some(user_group.id);
        }

        Ok(ProposalExecuteStage::Completed(operation))
    }
}
