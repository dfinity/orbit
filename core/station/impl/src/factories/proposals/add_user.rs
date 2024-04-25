use super::{Create, Execute, ProposalExecuteStage};
use crate::{
    errors::{ProposalError, ProposalExecuteError},
    models::{AddUserOperation, Proposal, ProposalExecutionPlan, ProposalOperation},
    services::USER_SERVICE,
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;

pub struct AddUserProposalCreate {}

impl Create<station_api::AddUserOperationInput> for AddUserProposalCreate {
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: station_api::CreateProposalInput,
        operation_input: station_api::AddUserOperationInput,
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
            .add_user(self.operation.input.clone())
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
