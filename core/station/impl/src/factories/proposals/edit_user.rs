use super::{Create, Execute, ProposalExecuteStage};
use crate::{
    errors::{ProposalError, ProposalExecuteError},
    models::{EditUserOperation, Proposal, ProposalExecutionPlan, ProposalOperation},
    services::USER_SERVICE,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

pub struct EditUserProposalCreate {}

impl Create<station_api::EditUserOperationInput> for EditUserProposalCreate {
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: station_api::CreateProposalInput,
        operation_input: station_api::EditUserOperationInput,
    ) -> Result<Proposal, ProposalError> {
        let proposal = Proposal::new(
            proposal_id,
            proposed_by_user,
            Proposal::default_expiration_dt_ns(),
            ProposalOperation::EditUser(EditUserOperation {
                input: operation_input.into(),
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(ProposalExecutionPlan::Immediate),
            input.title.unwrap_or_else(|| "User edit".to_string()),
            input.summary,
        );

        Ok(proposal)
    }
}

pub struct EditUserProposalExecute<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o EditUserOperation,
}

impl<'p, 'o> EditUserProposalExecute<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o EditUserOperation) -> Self {
        Self {
            proposal,
            operation,
        }
    }
}

#[async_trait]
impl Execute for EditUserProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        USER_SERVICE
            .edit_user(self.operation.input.clone())
            .await
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to edit user: {}", e),
            })?;

        Ok(ProposalExecuteStage::Completed(
            self.proposal.operation.clone(),
        ))
    }
}
