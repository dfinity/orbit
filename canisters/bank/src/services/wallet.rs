use super::UserService;
use crate::{
    core::{generate_uuid_v4, CallContext, WithCallContext, WALLET_BALANCE_FRESHNESS_IN_MS},
    errors::WalletError,
    factories::blockchains::BlockchainApiFactory,
    mappers::{BlockchainMapper, HelperMapper, WalletMapper},
    models::{Wallet, WalletBalance, WalletId},
    repositories::WalletRepository,
    transport::{CreateWalletInput, FetchWalletBalancesInput, WalletBalanceDTO},
};
use candid::Principal;
use ic_canister_core::{
    api::ServiceResult, cdk::api::time, model::ModelValidator, repository::Repository, types::UUID,
};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct WalletService {
    call_context: CallContext,
    user_service: UserService,
    wallet_repository: WalletRepository,
}

impl WithCallContext for WalletService {
    fn with_call_context(call_context: CallContext) -> Self {
        Self {
            call_context: call_context.clone(),
            user_service: UserService::with_call_context(call_context.clone()),
            ..Default::default()
        }
    }
}

impl WalletService {
    /// Returns the wallet associated with the given wallet id.
    pub fn get_wallet(&self, id: &WalletId) -> ServiceResult<Wallet> {
        let wallet_key = Wallet::key(*id);
        let wallet =
            self.wallet_repository
                .get(&wallet_key)
                .ok_or(WalletError::WalletNotFound {
                    id: Uuid::from_bytes(*id).hyphenated().to_string(),
                })?;

        self.assert_wallet_access(&wallet)?;

        Ok(wallet)
    }

    /// Returns a list of all the wallets of the requested owner identity.
    ///
    /// If the caller has a different identity than the requested owner, then the call
    /// will fail with a forbidden error if the user is not an admin.
    pub fn list_wallets(&self, owner_identity: Principal) -> ServiceResult<Vec<Wallet>> {
        let user = self.user_service.get_user_by_identity(&owner_identity)?;

        let wallets = self.wallet_repository.find_by_user_id(user.id);

        Ok(wallets)
    }

    /// Creates a new wallet, if the caller has not added itself as one of the owners of the wallet,
    /// it will be added automatically.
    ///
    /// This operation will fail if the user does not have an associated user.
    pub async fn create_wallet(&self, input: CreateWalletInput) -> ServiceResult<Wallet> {
        let caller_user = self
            .user_service
            .get_user_by_identity(&self.call_context.caller())?;

        let mut owners_users: HashSet<UUID> = HashSet::from_iter(vec![caller_user.id]);
        for user_id in input.owners.iter() {
            let user_id = HelperMapper::to_uuid(user_id.clone())?;
            self.user_service.assert_user_exists(user_id.as_bytes())?;

            owners_users.insert(*user_id.as_bytes());
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
            owners_users.iter().copied().collect(),
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

        Ok(new_wallet)
    }

    /// Returns the balances of the requested wallets.
    ///
    /// If the balance is considered fresh it will be returned, otherwise it will be fetched from the blockchain.
    pub async fn fetch_wallet_balances(
        &self,
        input: FetchWalletBalancesInput,
    ) -> ServiceResult<Vec<WalletBalanceDTO>> {
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

        for wallet in wallets.iter() {
            self.assert_wallet_access(wallet)?;
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
                        .insert(wallet.to_key(), wallet.clone());

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

    /// Checks if the caller has access to the given wallet.
    ///
    /// Canister controllers have access to all wallets.
    pub fn assert_wallet_access(&self, wallet: &Wallet) -> ServiceResult<()> {
        if self.call_context.is_admin() {
            return Ok(());
        }

        let caller_user = self
            .user_service
            .get_user_by_identity(&self.call_context.caller())?;

        let is_wallet_owner = wallet.owners.contains(&caller_user.id);

        if !is_wallet_owner {
            Err(WalletError::Forbidden)?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::test_utils,
        models::{user_test_utils::mock_user, wallet_test_utils::mock_wallet, User},
        repositories::UserRepository,
    };

    struct TestContext {
        repository: WalletRepository,
        service: WalletService,
        caller_user: User,
    }

    fn setup() -> TestContext {
        test_utils::init_canister_config();

        let call_context = CallContext::new(Principal::from_slice(&[9; 29]));
        let mut user = mock_user();
        user.identities = vec![call_context.caller()];

        UserRepository::default().insert(user.to_key(), user.clone());

        TestContext {
            repository: WalletRepository::default(),
            service: WalletService::with_call_context(call_context),
            caller_user: user,
        }
    }

    #[test]
    fn get_wallet() {
        let ctx = setup();
        let mut wallet = mock_wallet();
        wallet.owners.push(ctx.caller_user.id);

        ctx.repository.insert(wallet.to_key(), wallet.clone());

        let result = ctx.service.get_wallet(&wallet.id);

        assert!(result.is_ok());
    }

    #[test]
    fn fail_get_wallet_not_allowed() {
        let ctx = setup();
        let wallet = mock_wallet();

        ctx.repository.insert(wallet.to_key(), wallet.clone());

        let result = ctx.service.get_wallet(&wallet.id);

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn create_wallet() {
        let ctx = setup();
        let input = CreateWalletInput {
            name: Some("foo".to_string()),
            owners: vec![Uuid::from_bytes(ctx.caller_user.id).to_string()],
            blockchain: "icp".to_string(),
            standard: "native".to_string(),
            metadata: None,
            policies: vec![],
        };

        let result = ctx.service.create_wallet(input).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn fail_create_wallet_unknown_blockchain() {
        let ctx = setup();
        let input = CreateWalletInput {
            name: Some("foo".to_string()),
            owners: vec![Uuid::from_bytes(ctx.caller_user.id).to_string()],
            blockchain: "unknown".to_string(),
            standard: "native".to_string(),
            metadata: None,
            policies: vec![],
        };

        let result = ctx.service.create_wallet(input).await;

        assert!(result.is_err());
    }
}
