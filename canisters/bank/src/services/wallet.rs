use super::AccountService;
use crate::{
    blockchains::BlockchainApiFactory,
    core::{CallContext, WithCallContext, WALLET_BALANCE_FRESHNESS_IN_MS},
    errors::{AccountError, WalletError},
    mappers::{BlockchainMapper, HelperMapper, WalletMapper},
    models::{Wallet, WalletAccount, WalletBalance, WalletValidator},
    repositories::{WalletAccountRepository, WalletRepository},
    transport::{
        CreateWalletInput, CreateWalletInputOwnersItemDTO, GetWalletBalanceInput, GetWalletInput,
        WalletBalanceDTO, WalletDTO, WalletListItemDTO,
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
    wallet_account_repository: WalletAccountRepository,
    blockchain_mapper: BlockchainMapper,
    wallet_mapper: WalletMapper,
    helper_mapper: HelperMapper,
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
            .get_user_account_or_create(&self.call_context.caller())
            .await?;

        let mut owners_accounts: HashSet<UUID> = HashSet::from_iter(vec![caller_account.id]);
        // this validation repeated here to avoid unnecessary calls to create accounts that will not be used,
        // the validation is also done in the wallet model itself.
        let nr_owners = input.owners.len();
        if nr_owners > WalletValidator::OWNERS_RANGE.1 as usize {
            Err(WalletError::InvalidOwnersRange {
                min_owners: WalletValidator::OWNERS_RANGE.0,
                max_owners: WalletValidator::OWNERS_RANGE.1,
            })?
        }

        for owner in input.owners.iter() {
            match owner {
                CreateWalletInputOwnersItemDTO::AccountId(account_id) => {
                    let account_id = self.helper_mapper.uuid_from_str(account_id.clone())?;
                    self.account_service
                        .assert_account_exists(account_id.as_bytes())
                        .await?;

                    owners_accounts.insert(*account_id.as_bytes());
                }
                CreateWalletInputOwnersItemDTO::Principal_(principal) => {
                    let new_account = self
                        .account_service
                        .get_user_account_or_create(principal)
                        .await?;

                    owners_accounts.insert(new_account.id);
                }
            }
        }

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
        let mut new_wallet = self.wallet_mapper.new_wallet_from_create_input(
            input,
            *uuid.as_bytes(),
            None,
            owners_accounts.iter().copied().collect(),
        )?;

        let wallet_accounts_association = new_wallet
            .owners
            .iter()
            .map(|owner_account_id| {
                self.wallet_mapper
                    .account_to_wallet_association(&new_wallet, owner_account_id)
            })
            .collect::<Vec<WalletAccount>>();

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
        wallet_accounts_association
            .iter()
            .for_each(|wallet_account| {
                self.wallet_account_repository
                    .insert(wallet_account.as_key(), wallet_account.clone());
            });

        Ok(self.wallet_mapper.wallet_to_dto(new_wallet))
    }

    /// Returns the wallet associated with the given wallet id.
    pub async fn get_wallet_core(&self, input: GetWalletInput) -> ServiceResult<Wallet> {
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

        let wallet_id = self.helper_mapper.uuid_from_str(input.wallet_id.clone())?;
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
                wallet: input.wallet_id.clone(),
            })?
        }

        Ok(wallet)
    }

    /// Returns the wallet associated with the given wallet id.
    pub async fn get_wallet(&self, input: GetWalletInput) -> ServiceResult<WalletDTO> {
        let wallet = self.get_wallet_core(input).await?;

        Ok(self.wallet_mapper.wallet_to_dto(wallet))
    }

    /// Returns the balance of the given wallet id, fetching it from the blockchain ledger if necessary.
    pub async fn fetch_wallet_balance(
        &self,
        input: GetWalletBalanceInput,
    ) -> ServiceResult<WalletBalanceDTO> {
        let caller_account = match self
            .account_service
            .maybe_resolve_account(&self.call_context.caller())
            .await?
        {
            Some(account) => account,
            None => Err(WalletError::Forbidden {
                wallet: input.wallet_id.clone(),
            })?,
        };
        let wallet_id = self.helper_mapper.uuid_from_str(input.wallet_id.clone())?;
        let wallet_key = Wallet::key(*wallet_id.as_bytes());
        let mut wallet =
            self.wallet_repository
                .get(&wallet_key)
                .ok_or(WalletError::WalletNotFound {
                    id: wallet_id.hyphenated().to_string(),
                })?;

        let is_wallet_owner = wallet.owners.contains(&caller_account.id);
        if !is_wallet_owner {
            Err(WalletError::Forbidden {
                wallet: input.wallet_id.clone(),
            })?
        }

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
                    balance: candid::Nat(fetched_balance),
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

    /// Returns a list of all the wallets of the requested owner, if no owner is provided then it returns
    /// the list of all the wallets of the caller.
    pub async fn list_wallets(
        &self,
        owner: Option<Principal>,
    ) -> ServiceResult<Vec<WalletListItemDTO>> {
        let owner = owner.unwrap_or(self.call_context.caller());
        let account = self.account_service.resolve_account(&owner).await?;
        let wallet_accounts = self
            .wallet_account_repository
            .find_by_account_id(&account.id);
        let mut wallets: Vec<Wallet> = vec![];
        for wallet_account in wallet_accounts {
            let wallet = self
                .wallet_repository
                .get(&Wallet::key(wallet_account.wallet_id))
                .ok_or(WalletError::WalletNotFound {
                    id: Uuid::from_bytes(wallet_account.wallet_id)
                        .hyphenated()
                        .to_string(),
                })?;

            wallets.push(wallet);
        }

        let dtos = wallets
            .iter()
            .map(|wallet| self.wallet_mapper.wallet_list_item(wallet))
            .collect::<Vec<WalletListItemDTO>>();

        Ok(dtos)
    }
}
