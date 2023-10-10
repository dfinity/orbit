use super::{AccountService, WalletService};
use crate::{
    core::{CallContext, WithCallContext},
    errors::{TransferError, WalletError},
    factories::blockchains::BlockchainApiFactory,
    factories::operations::OperationProcessorFactory,
    mappers::{HelperMapper, TransferMapper},
    models::{
        Operation, OperationCode, OperationDecision, OperationStatus, Transfer, TransferStatus,
        Wallet, WalletPolicy, OPERATION_METADATA_KEY_TRANSFER_ID, OPERATION_METADATA_KEY_WALLET_ID,
    },
    repositories::{OperationRepository, TransferRepository, WalletRepository},
    transport::{
        GetTransferInput, GetTransfersInput, GetWalletInput, ListWalletTransfersInput, TransferDTO,
        TransferInput, TransferListItemDTO,
    },
};
use candid::Nat;
use ic_canister_core::{
    api::ServiceResult,
    utils::{generate_uuid_v4, rfc3339_to_timestamp},
};
use ic_canister_core::{cdk::api::time, model::ModelValidator, repository::Repository};
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct TransferService {
    call_context: CallContext,
    account_service: AccountService,
    helper_mapper: HelperMapper,
    transfer_mapper: TransferMapper,
    wallet_repository: WalletRepository,
    wallet_service: WalletService,
    transfer_repository: TransferRepository,
    operation_repository: OperationRepository,
}

impl WithCallContext for TransferService {
    fn with_call_context(&mut self, call_context: CallContext) -> &Self {
        self.call_context = call_context.to_owned();
        self.account_service
            .with_call_context(call_context.to_owned());
        self.wallet_service
            .with_call_context(call_context.to_owned());

        self
    }
}

impl TransferService {
    pub fn create() -> Self {
        Default::default()
    }

    pub fn get_transfer_core(&self, input: GetTransferInput) -> ServiceResult<Transfer> {
        let transfer_key = Transfer::key(
            *self
                .helper_mapper
                .uuid_from_str(input.transfer_id.to_owned())?
                .as_bytes(),
        );
        let transfer = self.transfer_repository.get(&transfer_key).ok_or({
            TransferError::TransferNotFound {
                transfer_id: input.transfer_id.to_owned(),
            }
        })?;

        Ok(transfer)
    }

    pub fn check_transfer_access(&self, transfer: &Transfer) -> ServiceResult<()> {
        let caller_account = self
            .account_service
            .resolve_account(&self.call_context.caller())?;
        let wallet_key = Wallet::key(transfer.from_wallet);
        let wallet = self.wallet_repository.get(&wallet_key).ok_or({
            WalletError::WalletNotFound {
                id: Uuid::from_bytes(transfer.from_wallet)
                    .hyphenated()
                    .to_string(),
            }
        })?;
        let is_transfer_creator = caller_account.id == transfer.initiator_account;
        let is_wallet_owner = wallet.owners.contains(&caller_account.id);
        if !is_transfer_creator && !is_wallet_owner {
            Err(WalletError::Forbidden)?
        }

        Ok(())
    }

    pub async fn get_transfer(&self, input: GetTransferInput) -> ServiceResult<TransferDTO> {
        let transfer = self.get_transfer_core(input)?;
        self.check_transfer_access(&transfer)?;
        let dto = self.transfer_mapper.transfer_to_dto(transfer);
        Ok(dto)
    }

    pub async fn get_transfers(&self, input: GetTransfersInput) -> ServiceResult<Vec<TransferDTO>> {
        if input.transfer_ids.len() > 50 {
            Err(TransferError::GetTransfersBatchNotAllowed { max: 50 })?
        }

        let mut transfers = Vec::new();
        for transfer_id in input.transfer_ids.iter() {
            let transfer = self.get_transfer_core(GetTransferInput {
                transfer_id: transfer_id.to_owned(),
            })?;
            self.check_transfer_access(&transfer)?;
            transfers.push(self.transfer_mapper.transfer_to_dto(transfer));
        }

        Ok(transfers)
    }

    pub async fn create_transfer(&self, input: TransferInput) -> ServiceResult<TransferDTO> {
        // validate account is owner of wallet
        let caller_account = self
            .account_service
            .resolve_account(&self.call_context.caller())?;
        let wallet_id = self
            .helper_mapper
            .uuid_from_str(input.from_wallet_id.clone())?;
        let wallet_key = Wallet::key(*wallet_id.as_bytes());
        let wallet =
            self.wallet_repository
                .get(&wallet_key)
                .ok_or(WalletError::WalletNotFound {
                    id: wallet_id.hyphenated().to_string(),
                })?;
        let is_wallet_owner = wallet.owners.contains(&caller_account.id);
        if !is_wallet_owner {
            Err(WalletError::Forbidden)?
        }

        // create transfer
        let blockchain_api = BlockchainApiFactory::build(&wallet.blockchain, &wallet.standard)?;
        let default_fee = blockchain_api.transaction_fee(&wallet).await?;
        let transfer_id = generate_uuid_v4().await;

        let mut transfer = self.transfer_mapper.new_transfer_from_input(
            input,
            *transfer_id.as_bytes(),
            caller_account.id,
            Nat(default_fee.fee),
            blockchain_api.default_network(),
            Transfer::default_expiration_dt(),
        )?;
        transfer.make_policy_snapshot(&wallet);

        transfer.validate()?;

        // build operations
        let operations = self
            .build_operations_from_wallet_policies(&wallet, &transfer)
            .await;

        let has_approve_transfer_operation = operations
            .iter()
            .any(|operation| matches!(operation.code, OperationCode::ApproveTransfer));

        if !has_approve_transfer_operation {
            transfer.status = TransferStatus::Approved;
        }

        // save transfer to stable memory
        self.transfer_repository
            .insert(transfer.as_key(), transfer.to_owned());

        operations.iter().for_each(|operation| {
            self.operation_repository
                .insert(operation.as_key(), operation.to_owned());
        });

        let processor = OperationProcessorFactory::build(&OperationCode::ApproveTransfer);
        for operation in operations.iter() {
            processor
                .post_process(operation)
                .await
                .expect("Operation post processing failed");
        }

        let dto = self.transfer_mapper.transfer_to_dto(transfer);

        Ok(dto)
    }

    async fn build_operations_from_wallet_policies(
        &self,
        wallet: &Wallet,
        transfer: &Transfer,
    ) -> Vec<Operation> {
        let mut required_operations: Vec<Operation> = Vec::new();
        let wallet_id = Uuid::from_bytes(wallet.id).hyphenated().to_string();
        let transfer_id = Uuid::from_bytes(transfer.id).hyphenated().to_string();
        for policy in wallet.policies.iter() {
            match policy {
                WalletPolicy::ApprovalThreshold(_) => {
                    let operation_id = generate_uuid_v4().await;
                    let mut operation = Operation {
                        id: *operation_id.as_bytes(),
                        code: OperationCode::ApproveTransfer,
                        status: OperationStatus::Pending,
                        created_timestamp: time(),
                        originator_account_id: Some(transfer.initiator_account),
                        metadata: vec![
                            (
                                OPERATION_METADATA_KEY_TRANSFER_ID.to_owned(),
                                transfer_id.to_owned(),
                            ),
                            (
                                OPERATION_METADATA_KEY_WALLET_ID.to_owned(),
                                wallet_id.to_owned(),
                            ),
                        ],
                        last_modification_timestamp: time(),
                        decisions: Vec::new(),
                    };

                    for owner in wallet.owners.iter() {
                        operation.decisions.push(OperationDecision {
                            account_id: *owner,
                            status: match transfer.initiator_account == *owner {
                                true => OperationStatus::Adopted,
                                false => OperationStatus::Pending,
                            },
                            decided_dt: match transfer.initiator_account == *owner {
                                true => Some(time()),
                                false => None,
                            },
                            last_modification_timestamp: time(),
                            read: transfer.initiator_account == *owner,
                            status_reason: None,
                        });
                    }

                    required_operations.push(operation.to_owned());
                }
            }
        }

        required_operations
    }

    pub async fn list_wallet_transfers(
        &self,
        input: ListWalletTransfersInput,
    ) -> ServiceResult<Vec<TransferListItemDTO>> {
        let wallet = self
            .wallet_service
            .get_wallet_core(GetWalletInput {
                wallet_id: input.wallet_id,
            })
            .await?;

        let transfers = self.transfer_repository.find_by_wallet(
            wallet.id,
            input.from_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
            input.to_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
            input.status,
        );

        let dtos: Vec<TransferListItemDTO> = transfers
            .iter()
            .map(|transfer| {
                self.transfer_mapper
                    .transfer_to_list_item_dto(transfer.to_owned())
            })
            .collect();

        Ok(dtos)
    }
}
