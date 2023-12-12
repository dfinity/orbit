use super::{PolicyService, UserService};
use crate::{
    core::{generate_uuid_v4, CallContext, ACCOUNT_BALANCE_FRESHNESS_IN_MS},
    errors::AccountError,
    factories::blockchains::BlockchainApiFactory,
    mappers::{AccountMapper, HelperMapper},
    models::{
        specifier::{AccountSpecifier, AddressSpecifier, ProposalSpecifier},
        Account, AccountBalance, AccountId, AddAccountOperationInput, EditAccountOperationInput,
    },
    repositories::AccountRepository,
};
use candid::Principal;
use ic_canister_core::{
    api::ServiceResult, cdk::api::time, model::ModelValidator, repository::Repository,
};
use lazy_static::lazy_static;
use uuid::Uuid;
use wallet_api::{AccountBalanceDTO, FetchAccountBalancesInput};

lazy_static! {
    pub static ref ACCOUNT_SERVICE: AccountService = AccountService::new(
        UserService::default(),
        PolicyService::default(),
        AccountRepository::default(),
    );
}

#[derive(Default, Debug)]
pub struct AccountService {
    user_service: UserService,
    policy_service: PolicyService,
    account_repository: AccountRepository,
}

impl AccountService {
    pub fn new(
        user_service: UserService,
        policy_service: PolicyService,
        account_repository: AccountRepository,
    ) -> Self {
        Self {
            user_service,
            policy_service,
            account_repository,
        }
    }

    /// Returns the account associated with the given account id.
    pub fn get_account(&self, id: &AccountId) -> ServiceResult<Account> {
        let account_key = Account::key(*id);
        let account =
            self.account_repository
                .get(&account_key)
                .ok_or(AccountError::AccountNotFound {
                    id: Uuid::from_bytes(*id).hyphenated().to_string(),
                })?;

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
    /// This operation will fail if an account owner does not have an associated user.
    pub async fn create_account(&self, input: AddAccountOperationInput) -> ServiceResult<Account> {
        for user_id in input.owners.iter() {
            self.user_service.assert_user_exists(user_id)?;
        }

        let uuid = generate_uuid_v4().await;
        let key = Account::key(*uuid.as_bytes());
        let blockchain_api =
            BlockchainApiFactory::build(&input.blockchain.clone(), &input.standard.clone())?;
        let mut new_account =
            AccountMapper::from_create_input(input.to_owned(), *uuid.as_bytes(), None)?;

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

        // adds the associated transfer policy based on the transfer criteria
        if let Some(transfer_criteria) = input.policies.transfer {
            let policy = self
                .policy_service
                .add_proposal_policy(
                    ProposalSpecifier::Transfer(
                        AccountSpecifier::Id(vec![*uuid.as_bytes()]),
                        AddressSpecifier::Any,
                    ),
                    transfer_criteria.to_owned(),
                )
                .await?;

            new_account.policies.transfer_policy_id = Some(policy.id);
        }

        // adds the associated edit policy based on the edit criteria
        if let Some(edit_criteria) = input.policies.edit {
            let policy = self
                .policy_service
                .add_proposal_policy(
                    ProposalSpecifier::EditAccount(AccountSpecifier::Id(vec![*uuid.as_bytes()])),
                    edit_criteria.to_owned(),
                )
                .await?;

            new_account.policies.edit_policy_id = Some(policy.id);
        }

        // Inserting the account into the repository and its associations is the last step of the account creation
        // process to avoid potential consistency issues due to the fact that some of the calls to create the account
        // happen in an asynchronous way.
        self.account_repository.insert(key, new_account.clone());

        Ok(new_account)
    }

    /// Edits the account with the given id and updates the associated policies if provided.
    ///
    /// This operation will fail if an account owner does not have an associated user.
    pub async fn edit_account(&self, input: EditAccountOperationInput) -> ServiceResult<Account> {
        let mut account = self.get_account(&input.account_id)?;

        if let Some(name) = &input.name {
            account.name = name.to_owned();
        }

        if let Some(owners) = &input.owners {
            for user_id in owners.iter() {
                self.user_service.assert_user_exists(user_id)?;
            }

            account.owners = owners.to_owned();
        }

        if let Some(policies) = input.policies {
            match (account.policies.transfer_policy_id, policies.transfer) {
                (Some(id), Some(criteria)) => {
                    self.policy_service
                        .edit_proposal_policy(
                            &id,
                            ProposalSpecifier::Transfer(
                                AccountSpecifier::Id(vec![account.id]),
                                AddressSpecifier::Any,
                            ),
                            criteria.to_owned(),
                        )
                        .await?;
                }
                (None, Some(criteria)) => {
                    let policy = self
                        .policy_service
                        .add_proposal_policy(
                            ProposalSpecifier::Transfer(
                                AccountSpecifier::Id(vec![account.id]),
                                AddressSpecifier::Any,
                            ),
                            criteria.to_owned(),
                        )
                        .await?;

                    account.policies.transfer_policy_id = Some(policy.id);
                }
                _ => {}
            }
        }

        account.validate()?;

        self.account_repository
            .insert(account.to_key(), account.to_owned());

        Ok(account)
    }

    /// Returns the balances of the requested accounts.
    ///
    /// If the balance is considered fresh it will be returned, otherwise it will be fetched from the blockchain.
    pub async fn fetch_account_balances(
        &self,
        input: FetchAccountBalancesInput,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::test_utils,
        models::{
            account_test_utils::mock_account, criteria::Criteria, user_test_utils::mock_user,
            AccountPoliciesInput, AddAccountOperation, AddAccountOperationInput, Blockchain,
            BlockchainStandard, EvaluationStatus, User,
        },
        repositories::UserRepository,
    };

    struct TestContext {
        repository: AccountRepository,
        service: AccountService,
        caller_user: User,
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
        }
    }

    #[test]
    fn get_account() {
        let ctx = setup();
        let mut account = mock_account();
        account.owners.push(ctx.caller_user.id);

        ctx.repository.insert(account.to_key(), account.clone());

        let result = ctx.service.get_account(&account.id);

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn create_account() {
        let ctx = setup();
        let operation = AddAccountOperation {
            account_id: None,
            input: AddAccountOperationInput {
                name: "foo".to_string(),
                owners: vec![ctx.caller_user.id],
                blockchain: Blockchain::InternetComputer,
                standard: BlockchainStandard::Native,
                metadata: vec![],
                policies: AccountPoliciesInput {
                    transfer: Some(Criteria::Auto(EvaluationStatus::Adopted)),
                    edit: Some(Criteria::Auto(EvaluationStatus::Adopted)),
                },
            },
        };

        let result = ctx.service.create_account(operation.input).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn fail_create_account_invalid_blockchain_standard() {
        let ctx = setup();
        let operation = AddAccountOperation {
            account_id: None,
            input: AddAccountOperationInput {
                name: "foo".to_string(),
                owners: vec![ctx.caller_user.id],
                blockchain: Blockchain::InternetComputer,
                standard: BlockchainStandard::ERC20,
                metadata: vec![],
                policies: AccountPoliciesInput {
                    transfer: Some(Criteria::Auto(EvaluationStatus::Adopted)),
                    edit: Some(Criteria::Auto(EvaluationStatus::Adopted)),
                },
            },
        };

        let result = ctx.service.create_account(operation.input).await;

        assert!(result.is_err());
    }
}
