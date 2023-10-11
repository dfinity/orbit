use super::AccountService;
use crate::{
    core::{CallContext, WithCallContext, WALLET_BALANCE_FRESHNESS_IN_MS},
    errors::WalletError,
    factories::blockchains::BlockchainApiFactory,
    mappers::{BlockchainMapper, HelperMapper, WalletMapper},
    models::{Wallet, WalletBalance},
    repositories::WalletRepository,
    transport::{
        CreateWalletInput, FetchWalletBalancesInput, GetWalletInput, WalletBalanceDTO, WalletDTO,
        WalletListItemDTO,
    },
};
use candid::Principal;
use ic_canister_core::{
    api::ServiceResult, cdk::api::time, model::ModelValidator, repository::Repository, types::UUID,
    utils::generate_uuid_v4,
};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct WalletService {
    call_context: CallContext,
    account_service: AccountService,
    wallet_repository: WalletRepository,
}

impl WithCallContext for WalletService {
    fn with_call_context(&mut self, call_context: CallContext) -> &Self {
        self.call_context = call_context.clone();
        self.account_service.with_call_context(call_context);

        self
    }
}

impl WalletService {
    pub fn create() -> Self {
        Default::default()
    }

    /// Creates a new wallet, if the caller has not added itself as one of the owners of the wallet,
    /// it will be added automatically.
    ///
    /// This operation will fail if the user does not have an associated account.
    pub async fn create_wallet(&self, input: CreateWalletInput) -> ServiceResult<WalletDTO> {
        let caller_account = self
            .account_service
            .resolve_account(&self.call_context.caller())?;

        let mut owners_accounts: HashSet<UUID> = HashSet::from_iter(vec![caller_account.id]);
        for account_id in input.owners.iter() {
            let account_id = HelperMapper::to_uuid(account_id.clone())?;
            self.account_service
                .assert_account_exists(account_id.as_bytes())?;

            owners_accounts.insert(*account_id.as_bytes());
        }

        let uuid = generate_uuid_v4().await;
        let key = Wallet::key(*uuid.as_bytes());
        let blockchain_api = BlockchainApiFactory::build(
            &BlockchainMapper::to_blockchain(input.blockchain.clone())?,
            &BlockchainMapper::to_blockchain_standard(input.standard.clone())?,
        )?;
        let mut new_wallet = WalletMapper::from_create_input(
            input,
            *uuid.as_bytes(),
            None,
            owners_accounts.iter().copied().collect(),
        )?;

        // The wallet address is generated after the wallet is created from the user input and
        // all the validations are successfully completed.
        if new_wallet.address.is_empty() {
            let wallet_address = blockchain_api.generate_address(&new_wallet).await?;
            new_wallet.address = wallet_address;
        }

        // The decimals of the asset are fetched from the blockchain and stored in the wallet,
        // depending on the blockchain standard used by the wallet the decimals used by each asset can vary.
        new_wallet.decimals = blockchain_api.decimals(&new_wallet).await?;

        // Validations happen after all the fields are set in the wallet to avoid partial data in the repository.
        new_wallet.validate()?;

        // Inserting the wallet into the repository and its associations is the last step of the wallet creation
        // process to avoid potential consistency issues due to the fact that some of the calls to create the wallet
        // happen in an asynchronous way.
        self.wallet_repository.insert(key, new_wallet.clone());

        Ok(new_wallet.to_dto())
    }

    /// Returns the wallet associated with the given wallet id.
    pub async fn get_wallet_core(&self, input: GetWalletInput) -> ServiceResult<Wallet> {
        let caller_account = self
            .account_service
            .resolve_account(&self.call_context.caller())?;

        let wallet_id = HelperMapper::to_uuid(input.wallet_id.clone())?;
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

        Ok(wallet)
    }

    /// Returns the wallet associated with the given wallet id.
    pub async fn get_wallet(&self, input: GetWalletInput) -> ServiceResult<WalletDTO> {
        let wallet = self.get_wallet_core(input).await?;

        Ok(wallet.to_dto())
    }

    /// Returns the balances of the requested wallets.
    ///
    /// If the balance is considered fresh it will be returned, otherwise it will be fetched from the blockchain.
    pub async fn fetch_wallet_balances(
        &self,
        input: FetchWalletBalancesInput,
    ) -> ServiceResult<Vec<WalletBalanceDTO>> {
        let account = self
            .account_service
            .resolve_account(&self.call_context.caller())?;

        if input.wallet_ids.is_empty() || input.wallet_ids.len() > 5 {
            Err(WalletError::WalletBalancesBatchRange { min: 1, max: 5 })?
        }

        let wallet_ids = input
            .wallet_ids
            .iter()
            .map(|id| HelperMapper::to_uuid(id.clone()))
            .collect::<Result<Vec<Uuid>, _>>()?;

        let wallets = self
            .wallet_repository
            .find_by_ids(wallet_ids.iter().map(|id| *id.as_bytes()).collect());

        let can_access_wallets = wallets
            .iter()
            .all(|wallet| wallet.owners.contains(&account.id));

        if !can_access_wallets {
            Err(WalletError::Forbidden)?
        }

        let mut balances = Vec::new();
        for mut wallet in wallets {
            let balance_considered_fresh = match &wallet.balance {
                Some(balance) => {
                    let balance_age_ns = time() - balance.last_modification_timestamp;
                    (balance_age_ns / 1_000_000) < WALLET_BALANCE_FRESHNESS_IN_MS
                }
                None => false,
            };
            let balance: WalletBalance = match (&wallet.balance, balance_considered_fresh) {
                (None, _) | (_, false) => {
                    let blockchain_api =
                        BlockchainApiFactory::build(&wallet.blockchain, &wallet.standard)?;
                    let fetched_balance = blockchain_api.balance(&wallet).await?;
                    let new_balance = WalletBalance {
                        balance: candid::Nat(fetched_balance),
                        last_modification_timestamp: time(),
                    };

                    wallet.balance = Some(new_balance.clone());

                    self.wallet_repository
                        .insert(wallet.as_key(), wallet.clone());

                    new_balance
                }
                (_, _) => wallet.balance.unwrap(),
            };

            balances.push(WalletMapper::to_balance_dto(
                balance,
                wallet.decimals,
                wallet.id,
            ));
        }

        Ok(balances)
    }

    /// Returns a list of all the wallets of the requested owner, if no owner is provided then it returns
    /// the list of all the wallets of the caller.
    pub async fn list_wallets(
        &self,
        owner: Option<Principal>,
    ) -> ServiceResult<Vec<WalletListItemDTO>> {
        let owner = owner.unwrap_or(self.call_context.caller());
        let account = self.account_service.resolve_account(&owner)?;
        let dtos = self
            .wallet_repository
            .find_by_account_id(account.id)
            .iter()
            .map(|wallet| wallet.to_list_item_dto())
            .collect::<Vec<WalletListItemDTO>>();

        Ok(dtos)
    }
}
