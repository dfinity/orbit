use super::{Create, Execute, RequestExecuteStage};
use crate::{
    core::generate_uuid_v4,
    errors::{RequestError, RequestExecuteError},
    factories::blockchains::BlockchainApiFactory,
    mappers::HelperMapper,
    models::{
        Account, Metadata, Request, RequestExecutionPlan, RequestOperation, Transfer,
        TransferOperation, TransferOperationInput,
    },
    repositories::ACCOUNT_REPOSITORY,
    services::TransferService,
};
use async_trait::async_trait;
use orbit_essentials::model::ModelValidator;
use orbit_essentials::repository::Repository;
use orbit_essentials::types::UUID;
use uuid::Uuid;

fn get_account(from_account_id: &UUID) -> Option<Account> {
    ACCOUNT_REPOSITORY.get(&Account::key(*from_account_id))
}

pub struct TransferRequestCreate {}

#[async_trait]
impl Create<station_api::TransferOperationInput> for TransferRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::TransferOperationInput,
    ) -> Result<Request, RequestError> {
        let from_account_id =
            HelperMapper::to_uuid(operation_input.from_account_id).map_err(|e| {
                RequestError::ValidationError {
                    info: format!("Invalid from_account_id: {}", e),
                }
            })?;
        let request = Request::new(
            request_id,
            requested_by_user,
            Request::default_expiration_dt_ns(),
            RequestOperation::Transfer(TransferOperation {
                transfer_id: None,
                fee: None,
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
                .unwrap_or(RequestExecutionPlan::Immediate),
            input.title.unwrap_or_else(|| "Transfer".to_string()),
            input.summary,
        );

        request.validate()?;

        Ok(request)
    }
}

pub struct TransferRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o TransferOperation,
    transfer_service: TransferService,
}

impl<'p, 'o> TransferRequestExecute<'p, 'o> {
    pub fn new(request: &'p Request, operation: &'o TransferOperation) -> Self {
        Self {
            request,
            operation,
            transfer_service: TransferService::default(),
        }
    }
}

#[async_trait]
impl Execute for TransferRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        let account = get_account(&self.operation.input.from_account_id).ok_or(
            RequestExecuteError::Failed {
                reason: format!(
                    "Account {} does not exist.",
                    Uuid::from_bytes(self.operation.input.from_account_id).hyphenated()
                ),
            },
        )?;

        let blockchain_api = BlockchainApiFactory::build(&account.blockchain, &account.standard)
            .map_err(|e| RequestExecuteError::Failed {
                reason: format!("Failed to build blockchain api: {}", e),
            })?;
        let fee = match &self.operation.input.fee {
            Some(fee) => fee.clone(),
            None => {
                let transaction_fee =
                    blockchain_api
                        .transaction_fee(&account)
                        .await
                        .map_err(|e| RequestExecuteError::Failed {
                            reason: format!("Failed to fetch transaction fee: {}", e),
                        })?;

                candid::Nat(transaction_fee.fee)
            }
        };

        self.transfer_service
            .add_transfer(Transfer::new(
                self.request.id,
                *generate_uuid_v4().await.as_bytes(),
                self.request.requested_by,
                self.operation.input.from_account_id,
                self.operation.input.to.clone(),
                self.operation.input.metadata.clone(),
                self.operation.input.amount.clone(),
                fee,
                self.operation.input.network.clone(),
            ))
            .map_err(|e| RequestExecuteError::Failed {
                reason: format!("Failed to validate transfer: {}", e),
            })?;

        Ok(RequestExecuteStage::Processing(
            self.request.operation.clone(),
        ))
    }
}
