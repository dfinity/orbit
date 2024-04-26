use super::{Create, Execute, ProposalExecuteStage};
use crate::{
    errors::{ProposalError, ProposalExecuteError},
    models::{AddAccountOperation, Proposal, ProposalExecutionPlan, ProposalOperation},
    services::AccountService,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

pub struct AddAccountProposalCreate {}

impl Create<station_api::AddAccountOperationInput> for AddAccountProposalCreate {
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: station_api::CreateProposalInput,
        operation_input: station_api::AddAccountOperationInput,
    ) -> Result<Proposal, ProposalError> {
        let proposal = Proposal::new(
            proposal_id,
            proposed_by_user,
            Proposal::default_expiration_dt_ns(),
            ProposalOperation::AddAccount(AddAccountOperation {
                account_id: None,
                input: operation_input.into(),
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(ProposalExecutionPlan::Immediate),
            input
                .title
                .unwrap_or_else(|| "Account creation".to_string()),
            input.summary,
        );

        Ok(proposal)
    }
}

pub struct AddAccountProposalExecute<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o AddAccountOperation,
    account_service: AccountService,
}

impl<'p, 'o> AddAccountProposalExecute<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o AddAccountOperation) -> Self {
        Self {
            proposal,
            operation,
            account_service: AccountService::default(),
        }
    }
}

#[async_trait]
impl Execute for AddAccountProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        let account = self
            .account_service
            .create_account(self.operation.input.to_owned())
            .await
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to create account: {}", e),
            })?;

        let mut operation = self.proposal.operation.clone();

        if let ProposalOperation::AddAccount(ref mut operation) = operation {
            operation.account_id = Some(account.id);
        }

        Ok(ProposalExecuteStage::Completed(operation))
    }
}
