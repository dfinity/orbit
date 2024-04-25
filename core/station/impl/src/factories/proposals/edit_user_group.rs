use super::{Create, Execute, ProposalExecuteStage};
use crate::{
    errors::{ProposalError, ProposalExecuteError},
    models::{EditUserGroupOperation, Proposal, ProposalExecutionPlan, ProposalOperation},
    services::USER_GROUP_SERVICE,
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;

pub struct EditUserGroupProposalCreate {}

impl Create<station_api::EditUserGroupOperationInput> for EditUserGroupProposalCreate {
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: station_api::CreateProposalInput,
        operation_input: station_api::EditUserGroupOperationInput,
    ) -> Result<Proposal, ProposalError> {
        let proposal = Proposal::new(
            proposal_id,
            proposed_by_user,
            Proposal::default_expiration_dt_ns(),
            ProposalOperation::EditUserGroup(operation_input.into()),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(ProposalExecutionPlan::Immediate),
            input.title.unwrap_or_else(|| "User group edit".to_string()),
            input.summary,
        );

        Ok(proposal)
    }
}

pub struct EditUserGroupProposalExecute<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o EditUserGroupOperation,
}

impl<'p, 'o> EditUserGroupProposalExecute<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o EditUserGroupOperation) -> Self {
        Self {
            proposal,
            operation,
        }
    }
}

#[async_trait]
impl Execute for EditUserGroupProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        USER_GROUP_SERVICE
            .edit(self.operation.input.clone())
            .await
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to edit user group: {}", e),
            })?;

        Ok(ProposalExecuteStage::Completed(
            self.proposal.operation.clone(),
        ))
    }
}
