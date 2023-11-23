use super::{ProposalExecuteStage, ProposalProcessor};
use crate::{
    core::ic_cdk::api::trap,
    errors::{ProposalError, ProposalExecuteError},
    models::{
        AddAccountOperation, Policy, PolicyStatus, Proposal, ProposalExecutionPlan,
        ProposalOperation,
    },
    services::AccountService,
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;
use uuid::Uuid;
use wallet_api::ProposalOperationInput;

#[derive(Debug)]
pub struct AddAccountProposalProcessor<'proposal> {
    proposal: &'proposal Proposal,
    account_service: AccountService,
}

impl<'proposal> AddAccountProposalProcessor<'proposal> {
    pub fn new(proposal: &'proposal Proposal) -> Self {
        Self {
            proposal,
            account_service: AccountService::default(),
        }
    }

    fn unwrap_operation(&self) -> &AddAccountOperation {
        match self.proposal.operation {
            ProposalOperation::AddAccount(ref ctx) => ctx,
            _ => trap("Invalid proposal operation for processor"),
        }
    }
}

#[async_trait]
impl<'proposal> ProposalProcessor for AddAccountProposalProcessor<'proposal> {
    fn evaluate_policies(&self) -> Vec<(Policy, PolicyStatus)> {
        // TODO: Add policy evaluation once final policy design is ready

        Vec::new()
    }

    fn can_vote(&self, _user_id: &UUID) -> bool {
        // TODO: Add policy evaluation once final policy design is ready

        false
    }

    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        let input = self.unwrap_operation();

        let account = self
            .account_service
            .create_account(input.clone())
            .await
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to create account: {}", e),
            })?;

        let mut operation = self.proposal.operation.clone();

        if let ProposalOperation::AddAccount(ref mut ctx) = operation {
            ctx.id = Some(account.id);
        }

        Ok(ProposalExecuteStage::Completed(operation))
    }

    fn has_access(&self, user_id: &UUID) -> bool {
        self.proposal.users().contains(user_id)
    }

    async fn post_create(&self) {
        // TODO: Add once policy design is ready
    }

    fn new_proposal(
        id: Uuid,
        proposed_by_user: UUID,
        title: Option<String>,
        summary: Option<String>,
        execution_plan: Option<ProposalExecutionPlan>,
        operation: ProposalOperationInput,
    ) -> Result<Proposal, ProposalError> {
        match operation {
            ProposalOperationInput::AddAccount(input) => {
                let proposal = Proposal::new(
                    id,
                    proposed_by_user,
                    Proposal::default_expiration_dt_ns(),
                    ProposalOperation::AddAccount(AddAccountOperation::from(input)),
                    execution_plan.unwrap_or(ProposalExecutionPlan::Immediate),
                    title.unwrap_or_else(|| "Account creation".to_string()),
                    summary,
                );

                Ok(proposal)
            }
            _ => Err(ProposalError::ValidationError {
                info: "Invalid operation for proposal creation".to_string(),
            })?,
        }
    }
}
