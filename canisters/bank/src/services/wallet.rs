use crate::{
    blockchains::BlockchainApiFactory,
    core::{CallContext, WithCallContext, WALLET_BALANCE_FRESHNESS_IN_MS},
    errors::WalletError,
    mappers::{BlockchainMapper, HelperMapper, WalletMapper},
    models::{Wallet, WalletBalance},
    repositories::WalletRepository,
    transport::{
        CreateWalletInput, GetWalletBalanceInput, GetWalletInput, WalletBalanceDTO, WalletDTO,
    },
};
use ic_canister_core::{
    api::ServiceResult, cdk::api::time, repository::Repository, utils::generate_uuid_v4,
};

#[derive(Default, Debug)]
pub struct WalletService {
    // todo: removed if not used by the service
    _call_context: CallContext,
    wallet_repository: WalletRepository,
    blockchain_mapper: BlockchainMapper,
    wallet_mapper: WalletMapper,
    helper_mapper: HelperMapper,
}

impl WithCallContext for WalletService {
    fn with_call_context(self, call_context: CallContext) -> Self {
        Self {
            _call_context: call_context,
            ..self
        }
    }
}

impl WalletService {
    pub fn create() -> Self {
        Default::default()
    }

    /// Creates a new wallet for the given user.
    pub async fn create_wallet(&self, input: CreateWalletInput) -> ServiceResult<WalletDTO> {
        let uuid = generate_uuid_v4().await;
        let key = Wallet::key(*uuid.as_bytes());
        let blockchain_api = BlockchainApiFactory::build(
            &self
                .blockchain_mapper
                .str_to_blockchain(input.blockchain.clone())?,
            &self
                .blockchain_mapper
                .str_to_blockchain_standard(input.standard.clone())?,
        )?;
        let mut new_wallet =
            self.wallet_mapper
                .new_wallet_from_create_input(input, uuid, None, vec![])?;

        // The wallet address is generated after the wallet is created from the user input and
        // all the validations are successfully completed.
        if new_wallet.address.is_empty() {
            let wallet_address = blockchain_api.generate_address(&new_wallet).await?;
            new_wallet.address = wallet_address;
        }

        // The decimals of the asset are fetched from the blockchain and stored in the wallet,
        // depending on the blockchain standard used by the wallet the decimals used by each asset can vary.
        new_wallet.decimals = blockchain_api.decimals(&new_wallet).await?;

        // Inserting the wallet into the repository is the last step of the wallet creation process
        // to avoid potential consistency issues due to the fact that some of the calls to create the wallet
        // happen in an asynchronous way.
        self.wallet_repository.insert(key, new_wallet.clone());

        Ok(self.wallet_mapper.wallet_to_dto(new_wallet))
    }

    pub async fn get_wallet(&self, input: GetWalletInput) -> ServiceResult<WalletDTO> {
        let wallet_id = self.helper_mapper.uuid_from_str(input.wallet_id)?;
        let wallet_key = Wallet::key(*wallet_id.as_bytes());
        let wallet =
            self.wallet_repository
                .get(&wallet_key)
                .ok_or(WalletError::WalletNotFound {
                    id: wallet_id.hyphenated().to_string(),
                })?;

        Ok(self.wallet_mapper.wallet_to_dto(wallet))
    }

    pub async fn fetch_wallet_balance(
        &self,
        input: GetWalletBalanceInput,
    ) -> ServiceResult<WalletBalanceDTO> {
        let wallet_id = self.helper_mapper.uuid_from_str(input.wallet_id)?;
        let wallet_key = Wallet::key(*wallet_id.as_bytes());
        let mut wallet =
            self.wallet_repository
                .get(&wallet_key)
                .ok_or(WalletError::WalletNotFound {
                    id: wallet_id.hyphenated().to_string(),
                })?;

        let updated_balance: WalletBalance;
        let balance_considered_fresh = match &wallet.balance {
            Some(balance) => {
                let balance_age_ns = time() - balance.last_modification_timestamp;
                (balance_age_ns / 1_000_000) < WALLET_BALANCE_FRESHNESS_IN_MS
            }
            None => false,
        };

        match (&wallet.balance, balance_considered_fresh) {
            (None, _) | (_, false) => {
                let blockchain_api =
                    BlockchainApiFactory::build(&wallet.blockchain, &wallet.standard)?;
                let fetched_balance = blockchain_api.balance(&wallet).await?;
                let new_balance = WalletBalance {
                    balance: fetched_balance,
                    last_modification_timestamp: time(),
                };

                updated_balance = new_balance;
                wallet.balance = Some(updated_balance.clone());

                self.wallet_repository.insert(wallet_key, wallet.clone());
            }
            (_, _) => {
                updated_balance = wallet.balance.unwrap();
            }
        }

        let updated_balance_dto =
            self.wallet_mapper
                .balance_to_dto(updated_balance, wallet.decimals, wallet.id);

        Ok(updated_balance_dto)
    }
}
