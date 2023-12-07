use super::{Create, CreateHook, Execute, ProposalExecuteStage, Validate};
use crate::{
    errors::{ProposalError, ProposalExecuteError},
    models::{AddAccountOperation, Proposal, ProposalExecutionPlan, ProposalOperation},
    services::AccountService,
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;

pub struct AddAccountProposalCreate {}

impl Create<wallet_api::AddAccountOperationInput> for AddAccountProposalCreate {
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: wallet_api::CreateProposalInput,
        operation_input: wallet_api::AddAccountOperationInput,
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

pub struct AddAccountProposalCreateHook<'p, 'o> {
    _proposal: &'p Proposal,
    _operation: &'o AddAccountOperation,
}

impl<'p, 'o> AddAccountProposalCreateHook<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o AddAccountOperation) -> Self {
        Self {
            _proposal: proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl CreateHook for AddAccountProposalCreateHook<'_, '_> {
    async fn on_created(&self) {
        // TODO: Add once policy design is ready
    }
}

pub struct AddAccountProposalValidate<'p, 'o> {
    proposal: &'p Proposal,
    _operation: &'o AddAccountOperation,
}

impl<'p, 'o> AddAccountProposalValidate<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o AddAccountOperation) -> Self {
        Self {
            proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl Validate for AddAccountProposalValidate<'_, '_> {
    fn can_vote(&self, _user_id: &UUID) -> bool {
        // TODO: Add once final policy design is ready

        false
    }

    fn can_view(&self, user_id: &UUID) -> bool {
        self.can_vote(user_id)
            || self.proposal.voters().contains(user_id)
            || self.proposal.proposed_by == *user_id
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
            .create_account(self.operation.clone())
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
