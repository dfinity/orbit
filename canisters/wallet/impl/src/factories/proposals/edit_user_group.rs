use super::{ProposalExecuteStage, ProposalHandler};
use crate::{
    core::ic_cdk::api::trap,
    errors::{ProposalError, ProposalExecuteError},
    models::{
        EditUserGroupOperation, Policy, PolicyStatus, Proposal, ProposalExecutionPlan,
        ProposalOperation,
    },
    services::UserGroupService,
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;
use uuid::Uuid;
use wallet_api::ProposalOperationInput;

#[derive(Debug)]
pub struct EditUserGroupProposal<'proposal> {
    proposal: &'proposal Proposal,
    user_group_service: UserGroupService,
}

impl<'proposal> EditUserGroupProposal<'proposal> {
    pub fn new(proposal: &'proposal Proposal) -> Self {
        Self {
            proposal,
            user_group_service: UserGroupService::default(),
        }
    }

    fn unwrap_operation(&self) -> &EditUserGroupOperation {
        match self.proposal.operation {
            ProposalOperation::EditUserGroup(ref ctx) => ctx,
            _ => trap("Invalid proposal operation for processor"),
        }
    }
}

#[async_trait]
impl<'proposal> ProposalHandler for EditUserGroupProposal<'proposal> {
    fn evaluate_policies(&self) -> Vec<(Policy, PolicyStatus)> {
        // TODO: Add policy evaluation once final policy design is ready

        Vec::new()
    }

    fn can_vote(&self, _user_id: &UUID) -> bool {
        // TODO: Add policy evaluation once final policy design is ready

        false
    }

    fn has_access(&self, user_id: &UUID) -> bool {
        // TODO: Add necessary access policies once final policy design is ready

        self.proposal.users().contains(user_id)
    }

    async fn on_created(&self) {
        // TODO: Add once policy design is ready
    }

    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        let operation = self.unwrap_operation();

        self.user_group_service
            .edit(operation.input.clone())
            .await
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to edit user group: {}", e),
            })?;

        Ok(ProposalExecuteStage::Completed(
            self.proposal.operation.clone(),
        ))
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
            ProposalOperationInput::EditUserGroup(input) => {
                let proposal = Proposal::new(
                    id,
                    proposed_by_user,
                    Proposal::default_expiration_dt_ns(),
                    ProposalOperation::EditUserGroup(input.into()),
                    execution_plan.unwrap_or(ProposalExecutionPlan::Immediate),
                    title.unwrap_or_else(|| "User group update".to_string()),
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
