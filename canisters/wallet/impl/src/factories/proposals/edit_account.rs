use super::{Create, CreateHook, Execute, ProposalExecuteStage};
use crate::{
    core::ic_cdk::api::trap,
    errors::{ProposalError, ProposalExecuteError},
    mappers::HelperMapper,
    models::{
        Account, EditAccountOperation, EditAccountOperationInput, NotificationType, Proposal,
        ProposalExecutionPlan, ProposalOperation,
    },
    repositories::ACCOUNT_REPOSITORY,
    services::NotificationService,
};
use async_trait::async_trait;
use ic_canister_core::model::ModelValidator;
use ic_canister_core::repository::Repository;
use ic_canister_core::types::UUID;
use uuid::Uuid;

pub struct EditAccountProposalCreate {}

impl Create<wallet_api::EditAccountOperationInput> for EditAccountProposalCreate {
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: wallet_api::CreateProposalInput,
        operation_input: wallet_api::EditAccountOperationInput,
    ) -> Result<Proposal, ProposalError> {
        let from_account_id = HelperMapper::to_uuid(operation_input.account_id).map_err(|e| {
            ProposalError::ValidationError {
                info: format!("Invalid from_account_id: {}", e),
            }
        })?;

        let proposal = Proposal::new(
            proposal_id,
            proposed_by_user,
            Proposal::default_expiration_dt_ns(),
            ProposalOperation::EditAccount(EditAccountOperation {
                input: EditAccountOperationInput {
                    account_id: *from_account_id.as_bytes(),
                    owners: match operation_input.owners {
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
                    policies: operation_input.policies.map(|policies| {
                        policies
                            .iter()
                            .map(|policy| policy.clone().into())
                            .collect()
                    }),
                    name: operation_input.name,
                },
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(ProposalExecutionPlan::Immediate),
            input.title.unwrap_or_else(|| "Account edit".to_string()),
            input.summary,
        );

        Ok(proposal)
    }
}

pub struct EditAccountProposalCreateHook<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o EditAccountOperation,
    notification_service: NotificationService,
}

impl<'p, 'o> EditAccountProposalCreateHook<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o EditAccountOperation) -> Self {
        Self {
            proposal,
            operation,
            notification_service: NotificationService::default(),
        }
    }
}

#[async_trait]
impl CreateHook for EditAccountProposalCreateHook<'_, '_> {
    async fn on_created(&self) {
        let account = ACCOUNT_REPOSITORY
            .get(&Account::key(self.operation.input.account_id))
            .unwrap_or_else(|| {
                trap(&format!(
                    "Account not found: {}",
                    Uuid::from_bytes(self.operation.input.account_id).hyphenated()
                ))
            });

        for owner in account.owners {
            let should_send = self.proposal.proposed_by != owner;

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
}

pub struct EditAccountProposalExecute<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o EditAccountOperation,
}

impl<'p, 'o> EditAccountProposalExecute<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o EditAccountOperation) -> Self {
        Self {
            proposal,
            operation,
        }
    }
}

#[async_trait]
impl Execute for EditAccountProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        let mut account = ACCOUNT_REPOSITORY
            .get(&Account::key(self.operation.input.account_id))
            .unwrap_or_else(|| {
                trap(&format!(
                    "Account not found: {}",
                    Uuid::from_bytes(self.operation.input.account_id).hyphenated()
                ))
            });

        if let Some(name) = &self.operation.input.name {
            account.name = name.clone();
        }

        if let Some(owners) = &self.operation.input.owners {
            account.owners = owners.clone();
        }

        if let Some(policies) = &self.operation.input.policies {
            account.policies = policies.clone();
        }

        account
            .validate()
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to validate account: {}", e),
            })?;

        ACCOUNT_REPOSITORY.insert(account.to_key(), account.to_owned());

        Ok(ProposalExecuteStage::Completed(
            self.proposal.operation.clone(),
        ))
    }
}
