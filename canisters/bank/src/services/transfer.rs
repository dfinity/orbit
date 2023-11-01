use super::{AccountService, WalletService};
use crate::{
    core::{generate_uuid_v4, ic_cdk::api::time, CallContext, WithCallContext},
    errors::{TransferError, WalletError},
    factories::blockchains::BlockchainApiFactory,
    factories::operations::OperationProcessorFactory,
    mappers::{HelperMapper, TransferMapper},
    models::{
        Operation, OperationCode, OperationDecision, OperationStatus, Transfer, TransferId,
        TransferStatus, Wallet, WalletPolicy, OPERATION_METADATA_KEY_TRANSFER_ID,
        OPERATION_METADATA_KEY_WALLET_ID,
    },
    repositories::{OperationRepository, TransferRepository, WalletRepository},
    transport::{ListWalletTransfersInput, TransferInput},
};
use candid::Nat;
use ic_canister_core::{api::ServiceResult, utils::rfc3339_to_timestamp};
use ic_canister_core::{model::ModelValidator, repository::Repository};
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct TransferService {
    call_context: CallContext,
    account_service: AccountService,
    wallet_service: WalletService,
    wallet_repository: WalletRepository,
    transfer_repository: TransferRepository,
    operation_repository: OperationRepository,
}

impl WithCallContext for TransferService {
    fn with_call_context(call_context: CallContext) -> Self {
        Self {
            call_context: call_context.clone(),
            account_service: AccountService::with_call_context(call_context.clone()),
            wallet_service: WalletService::with_call_context(call_context.clone()),
            ..Default::default()
        }
    }
}

impl TransferService {
    pub fn get_transfer(&self, id: &TransferId) -> ServiceResult<Transfer> {
        let transfer_key = Transfer::key(*id);
        let transfer = self.transfer_repository.get(&transfer_key).ok_or({
            TransferError::TransferNotFound {
                transfer_id: Uuid::from_bytes(*id).hyphenated().to_string(),
            }
        })?;

        self.assert_transfer_access(&transfer)?;

        Ok(transfer)
    }

    pub fn get_transfers(&self, transfer_ids: Vec<TransferId>) -> ServiceResult<Vec<Transfer>> {
        if transfer_ids.len() > 50 {
            Err(TransferError::GetTransfersBatchNotAllowed { max: 50 })?
        }

        let mut transfers = Vec::new();
        for transfer_id in transfer_ids.iter() {
            let transfer = self.get_transfer(transfer_id)?;
            self.assert_transfer_access(&transfer)?;
            transfers.push(transfer);
        }

        Ok(transfers)
    }

    pub async fn create_transfer(&self, input: TransferInput) -> ServiceResult<Transfer> {
        // validate account is owner of wallet
        let caller_account = self
            .account_service
            .get_account_by_identity(&self.call_context.caller())?;
        let wallet_id = HelperMapper::to_uuid(input.from_wallet_id.clone())?;
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

        let mut transfer = TransferMapper::from_create_input(
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
            .insert(transfer.to_key(), transfer.to_owned());

        operations.iter().for_each(|operation| {
            self.operation_repository
                .insert(operation.to_key(), operation.to_owned());
        });

        let processor = OperationProcessorFactory::build(&OperationCode::ApproveTransfer);
        for operation in operations.iter() {
            processor
                .post_process(operation)
                .await
                .expect("Operation post processing failed");
        }

        Ok(transfer)
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

    pub fn list_wallet_transfers(
        &self,
        input: ListWalletTransfersInput,
    ) -> ServiceResult<Vec<Transfer>> {
        let wallet = self
            .wallet_service
            .get_wallet(HelperMapper::to_uuid(input.wallet_id)?.as_bytes())?;

        let transfers = self.transfer_repository.find_by_wallet(
            wallet.id,
            input.from_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
            input.to_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
            input.status,
        );

        Ok(transfers)
    }

    fn assert_transfer_access(&self, transfer: &Transfer) -> ServiceResult<()> {
        let caller_account = self
            .account_service
            .get_account_by_identity(&self.call_context.caller())?;
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
}
