use std::str::FromStr;

use super::{Create, Execute, RequestExecuteStage};
use crate::{
    core::generate_uuid_v4,
    errors::{RequestError, RequestExecuteError},
    factories::blockchains::BlockchainApiFactory,
    mappers::HelperMapper,
    models::{
        Metadata, Request, RequestOperation, TokenStandard, Transfer, TransferOperation,
        TransferOperationInput,
    },
    repositories::ASSET_REPOSITORY,
    services::TransferService,
};
use async_trait::async_trait;
use orbit_essentials::model::ModelValidator;
use orbit_essentials::repository::Repository;
use orbit_essentials::types::UUID;
use uuid::Uuid;

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

        let from_asset_id = HelperMapper::to_uuid(operation_input.from_asset_id.clone())
            .map_err(|e| RequestError::ValidationError {
                info: format!("Invalid from_asset_id: {}", e),
            })?
            .as_bytes()
            .to_owned();

        let asset = ASSET_REPOSITORY
            .get(&from_asset_id)
            .ok_or(RequestError::ValidationError {
                info: format!("Asset {} does not exist.", operation_input.from_asset_id),
            })?;

        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::Transfer(TransferOperation {
                transfer_id: None,
                fee: None,
                asset,
                input: TransferOperationInput {
                    from_account_id: *from_account_id.as_bytes(),
                    from_asset_id,
                    with_standard: TokenStandard::from_str(&operation_input.with_standard)
                        .map_err(|_| RequestError::ValidationError {
                            info: "Invalid with_standard.".to_owned(),
                        })?,
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
            "Transfer".to_string(),
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
        let asset = ASSET_REPOSITORY
            .get(&self.operation.input.from_asset_id)
            .ok_or(RequestExecuteError::Failed {
                reason: format!(
                    "Asset {} does not exist.",
                    Uuid::from_bytes(self.operation.input.from_asset_id).hyphenated()
                ),
            })?;

        let blockchain_api = BlockchainApiFactory::build(&asset.blockchain).map_err(|e| {
            RequestExecuteError::Failed {
                reason: format!("Failed to build blockchain api: {}", e),
            }
        })?;
        let fee = match &self.operation.input.fee {
            Some(fee) => fee.clone(),
            None => {
                let transaction_fee = blockchain_api
                    .transaction_fee(&asset, self.operation.input.with_standard.clone())
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
                self.operation.input.from_asset_id,
                self.operation.input.with_standard.clone(),
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
