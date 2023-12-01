use super::{ProposalExecuteStage, ProposalHandler};
use crate::{
    core::ic_cdk::api::trap,
    errors::{ProposalError, ProposalExecuteError},
    models::{
        AddUserGroupOperation, Policy, PolicyStatus, Proposal, ProposalExecutionPlan,
        ProposalOperation,
    },
    services::UserGroupService,
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;
use uuid::Uuid;
use wallet_api::ProposalOperationInput;

#[derive(Debug)]
pub struct AddUserGroupProposalHandler<'proposal> {
    proposal: &'proposal Proposal,
    user_group_service: UserGroupService,
}

impl<'proposal> AddUserGroupProposalHandler<'proposal> {
    pub fn new(proposal: &'proposal Proposal) -> Self {
        Self {
            proposal,
            user_group_service: UserGroupService::default(),
        }
    }

    fn unwrap_operation(&self) -> &AddUserGroupOperation {
        match self.proposal.operation {
            ProposalOperation::AddUserGroup(ref ctx) => ctx,
            _ => trap("Invalid proposal operation for processor"),
        }
    }
}

#[async_trait]
impl<'proposal> ProposalHandler for AddUserGroupProposalHandler<'proposal> {
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

        let user_group = self
            .user_group_service
            .create(operation.input.clone())
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

    fn new_proposal(
        id: Uuid,
        proposed_by_user: UUID,
        title: Option<String>,
        summary: Option<String>,
        execution_plan: Option<ProposalExecutionPlan>,
        operation: ProposalOperationInput,
    ) -> Result<Proposal, ProposalError> {
        match operation {
            ProposalOperationInput::AddUserGroup(input) => {
                let proposal = Proposal::new(
                    id,
                    proposed_by_user,
                    Proposal::default_expiration_dt_ns(),
                    ProposalOperation::AddUserGroup(input.into()),
                    execution_plan.unwrap_or(ProposalExecutionPlan::Immediate),
                    title.unwrap_or_else(|| "User group creation".to_string()),
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
