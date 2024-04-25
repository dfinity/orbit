use super::{Create, Execute, ProposalExecuteStage};
use crate::{
    errors::{ProposalError, ProposalExecuteError},
    models::{AddUserGroupOperation, Proposal, ProposalExecutionPlan, ProposalOperation},
    services::USER_GROUP_SERVICE,
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;

pub struct AddUserGroupProposalCreate {}

impl Create<station_api::AddUserGroupOperationInput> for AddUserGroupProposalCreate {
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: station_api::CreateProposalInput,
        operation_input: station_api::AddUserGroupOperationInput,
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
