use super::{Create, Execute, ProposalExecuteStage};
use crate::{
    core::generate_uuid_v4,
    errors::{ProposalError, ProposalExecuteError},
    factories::blockchains::BlockchainApiFactory,
    mappers::HelperMapper,
    models::{
        Account, Metadata, Proposal, ProposalExecutionPlan, ProposalOperation, Transfer,
        TransferOperation, TransferOperationInput,
    },
    repositories::ACCOUNT_REPOSITORY,
    services::TransferService,
};
use async_trait::async_trait;
use ic_canister_core::model::ModelValidator;
use ic_canister_core::repository::Repository;
use ic_canister_core::types::UUID;
use uuid::Uuid;

fn get_account(from_account_id: &UUID) -> Option<Account> {
    ACCOUNT_REPOSITORY.get(&Account::key(*from_account_id))
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
                    metadata: Metadata::default(),
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

pub struct TransferProposalExecute<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o TransferOperation,
    transfer_service: TransferService,
}

impl<'p, 'o> TransferProposalExecute<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o TransferOperation) -> Self {
        Self {
            proposal,
            operation,
            transfer_service: TransferService::default(),
        }
    }
}

#[async_trait]
impl Execute for TransferProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        let account = get_account(&self.operation.input.from_account_id).ok_or(
            ProposalExecuteError::Failed {
                reason: format!(
                    "Account {} does not exist.",
                    Uuid::from_bytes(self.operation.input.from_account_id).hyphenated()
                ),
            },
        )?;

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

        self.transfer_service
            .add_transfer(Transfer::new(
                self.proposal.id,
                *generate_uuid_v4().await.as_bytes(),
                self.proposal.proposed_by,
                self.operation.input.from_account_id,
                self.operation.input.to.clone(),
                self.operation.input.metadata.clone(),
                self.operation.input.amount.clone(),
                fee,
                self.operation.input.network.clone(),
            ))
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to validate transfer: {}", e),
            })?;

        Ok(ProposalExecuteStage::Processing(
            self.proposal.operation.clone(),
        ))
    }
}
