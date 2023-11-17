use super::ProposalProcessor;
use crate::{
    core::ic_cdk::api::trap,
    errors::{ProposalError, ProposalExecuteError},
    mappers::HelperMapper,
    models::{
        Account, AccountEditOperation, ApprovalThresholdPolicy, NotificationType, Policy,
        PolicyStatus, Proposal, ProposalExecutionPlan, ProposalOperation, ProposalStatus,
        ProposalVoteStatus,
    },
    repositories::AccountRepository,
    services::NotificationService,
    transport::ProposalOperationInput,
};
use async_trait::async_trait;
use ic_canister_core::model::ModelValidator;
use ic_canister_core::repository::Repository;
use ic_canister_core::types::UUID;
use uuid::Uuid;

#[derive(Debug)]
pub struct AccountEditProposalProcessor<'proposal> {
    proposal: &'proposal Proposal,
    account_repository: AccountRepository,
    notification_service: NotificationService,
}

impl<'proposal> AccountEditProposalProcessor<'proposal> {
    pub fn new(proposal: &'proposal Proposal) -> Self {
        Self {
            proposal,
            account_repository: AccountRepository::default(),
            notification_service: NotificationService::default(),
        }
    }

    fn unwrap_operation(&self) -> &AccountEditOperation {
        match self.proposal.operation {
            ProposalOperation::AccountEdit(ref ctx) => ctx,
            _ => trap("Invalid proposal operation for processor"),
        }
    }

    fn get_account(&self) -> Account {
        let operation = self.unwrap_operation();

        self.account_repository
            .get(&Account::key(operation.account_id))
            .unwrap_or_else(|| {
                trap(&format!(
                    "Account not found: {}",
                    Uuid::from_bytes(operation.account_id).hyphenated()
                ))
            })
    }
}

#[async_trait]
impl<'proposal> ProposalProcessor for AccountEditProposalProcessor<'proposal> {
    fn evaluate_policies(&self) -> Vec<(Policy, PolicyStatus)> {
        let account = self.get_account();
        let mut policy_list = account
            .policies
            .into_iter()
            .map(|policy| (policy, PolicyStatus::Pending))
            .collect::<Vec<(Policy, PolicyStatus)>>();
        let total_approvals = self
            .proposal
            .votes
            .iter()
            .filter(|vote| vote.status == ProposalVoteStatus::Accepted)
            .count();
        let missing_votes = account
            .owners
            .iter()
            .filter(|owner| {
                !self
                    .proposal
                    .votes
                    .iter()
                    .any(|vote| vote.user_id == **owner)
            })
            .count();

        for (policy, status) in &mut policy_list {
            match policy {
                Policy::ApprovalThreshold(threshold) => match threshold {
                    ApprovalThresholdPolicy::FixedThreshold(min_approvals) => {
                        let can_still_be_approved =
                            total_approvals + missing_votes >= *min_approvals as usize;

                        if total_approvals >= *min_approvals as usize {
                            *status = PolicyStatus::Fulfilled;
                        } else if !can_still_be_approved {
                            *status = PolicyStatus::Failed;
                        }
                    }
                    ApprovalThresholdPolicy::VariableThreshold(percentage) => {
                        let min_approvals = ((account.owners.len() as f64
                            * (*percentage as f64 / 100.0))
                            .ceil() as u8)
                            .max(1);
                        let can_still_be_approved =
                            total_approvals + missing_votes >= min_approvals as usize;

                        if total_approvals >= min_approvals as usize {
                            *status = PolicyStatus::Fulfilled;
                        } else if !can_still_be_approved {
                            *status = PolicyStatus::Failed;
                        }
                    }
                },
            }
        }

        policy_list
    }

    fn can_vote(&self, user_id: &UUID) -> bool {
        let account = self.get_account();
        let should_vote = account.policies.iter().any(|policy| match policy {
            Policy::ApprovalThreshold(_) => true,
        });

        should_vote && account.owners.contains(user_id)
    }

    async fn execute(&self) -> Result<(), ProposalExecuteError> {
        if self.proposal.status != ProposalStatus::Adopted {
            Err(ProposalExecuteError::NotAdopted)?;
        }
        let input = self.unwrap_operation();
        let mut account = self.get_account();

        if let Some(name) = &input.name {
            account.name = Some(name.clone());
        }

        if let Some(owners) = &input.owners {
            account.owners = owners.clone();
        }

        if let Some(policies) = &input.policies {
            account.policies = policies.clone();
        }

        account
            .validate()
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to validate account: {}", e),
            })?;

        self.account_repository
            .insert(account.to_key(), account.to_owned());

        Ok(())
    }

    fn has_access(&self, user_id: &UUID) -> bool {
        let account = self.get_account();

        self.proposal.users().contains(user_id) || account.owners.contains(user_id)
    }

    async fn post_create(&self) {
        let account = self.get_account();

        for owner in account.owners {
            let should_send = !self.proposal.users().contains(&owner);

            if should_send {
                self.notification_service
                    .send_notification(
                        owner,
                        NotificationType::AccountProposalCreated(self.proposal.id, account.id),
                        Some((
                            "Account edit requested".to_string(),
                            "notification_account_edit_proposed_title".to_string(),
                        )),
                        None,
                    )
                    .await
                    .unwrap_or_else(|e| trap(&format!("Failed to send notification: {:?}", e)));
            }
        }
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
            ProposalOperationInput::AccountEdit(input) => {
                let from_account_id = HelperMapper::to_uuid(input.account_id).map_err(|e| {
                    ProposalError::ValidationError {
                        info: format!("Invalid from_account_id: {}", e),
                    }
                })?;

                let proposal = Proposal::new(
                    id,
                    proposed_by_user,
                    Proposal::default_expiration_dt_ns(),
                    ProposalOperation::AccountEdit(AccountEditOperation {
                        account_id: *from_account_id.as_bytes(),
                        owners: match input.owners {
                            Some(owners) => Some(
                                owners
                                    .into_iter()
                                    .map(|owner| {
                                        HelperMapper::to_uuid(owner)
                                            .map_err(|e| ProposalError::ValidationError {
                                                info: format!("Invalid owner: {}", e),
                                            })
                                            .map(|uuid| *uuid.as_bytes())
                                    })
                                    .collect::<Result<Vec<UUID>, _>>()?,
                            ),
                            None => None,
                        },
                        policies: input.policies.map(|policies| {
                            policies
                                .iter()
                                .map(|policy| policy.clone().into())
                                .collect()
                        }),
                        name: input.name,
                    }),
                    execution_plan.unwrap_or(ProposalExecutionPlan::Immediate),
                    title.unwrap_or_else(|| "Account edit".to_string()),
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
