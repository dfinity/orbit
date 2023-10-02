use super::{AccountService, WalletService};
use crate::{
    blockchains::BlockchainApiFactory,
    core::{CallContext, WithCallContext},
    errors::{AccountError, TransferError, WalletError},
    mappers::{HelperMapper, TransferMapper},
    models::{Transfer, Wallet},
    repositories::{
        TransferListIndexRepository, TransferQueueRepository, TransferRepository, WalletRepository,
    },
    transport::{
        GetTransferInput, GetWalletInput, ListWalletTransfersInput, TransferDTO, TransferInput,
        TransferListItemDTO,
    },
};
use candid::Nat;
use ic_canister_core::{
    api::ServiceResult,
    utils::{generate_uuid_v4, rfc3339_to_timestamp},
};
use ic_canister_core::{model::ModelValidator, repository::Repository};
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
    transfer_queue_repository: TransferQueueRepository,
    transfer_list_index_repository: TransferListIndexRepository,
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

    pub async fn get_transfer_core(&self, input: GetTransferInput) -> ServiceResult<Transfer> {
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

    pub async fn check_transfer_access(&self, transfer: &Transfer) -> ServiceResult<()> {
        let caller_account = self
            .account_service
            .resolve_account(&self.call_context.caller())
            .await?;
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
            Err(WalletError::Forbidden {
                wallet: Uuid::from_bytes(transfer.from_wallet)
                    .hyphenated()
                    .to_string(),
            })?
        }

        Ok(())
    }

    pub async fn get_transfer(&self, input: GetTransferInput) -> ServiceResult<TransferDTO> {
        let transfer = self.get_transfer_core(input).await?;
        self.check_transfer_access(&transfer).await?;
        let dto = self.transfer_mapper.transfer_to_dto(transfer);
        Ok(dto)
    }

    pub async fn create_transfer(&self, input: TransferInput) -> ServiceResult<TransferDTO> {
        // validate account is owner of wallet
        let caller_account = match self
            .account_service
            .maybe_resolve_account(&self.call_context.caller())
            .await?
        {
            Some(account) => account,
            None => Err(AccountError::NotFoundAccountIdentity {
                identity: self.call_context.caller().to_text(),
            })?,
        };
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
            Err(WalletError::Forbidden {
                wallet: input.from_wallet_id.clone(),
            })?
        }

        // create transfer
        let blockchain_api = BlockchainApiFactory::build(&wallet.blockchain, &wallet.standard)?;
        let default_fee = blockchain_api.transaction_fee(&wallet).await?;
        let transfer_id = generate_uuid_v4().await;

        let transfer = self.transfer_mapper.new_transfer_from_input(
            input,
            *transfer_id.as_bytes(),
            caller_account.id,
            Nat(default_fee.fee),
            blockchain_api.default_network(),
            Transfer::default_expiration_dt(),
        )?;
        transfer.validate()?;

        // save transfer to stable memory
        let index_entry = transfer.as_list_index();
        self.transfer_repository
            .insert(transfer.as_key(), transfer.to_owned());
        self.transfer_queue_repository.insert(
            transfer.as_transfer_queue_key(),
            transfer.as_transfer_queue_item(),
        );
        self.transfer_list_index_repository
            .insert(index_entry.as_key(), index_entry.to_owned());

        let dto = self.transfer_mapper.transfer_to_dto(transfer);

        Ok(dto)
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

        let transfers: Vec<Transfer> = self
            .transfer_list_index_repository
            .find_all_within_criteria(
                wallet.id,
                input.from_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
                input.to_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
                input.status,
            )?
            .iter()
            .map(|index_item| {
                let transfer_key = Transfer::key(index_item.transfer_id);
                self.transfer_repository.get(&transfer_key).unwrap()
            })
            .collect();

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
