use super::AccountService;
use crate::{
    blockchains::BlockchainApiFactory,
    core::{CallContext, WithCallContext},
    errors::{AccountError, WalletError},
    mappers::{HelperMapper, TransferMapper},
    models::{Transfer, Wallet},
    repositories::{TransferQueueRepository, TransferRepository, WalletRepository},
    transport::{TransferDTO, TransferInput},
};
use candid::Nat;
use ic_canister_core::{api::ServiceResult, utils::generate_uuid_v4};
use ic_canister_core::{model::ModelValidator, repository::Repository};

#[derive(Default, Debug)]
pub struct TransferService {
    call_context: CallContext,
    account_service: AccountService,
    helper_mapper: HelperMapper,
    transfer_mapper: TransferMapper,
    wallet_repository: WalletRepository,
    transfer_repository: TransferRepository,
    transfer_queue_repository: TransferQueueRepository,
}

impl WithCallContext for TransferService {
    fn with_call_context(&mut self, call_context: CallContext) -> &Self {
        self.call_context = call_context.clone();

        self
    }
}

impl TransferService {
    pub fn create() -> Self {
        Default::default()
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
        self.transfer_repository
            .insert(transfer.as_key(), transfer.to_owned());
        self.transfer_queue_repository.insert(
            transfer.as_transfer_queue_key(),
            transfer.as_transfer_queue_item(),
        );

        let dto = self.transfer_mapper.transfer_to_dto(transfer);

        Ok(dto)
    }
}
