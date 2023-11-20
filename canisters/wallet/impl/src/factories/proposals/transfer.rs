use super::ProposalProcessor;
use crate::{
    core::{generate_uuid_v4, ic_cdk::api::trap},
    errors::{ProposalError, ProposalExecuteError},
    factories::blockchains::BlockchainApiFactory,
    mappers::HelperMapper,
    models::{
        Account, ApprovalThresholdPolicy, NotificationType, Policy, PolicyStatus, Proposal,
        ProposalExecutionPlan, ProposalOperation, ProposalVoteStatus, Transfer, TransferOperation,
        TransferProposalCreatedNotification,
    },
    repositories::{AccountRepository, TransferRepository},
    services::NotificationService,
};
use async_trait::async_trait;
use ic_canister_core::model::ModelValidator;
use ic_canister_core::repository::Repository;
use ic_canister_core::types::UUID;
use uuid::Uuid;
use wallet_api::ProposalOperationInput;

#[derive(Debug)]
pub struct TransferProposalProcessor<'proposal> {
    transfer_repository: TransferRepository,
    account_repository: AccountRepository,
    proposal: &'proposal Proposal,
    notification_service: NotificationService,
}

impl<'proposal> TransferProposalProcessor<'proposal> {
    pub fn new(proposal: &'proposal Proposal) -> Self {
        Self {
            proposal,
            transfer_repository: TransferRepository::default(),
            account_repository: AccountRepository::default(),
            notification_service: NotificationService::default(),
        }
    }

    fn unwrap_operation(&self) -> &TransferOperation {
        match self.proposal.operation {
            ProposalOperation::Transfer(ref ctx) => ctx,
            _ => trap("Invalid proposal operation for processor"),
        }
    }

    fn get_account(&self) -> Account {
        let ctx = self.unwrap_operation();

        self.account_repository
            .get(&Account::key(ctx.from_account_id))
            .unwrap_or_else(|| {
                trap(&format!(
                    "Account not found: {}",
                    Uuid::from_bytes(ctx.from_account_id).hyphenated()
                ))
            })
    }
}

#[async_trait]
impl<'proposal> ProposalProcessor for TransferProposalProcessor<'proposal> {
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
        let input = self.unwrap_operation();
        let transfer_id = generate_uuid_v4().await;
        let account = self.get_account();

        let blockchain_api = BlockchainApiFactory::build(&account.blockchain, &account.standard)
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to build blockchain api: {}", e),
            })?;
        let fee = match &input.fee {
            Some(fee) => fee.clone(),
            None => {
                let transaction_fee =
                    blockchain_api
                        .transaction_fee(&account)
                        .await
                        .map_err(|e| ProposalExecuteError::Failed {
                            reason: format!("Failed to build blockchain api: {}", e),
                        })?;

                candid::Nat(transaction_fee.fee)
            }
        };
        let transfer = Transfer::new(
            *transfer_id.as_bytes(),
            self.proposal.proposed_by.expect("Proposer not found"),
            input.from_account_id,
            input.to.clone(),
            input.metadata.clone(),
            input.amount.clone(),
            fee,
            input.network.clone(),
        );

        transfer
            .validate()
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to validate transfer: {}", e),
            })?;

        self.transfer_repository
            .insert(transfer.to_key(), transfer.to_owned());

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
                        NotificationType::TransferProposalCreated(
                            TransferProposalCreatedNotification {
                                account_id: account.id,
                                proposal_id: self.proposal.id,
                            },
                        ),
                        None,
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
            ProposalOperationInput::Transfer(operation) => {
                let from_account_id =
                    HelperMapper::to_uuid(operation.from_account_id).map_err(|e| {
                        ProposalError::ValidationError {
                            info: format!("Invalid from_account_id: {}", e),
                        }
                    })?;
                let proposal = Proposal::new(
                    id,
                    proposed_by_user,
                    Proposal::default_expiration_dt_ns(),
                    ProposalOperation::Transfer(TransferOperation {
                        from_account_id: *from_account_id.as_bytes(),
                        to: operation.to,
                        amount: operation.amount,
                        fee: operation.fee,
                        // todo: add metadata mapping
                        metadata: vec![],
                        // todo: add network mapping
                        network: match operation.network {
                            Some(network) => network.id,
                            None => "mainnet".to_string(),
                        },
                    }),
                    execution_plan.unwrap_or(ProposalExecutionPlan::Immediate),
                    title.unwrap_or_else(|| "Transfer".to_string()),
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
