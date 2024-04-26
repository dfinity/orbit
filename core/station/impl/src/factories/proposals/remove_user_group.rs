use super::{Create, Execute, ProposalExecuteStage};
use crate::{
    errors::{ProposalError, ProposalExecuteError},
    models::{Proposal, ProposalExecutionPlan, ProposalOperation, RemoveUserGroupOperation},
    services::USER_GROUP_SERVICE,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

pub struct RemoveUserGroupProposalCreate {}

impl Create<station_api::RemoveUserGroupOperationInput> for RemoveUserGroupProposalCreate {
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: station_api::CreateProposalInput,
        operation_input: station_api::RemoveUserGroupOperationInput,
    ) -> Result<Proposal, ProposalError> {
        let proposal = Proposal::new(
            proposal_id,
            proposed_by_user,
            Proposal::default_expiration_dt_ns(),
            ProposalOperation::RemoveUserGroup(operation_input.into()),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(ProposalExecutionPlan::Immediate),
            input
                .title
                .unwrap_or_else(|| "User group removal".to_string()),
            input.summary,
        );

        Ok(proposal)
    }
}

pub struct RemoveUserGroupProposalExecute<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o RemoveUserGroupOperation,
}

impl<'p, 'o> RemoveUserGroupProposalExecute<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o RemoveUserGroupOperation) -> Self {
        Self {
            proposal,
            operation,
        }
    }
}

#[async_trait]
impl Execute for RemoveUserGroupProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        USER_GROUP_SERVICE
            .remove(&self.operation.input.user_group_id)
            .await
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to remove user group: {}", e),
            })?;

        Ok(ProposalExecuteStage::Completed(
            self.proposal.operation.clone(),
        ))
    }
}
