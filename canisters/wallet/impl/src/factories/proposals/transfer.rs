use super::{Create, CreateHook, Evaluate, Execute, ProposalExecuteStage, Validate};
use crate::{
    core::{generate_uuid_v4, ic_cdk::api::trap},
    errors::{ProposalError, ProposalEvaluateError, ProposalExecuteError},
    factories::blockchains::BlockchainApiFactory,
    mappers::HelperMapper,
    models::{
        Account, ApprovalThresholdPolicy, EvaluationStatus, NotificationType, Policy, Proposal,
        ProposalExecutionPlan, ProposalOperation, ProposalVoteStatus, Transfer, TransferOperation,
        TransferOperationInput, TransferProposalCreatedNotification,
    },
    repositories::{TransferRepository, ACCOUNT_REPOSITORY},
    services::NotificationService,
};
use async_trait::async_trait;
use ic_canister_core::model::ModelValidator;
use ic_canister_core::repository::Repository;
use ic_canister_core::types::UUID;
use uuid::Uuid;

fn get_account(from_account_id: &UUID) -> Account {
    ACCOUNT_REPOSITORY
        .get(&Account::key(*from_account_id))
        .unwrap_or_else(|| {
            trap(&format!(
                "Account not found: {}",
                Uuid::from_bytes(*from_account_id).hyphenated()
            ))
        })
}

pub struct TransferProposalCreate {}

impl Create<wallet_api::TransferOperationInput> for TransferProposalCreate {
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: wallet_api::CreateProposalInput,
        operation_input: wallet_api::TransferOperationInput,
    ) -> Result<Proposal, ProposalError> {
        let from_account_id =
            HelperMapper::to_uuid(operation_input.from_account_id).map_err(|e| {
                ProposalError::ValidationError {
                    info: format!("Invalid from_account_id: {}", e),
                }
            })?;
        let proposal = Proposal::new(
            proposal_id,
            proposed_by_user,
            Proposal::default_expiration_dt_ns(),
            ProposalOperation::Transfer(TransferOperation {
                transfer_id: None,
                input: TransferOperationInput {
                    from_account_id: *from_account_id.as_bytes(),
                    to: operation_input.to,
                    amount: operation_input.amount,
                    fee: operation_input.fee,
                    // todo: add metadata mapping
                    metadata: vec![],
                    // todo: add network mapping
                    network: match operation_input.network {
                        Some(network) => network.id,
                        None => "mainnet".to_string(),
                    },
                },
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(ProposalExecutionPlan::Immediate),
            input.title.unwrap_or_else(|| "Transfer".to_string()),
            input.summary,
        );

        proposal.validate()?;

        Ok(proposal)
    }
}

pub struct TransferProposalCreateHook<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o TransferOperation,
    notification_service: NotificationService,
}

impl<'p, 'o> TransferProposalCreateHook<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o TransferOperation) -> Self {
        Self {
            proposal,
            operation,
            notification_service: NotificationService::default(),
        }
    }
}

#[async_trait]
impl CreateHook for TransferProposalCreateHook<'_, '_> {
    async fn on_created(&self) {
        let account = get_account(&self.operation.input.from_account_id);

        for owner in account.owners {
            let should_send = self.proposal.proposed_by != owner;

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
}

pub struct TransferProposalValidate<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o TransferOperation,
}

impl<'p, 'o> TransferProposalValidate<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o TransferOperation) -> Self {
        Self {
            proposal,
            operation,
        }
    }
}

impl Validate for TransferProposalValidate<'_, '_> {
    fn can_vote(&self, user_id: &UUID) -> bool {
        let account = get_account(&self.operation.input.from_account_id);
        let should_vote = account.policies.iter().any(|policy| match policy {
            Policy::ApprovalThreshold(_) => true,
        });

        should_vote && account.owners.contains(user_id)
    }

    fn can_view(&self, user_id: &UUID) -> bool {
        let account = get_account(&self.operation.input.from_account_id);

        self.can_vote(user_id)
            || account.owners.contains(user_id)
            || self.proposal.voters().contains(user_id)
            || self.proposal.proposed_by == *user_id
    }
}

pub struct TransferProposalEvaluate<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o TransferOperation,
}

impl<'p, 'o> TransferProposalEvaluate<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o TransferOperation) -> Self {
        Self {
            proposal,
            operation,
        }
    }
}

#[async_trait]
impl Evaluate for TransferProposalEvaluate<'_, '_> {
    async fn evaluate(&self) -> Result<EvaluationStatus, ProposalEvaluateError> {
        let account = get_account(&self.operation.input.from_account_id);
        let mut policy_list = account
            .policies
            .into_iter()
            .map(|policy| (policy, EvaluationStatus::Pending))
            .collect::<Vec<(Policy, EvaluationStatus)>>();
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
                            *status = EvaluationStatus::Adopted;
                        } else if !can_still_be_approved {
                            *status = EvaluationStatus::Rejected;
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
                            *status = EvaluationStatus::Adopted;
                        } else if !can_still_be_approved {
                            *status = EvaluationStatus::Rejected;
                        }
                    }
                },
            }
        }

        if policy_list
            .iter()
            .all(|(_, status)| status == &EvaluationStatus::Adopted)
        {
            return Ok(EvaluationStatus::Adopted);
        } else if policy_list
            .iter()
            .any(|(_, status)| status == &EvaluationStatus::Rejected)
        {
            return Ok(EvaluationStatus::Rejected);
        }

        Ok(EvaluationStatus::Pending)
    }
}

pub struct TransferProposalExecute<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o TransferOperation,
    transfer_repository: TransferRepository,
}

impl<'p, 'o> TransferProposalExecute<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o TransferOperation) -> Self {
        Self {
            proposal,
            operation,
            transfer_repository: TransferRepository::default(),
        }
    }
}

#[async_trait]
impl Execute for TransferProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        let transfer_id = generate_uuid_v4().await;
        let account = get_account(&self.operation.input.from_account_id);

        let blockchain_api = BlockchainApiFactory::build(&account.blockchain, &account.standard)
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to build blockchain api: {}", e),
            })?;
        let fee = match &self.operation.input.fee {
            Some(fee) => fee.clone(),
            None => {
                let transaction_fee =
                    blockchain_api
                        .transaction_fee(&account)
                        .await
                        .map_err(|e| ProposalExecuteError::Failed {
                            reason: format!("Failed to fetch transaction fee: {}", e),
                        })?;

                candid::Nat(transaction_fee.fee)
            }
        };
        let transfer = Transfer::new(
            self.proposal.id,
            *transfer_id.as_bytes(),
            self.proposal.proposed_by,
            self.operation.input.from_account_id,
            self.operation.input.to.clone(),
            self.operation.input.metadata.clone(),
            self.operation.input.amount.clone(),
            fee,
            self.operation.input.network.clone(),
        );

        transfer
            .validate()
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to validate transfer: {}", e),
            })?;

        self.transfer_repository
            .insert(transfer.to_key(), transfer.to_owned());

        Ok(ProposalExecuteStage::Processing(
            self.proposal.operation.clone(),
        ))
    }
}
