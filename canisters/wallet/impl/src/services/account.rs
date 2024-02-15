use crate::{
    core::{
        access_control::evaluate_caller_access,
        generate_uuid_v4,
        utils::{paginated_items, PaginatedData, PaginatedItemsArgs},
        CallContext, ACCOUNT_BALANCE_FRESHNESS_IN_MS,
    },
    errors::AccountError,
    factories::blockchains::BlockchainApiFactory,
    mappers::{AccountCallerPrivileges, AccountMapper, HelperMapper},
    models::{
        access_control::{
            AccountActionSpecifier, ResourceSpecifier, ResourceType, TransferActionSpecifier,
        },
        specifier::{AccountSpecifier, CommonSpecifier, ProposalSpecifier},
        Account, AccountBalance, AccountId, AddAccountOperationInput,
        AddProposalPolicyOperationInput, EditAccountOperationInput,
        EditProposalPolicyOperationInput,
    },
    repositories::{AccountRepository, AccountWhereClause, ACCOUNT_REPOSITORY},
    services::{PolicyService, UserService, POLICY_SERVICE, USER_SERVICE},
};
use futures::{stream, StreamExt};
use ic_canister_core::{
    api::ServiceResult, cdk::api::time, model::ModelValidator, repository::Repository, types::UUID,
};
use lazy_static::lazy_static;
use std::sync::Arc;
use uuid::Uuid;
use wallet_api::{AccountBalanceDTO, FetchAccountBalancesInput, ListAccountsInput};

lazy_static! {
    pub static ref ACCOUNT_SERVICE: Arc<AccountService> = Arc::new(AccountService::new(
        Arc::clone(&USER_SERVICE),
        Arc::clone(&POLICY_SERVICE),
        Arc::clone(&ACCOUNT_REPOSITORY),
    ));
}

#[derive(Default, Debug)]
pub struct AccountService {
    user_service: Arc<UserService>,
    policy_service: Arc<PolicyService>,
    account_repository: Arc<AccountRepository>,
}

impl AccountService {
    const DEFAULT_ACCOUNT_LIST_LIMIT: u16 = 50;
    const MAX_ACCOUNT_LIST_LIMIT: u16 = 1000;

    pub fn new(
        user_service: Arc<UserService>,
        policy_service: Arc<PolicyService>,
        account_repository: Arc<AccountRepository>,
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

    /// Returns the caller privileges for the given account.
    pub async fn get_account_caller_privileges(
        &self,
        account_id: &UUID,
        ctx: &CallContext,
    ) -> ServiceResult<AccountCallerPrivileges> {
        let can_edit = evaluate_caller_access(
            ctx,
            &ResourceSpecifier::Common(
                ResourceType::Account,
                AccountActionSpecifier::Update(CommonSpecifier::Id(vec![*account_id])),
            ),
        )
        .await;

        let can_transfer = evaluate_caller_access(
            ctx,
            &ResourceSpecifier::Transfer(TransferActionSpecifier::Create(AccountSpecifier::Id(
                vec![*account_id],
            ))),
        )
        .await;

        Ok(AccountCallerPrivileges {
            can_edit: can_edit.is_ok(),
            can_transfer: can_transfer.is_ok(),
        })
    }

    /// Returns a list of all the accounts of the requested owner identity.
    pub async fn list_accounts(
        &self,
        input: ListAccountsInput,
        ctx: Option<&CallContext>,
    ) -> ServiceResult<PaginatedData<Account>> {
        let owner_ids = match ctx {
            Some(context) => Some(vec![
                self.user_service
                    .get_user_by_identity(&context.caller())?
                    .id,
            ]),
            None => None,
        };

        let mut accounts = self.account_repository.find_where(AccountWhereClause {
            owner_user_ids: owner_ids,
            search_term: None,
        });

        // filter out accounts that the caller does not have access to read
        if let Some(ctx) = ctx {
            accounts = stream::iter(accounts.iter())
                .filter_map(|account| async move {
                    match evaluate_caller_access(
                        ctx,
                        &ResourceSpecifier::Common(
                            ResourceType::Account,
                            AccountActionSpecifier::Read(CommonSpecifier::Id(vec![account
                                .id
                                .to_owned()])),
                        ),
                    )
                    .await
                    {
                        Ok(_) => Some(account.to_owned()),
                        Err(_) => None,
                    }
                })
                .collect()
                .await
        }

        let result = paginated_items(PaginatedItemsArgs {
            offset: input.paginate.to_owned().and_then(|p| p.offset),
            limit: input.paginate.and_then(|p| p.limit),
            default_limit: Some(Self::DEFAULT_ACCOUNT_LIST_LIMIT),
            max_limit: Some(Self::MAX_ACCOUNT_LIST_LIMIT),
            items: &accounts,
        })?;

        Ok(result)
    }

    /// Creates a new account, if the caller has not added itself as one of the owners of the account,
    /// it will be added automatically.
    ///
    /// This operation will fail if an account owner does not have an associated user.
    pub async fn create_account(&self, input: AddAccountOperationInput) -> ServiceResult<Account> {
        for user_id in input.owners.iter() {
            self.user_service.get_user(user_id)?;
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
                .add_proposal_policy(AddProposalPolicyOperationInput {
                    specifier: ProposalSpecifier::Transfer(AccountSpecifier::Id(vec![
                        *uuid.as_bytes()
                    ])),
                    criteria: transfer_criteria.to_owned(),
                })
                .await?;

            new_account.policies.transfer_policy_id = Some(policy.id);
        }

        // adds the associated edit policy based on the edit criteria
        if let Some(edit_criteria) = input.policies.edit {
            let policy = self
                .policy_service
                .add_proposal_policy(AddProposalPolicyOperationInput {
                    specifier: ProposalSpecifier::EditAccount(AccountSpecifier::Id(vec![
                        *uuid.as_bytes()
                    ])),
                    criteria: edit_criteria.to_owned(),
                })
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
                self.user_service.get_user(user_id)?;
            }

            account.owners = owners.to_owned();
        }

        if let Some(policies) = input.policies {
            match (account.policies.transfer_policy_id, policies.transfer) {
                (Some(id), Some(criteria)) => {
                    self.policy_service
                        .edit_proposal_policy(EditProposalPolicyOperationInput {
                            policy_id: id,
                            specifier: Some(ProposalSpecifier::Transfer(AccountSpecifier::Id(
                                vec![account.id],
                            ))),
                            criteria: Some(criteria.to_owned()),
                        })
                        .await?;
                }
                (None, Some(criteria)) => {
                    let policy = self
                        .policy_service
                        .add_proposal_policy(AddProposalPolicyOperationInput {
                            specifier: ProposalSpecifier::Transfer(AccountSpecifier::Id(vec![
                                account.id,
                            ])),
                            criteria: criteria.to_owned(),
                        })
                        .await?;

                    account.policies.transfer_policy_id = Some(policy.id);
                }
                _ => {}
            }

            match (account.policies.edit_policy_id, policies.edit) {
                (Some(id), Some(criteria)) => {
                    self.policy_service
                        .edit_proposal_policy(EditProposalPolicyOperationInput {
                            policy_id: id,
                            specifier: Some(ProposalSpecifier::EditAccount(AccountSpecifier::Id(
                                vec![account.id],
                            ))),
                            criteria: Some(criteria.to_owned()),
                        })
                        .await?;
                }
                (None, Some(criteria)) => {
                    let policy = self
                        .policy_service
                        .add_proposal_policy(AddProposalPolicyOperationInput {
                            specifier: ProposalSpecifier::EditAccount(AccountSpecifier::Id(vec![
                                account.id,
                            ])),
                            criteria: criteria.to_owned(),
                        })
                        .await?;

                    account.policies.edit_policy_id = Some(policy.id);
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
    use candid::Principal;

    use super::*;
    use crate::{
        core::{test_utils, CallContext},
        models::{
            account_test_utils::mock_account, criteria::Criteria, user_test_utils::mock_user,
            AccountPoliciesInput, AddAccountOperation, AddAccountOperationInput, Blockchain,
            BlockchainStandard, Metadata, User,
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
                metadata: Metadata::default(),
                policies: AccountPoliciesInput {
                    transfer: Some(Criteria::AutoAdopted),
                    edit: Some(Criteria::AutoAdopted),
                },
            },
        };

        let result = ctx.service.create_account(operation.input).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn edit_account() {
        let ctx = setup();
        let mut account = mock_account();
        account.owners.push(ctx.caller_user.id);

        ctx.repository.insert(account.to_key(), account.clone());

        let operation = EditAccountOperationInput {
            account_id: account.id,
            name: Some("test_edit".to_string()),
            owners: Some(vec![ctx.caller_user.id]),
            policies: None,
        };

        let result = ctx.service.edit_account(operation).await;

        assert!(result.is_ok());

        let updated_account = result.unwrap();

        assert_eq!(updated_account.name, "test_edit");
        assert_eq!(updated_account.owners, vec![ctx.caller_user.id]);
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
                metadata: Metadata::default(),
                policies: AccountPoliciesInput {
                    transfer: Some(Criteria::AutoAdopted),
                    edit: Some(Criteria::AutoAdopted),
                },
            },
        };

        let result = ctx.service.create_account(operation.input).await;

        assert!(result.is_err());
    }
}
