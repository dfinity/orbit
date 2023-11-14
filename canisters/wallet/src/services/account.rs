use super::UserService;
use crate::{
    core::{generate_uuid_v4, CallContext, ACCOUNT_BALANCE_FRESHNESS_IN_MS},
    errors::AccountError,
    factories::blockchains::BlockchainApiFactory,
    mappers::{AccountMapper, BlockchainMapper, HelperMapper},
    models::{Account, AccountBalance, AccountId},
    repositories::AccountRepository,
    transport::{AccountBalanceDTO, CreateAccountInput, FetchAccountBalancesInput},
};
use candid::Principal;
use ic_canister_core::{
    api::ServiceResult, cdk::api::time, model::ModelValidator, repository::Repository, types::UUID,
};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct AccountService {
    user_service: UserService,
    account_repository: AccountRepository,
}

impl AccountService {
    /// Returns the account associated with the given account id.
    pub fn get_account(&self, id: &AccountId, ctx: &CallContext) -> ServiceResult<Account> {
        let account_key = Account::key(*id);
        let account =
            self.account_repository
                .get(&account_key)
                .ok_or(AccountError::AccountNotFound {
                    id: Uuid::from_bytes(*id).hyphenated().to_string(),
                })?;

        self.assert_account_access(&account, ctx)?;

        Ok(account)
    }

    /// Returns a list of all the accounts of the requested owner identity.
    ///
    /// If the caller has a different identity than the requested owner, then the call
    /// will fail with a forbidden error if the user is not an admin.
    pub fn list_accounts(
        &self,
        owner_identity: Principal,
        ctx: &CallContext,
    ) -> ServiceResult<Vec<Account>> {
        let user = self
            .user_service
            .get_user_by_identity(&owner_identity, ctx)?;

        let accounts = self.account_repository.find_by_user_id(user.id);

        Ok(accounts)
    }

    /// Creates a new account, if the caller has not added itself as one of the owners of the account,
    /// it will be added automatically.
    ///
    /// This operation will fail if the user does not have an associated user.
    pub async fn create_account(
        &self,
        input: CreateAccountInput,
        ctx: &CallContext,
    ) -> ServiceResult<Account> {
        let caller_user = self.user_service.get_user_by_identity(&ctx.caller(), ctx)?;

        let mut owners_users: HashSet<UUID> = HashSet::from_iter(vec![caller_user.id]);
        for user_id in input.owners.iter() {
            let user_id = HelperMapper::to_uuid(user_id.clone())?;
            self.user_service.assert_user_exists(user_id.as_bytes())?;

            owners_users.insert(*user_id.as_bytes());
        }

        let uuid = generate_uuid_v4().await;
        let key = Account::key(*uuid.as_bytes());
        let blockchain_api = BlockchainApiFactory::build(
            &BlockchainMapper::to_blockchain(input.blockchain.clone())?,
            &BlockchainMapper::to_blockchain_standard(input.standard.clone())?,
        )?;
        let mut new_account = AccountMapper::from_create_input(
            input,
            *uuid.as_bytes(),
            None,
            owners_users.iter().copied().collect(),
        )?;

        // The account address is generated after the account is created from the user input and
        // all the validations are successfully completed.
        if new_account.address.is_empty() {
            let account_address = blockchain_api.generate_address(&new_account).await?;
            new_account.address = account_address;
        }

        // The decimals of the asset are fetched from the blockchain and stored in the account,
        // depending on the blockchain standard used by the account the decimals used by each asset can vary.
        new_account.decimals = blockchain_api.decimals(&new_account).await?;

        // Validations happen after all the fields are set in the account to avoid partial data in the repository.
        new_account.validate()?;

        // Inserting the account into the repository and its associations is the last step of the account creation
        // process to avoid potential consistency issues due to the fact that some of the calls to create the account
        // happen in an asynchronous way.
        self.account_repository.insert(key, new_account.clone());

        Ok(new_account)
    }

    /// Returns the balances of the requested accounts.
    ///
    /// If the balance is considered fresh it will be returned, otherwise it will be fetched from the blockchain.
    pub async fn fetch_account_balances(
        &self,
        input: FetchAccountBalancesInput,
        ctx: &CallContext,
    ) -> ServiceResult<Vec<AccountBalanceDTO>> {
        if input.account_ids.is_empty() || input.account_ids.len() > 5 {
            Err(AccountError::AccountBalancesBatchRange { min: 1, max: 5 })?
        }

        let account_ids = input
            .account_ids
            .iter()
            .map(|id| HelperMapper::to_uuid(id.clone()))
            .collect::<Result<Vec<Uuid>, _>>()?;

        let accounts = self
            .account_repository
            .find_by_ids(account_ids.iter().map(|id| *id.as_bytes()).collect());

        for account in accounts.iter() {
            self.assert_account_access(account, ctx)?;
        }

        let mut balances = Vec::new();
        for mut account in accounts {
            let balance_considered_fresh = match &account.balance {
                Some(balance) => {
                    let balance_age_ns = time() - balance.last_modification_timestamp;
                    (balance_age_ns / 1_000_000) < ACCOUNT_BALANCE_FRESHNESS_IN_MS
                }
                None => false,
            };
            let balance: AccountBalance = match (&account.balance, balance_considered_fresh) {
                (None, _) | (_, false) => {
                    let blockchain_api =
                        BlockchainApiFactory::build(&account.blockchain, &account.standard)?;
                    let fetched_balance = blockchain_api.balance(&account).await?;
                    let new_balance = AccountBalance {
                        balance: candid::Nat(fetched_balance),
                        last_modification_timestamp: time(),
                    };

                    account.balance = Some(new_balance.clone());

                    self.account_repository
                        .insert(account.to_key(), account.clone());

                    new_balance
                }
                (_, _) => account.balance.unwrap(),
            };

            balances.push(AccountMapper::to_balance_dto(
                balance,
                account.decimals,
                account.id,
            ));
        }

        Ok(balances)
    }

    /// Checks if the caller has access to the given account.
    ///
    /// Canister controllers have access to all accounts.
    pub fn assert_account_access(&self, account: &Account, ctx: &CallContext) -> ServiceResult<()> {
        if ctx.is_admin() {
            return Ok(());
        }

        let caller_user = self.user_service.get_user_by_identity(&ctx.caller(), ctx)?;

        let is_account_owner = account.owners.contains(&caller_user.id);

        if !is_account_owner {
            Err(AccountError::Forbidden)?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::test_utils,
        models::{account_test_utils::mock_account, user_test_utils::mock_user, User},
        repositories::UserRepository,
    };

    struct TestContext {
        repository: AccountRepository,
        service: AccountService,
        caller_user: User,
        call_context: CallContext,
    }

    fn setup() -> TestContext {
        test_utils::init_canister_config();

        let call_context = CallContext::new(Principal::from_slice(&[9; 29]));
        let mut user = mock_user();
        user.identities = vec![call_context.caller()];

        UserRepository::default().insert(user.to_key(), user.clone());

        TestContext {
            repository: AccountRepository::default(),
            service: AccountService::default(),
            caller_user: user,
            call_context,
        }
    }

    #[test]
    fn get_account() {
        let ctx = setup();
        let mut account = mock_account();
        account.owners.push(ctx.caller_user.id);

        ctx.repository.insert(account.to_key(), account.clone());

        let result = ctx.service.get_account(&account.id, &ctx.call_context);

        assert!(result.is_ok());
    }

    #[test]
    fn fail_get_account_not_allowed() {
        let ctx = setup();
        let account = mock_account();

        ctx.repository.insert(account.to_key(), account.clone());

        let result = ctx.service.get_account(&account.id, &ctx.call_context);

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn create_account() {
        let ctx = setup();
        let input = CreateAccountInput {
            name: Some("foo".to_string()),
            owners: vec![Uuid::from_bytes(ctx.caller_user.id).to_string()],
            blockchain: "icp".to_string(),
            standard: "native".to_string(),
            metadata: None,
            policies: vec![],
        };

        let result = ctx.service.create_account(input, &ctx.call_context).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn fail_create_account_unknown_blockchain() {
        let ctx = setup();
        let input = CreateAccountInput {
            name: Some("foo".to_string()),
            owners: vec![Uuid::from_bytes(ctx.caller_user.id).to_string()],
            blockchain: "unknown".to_string(),
            standard: "native".to_string(),
            metadata: None,
            policies: vec![],
        };

        let result = ctx.service.create_account(input, &ctx.call_context).await;

        assert!(result.is_err());
    }
}
