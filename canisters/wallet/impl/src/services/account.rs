use crate::{
    core::{
        authorization::Authorization,
        generate_uuid_v4,
        ic_cdk::next_time,
        utils::{paginated_items, retain_accessible_resources, PaginatedData, PaginatedItemsArgs},
        CallContext, ACCOUNT_BALANCE_FRESHNESS_IN_MS,
    },
    errors::AccountError,
    factories::blockchains::BlockchainApiFactory,
    mappers::{account::AccountMapper, HelperMapper},
    models::{
        resource::{AccountResourceAction, Resource, ResourceId, ResourceIds},
        specifier::ProposalSpecifier,
        Account, AccountBalance, AccountCallerPrivileges, AccountId, AddAccountOperationInput,
        AddProposalPolicyOperationInput, EditAccessPolicyOperationInput, EditAccountOperationInput,
    },
    repositories::{AccountRepository, AccountWhereClause, ACCOUNT_REPOSITORY},
    services::{
        access_policy::{AccessPolicyService, ACCESS_POLICY_SERVICE},
        ProposalPolicyService, PROPOSAL_POLICY_SERVICE,
    },
};
use ic_canister_core::{
    api::ServiceResult, model::ModelValidator, repository::Repository, types::UUID,
};
use lazy_static::lazy_static;
use std::sync::Arc;
use uuid::Uuid;
use wallet_api::{AccountBalanceDTO, FetchAccountBalancesInput, ListAccountsInput};

lazy_static! {
    pub static ref ACCOUNT_SERVICE: Arc<AccountService> = Arc::new(AccountService::new(
        Arc::clone(&PROPOSAL_POLICY_SERVICE),
        Arc::clone(&ACCESS_POLICY_SERVICE),
        Arc::clone(&ACCOUNT_REPOSITORY),
    ));
}

#[derive(Default, Debug)]
pub struct AccountService {
    proposal_policy_service: Arc<ProposalPolicyService>,
    access_policy_service: Arc<AccessPolicyService>,
    account_repository: Arc<AccountRepository>,
}

impl AccountService {
    const DEFAULT_ACCOUNT_LIST_LIMIT: u16 = 50;
    const MAX_ACCOUNT_LIST_LIMIT: u16 = 1000;

    pub fn new(
        proposal_policy_service: Arc<ProposalPolicyService>,
        access_policy_service: Arc<AccessPolicyService>,
        account_repository: Arc<AccountRepository>,
    ) -> Self {
        Self {
            proposal_policy_service,
            access_policy_service,
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
    pub async fn get_caller_privileges_for_account(
        &self,
        account_id: &UUID,
        ctx: &CallContext,
    ) -> ServiceResult<AccountCallerPrivileges> {
        Ok(AccountCallerPrivileges {
            id: *account_id,
            can_edit: Authorization::is_allowed(
                ctx,
                &Resource::Account(AccountResourceAction::Update(ResourceId::Id(*account_id))),
            ),
            can_transfer: Authorization::is_allowed(
                ctx,
                &Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(*account_id))),
            ),
        })
    }

    /// Returns a list of all the accounts of the requested owner identity.
    pub async fn list_accounts(
        &self,
        input: ListAccountsInput,
        ctx: &CallContext,
    ) -> ServiceResult<PaginatedData<Account>> {
        let mut accounts = self
            .account_repository
            .find_where(AccountWhereClause { search_term: None });

        // filter out accounts that the caller does not have access to read
        retain_accessible_resources(ctx, &mut accounts, |account: &Account| {
            Resource::Account(AccountResourceAction::Read(ResourceId::Id(account.id)))
        });

        let result = paginated_items(PaginatedItemsArgs {
            offset: input.paginate.to_owned().and_then(|p| p.offset),
            limit: input.paginate.and_then(|p| p.limit),
            default_limit: Some(Self::DEFAULT_ACCOUNT_LIST_LIMIT),
            max_limit: Some(Self::MAX_ACCOUNT_LIST_LIMIT),
            items: &accounts,
        })?;

        Ok(result)
    }

    /// Creates a new account.
    pub async fn create_account(&self, input: AddAccountOperationInput) -> ServiceResult<Account> {
        if self
            .account_repository
            .find_account_id_by_name(&input.name)
            .is_some()
        {
            Err(AccountError::AccountNameAlreadyExists)?
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

        // adds the associated transfer policy based on the transfer criteria
        if let Some(criteria) = &input.transfer_approval_policy {
            let transfer_approval_policy = self
                .proposal_policy_service
                .add_proposal_policy(AddProposalPolicyOperationInput {
                    specifier: ProposalSpecifier::Transfer(ResourceIds::Ids(
                        vec![*uuid.as_bytes()],
                    )),
                    criteria: criteria.clone(),
                })
                .await?;

            new_account.transfer_approval_policy_id = Some(transfer_approval_policy.id);
        }

        // adds the associated edit policy based on the edit criteria
        if let Some(criteria) = &input.update_approval_policy {
            let update_approval_policy = self
                .proposal_policy_service
                .add_proposal_policy(AddProposalPolicyOperationInput {
                    specifier: ProposalSpecifier::EditAccount(ResourceIds::Ids(vec![
                        *uuid.as_bytes()
                    ])),
                    criteria: criteria.to_owned(),
                })
                .await?;

            new_account.update_approval_policy_id = Some(update_approval_policy.id);
        }

        // Validations happen after all the fields are set in the account to avoid partial data in the repository.
        new_account.validate()?;

        // Inserting the account into the repository and its associations is the last step of the account creation
        // process to avoid potential consistency issues due to the fact that some of the calls to create the account
        // happen in an asynchronous way.
        self.account_repository.insert(key, new_account.clone());

        // Adds the access policies for the account.
        self.access_policy_service
            .edit_access_policy(EditAccessPolicyOperationInput {
                auth_scope: Some(input.read_access_policy.auth_scope),
                users: Some(input.read_access_policy.users),
                user_groups: Some(input.read_access_policy.user_groups),
                resource: Resource::Account(AccountResourceAction::Read(ResourceId::Id(
                    *uuid.as_bytes(),
                ))),
            })
            .await?;

        self.access_policy_service
            .edit_access_policy(EditAccessPolicyOperationInput {
                auth_scope: Some(input.update_access_policy.auth_scope),
                users: Some(input.update_access_policy.users),
                user_groups: Some(input.update_access_policy.user_groups),
                resource: Resource::Account(AccountResourceAction::Update(ResourceId::Id(
                    *uuid.as_bytes(),
                ))),
            })
            .await?;

        self.access_policy_service
            .edit_access_policy(EditAccessPolicyOperationInput {
                auth_scope: Some(input.transfer_access_policy.auth_scope),
                users: Some(input.transfer_access_policy.users),
                user_groups: Some(input.transfer_access_policy.user_groups),
                resource: Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(
                    *uuid.as_bytes(),
                ))),
            })
            .await?;

        Ok(new_account)
    }

    /// Edits the account with the given id and updates the associated policies if provided.
    ///
    /// This operation will fail if an account owner does not have an associated user.
    pub async fn edit_account(&self, input: EditAccountOperationInput) -> ServiceResult<Account> {
        let mut account = self.get_account(&input.account_id)?;

        if let Some(name) = &input.name {
            account.name = name.to_owned();

            if self
                .account_repository
                .find_account_id_by_name(name)
                .is_some_and(|id| id != account.id)
            {
                Err(AccountError::AccountNameAlreadyExists)?
            }
        }

        if let Some(transfer_approval_policy_input) = input.transfer_approval_policy {
            self.proposal_policy_service
                .handle_policy_change(
                    ProposalSpecifier::Transfer(ResourceIds::Ids(vec![account.id])),
                    transfer_approval_policy_input,
                    &mut account.transfer_approval_policy_id,
                )
                .await?;
        }

        if let Some(update_approval_policy_input) = input.update_approval_policy {
            self.proposal_policy_service
                .handle_policy_change(
                    ProposalSpecifier::EditAccount(ResourceIds::Ids(vec![account.id])),
                    update_approval_policy_input,
                    &mut account.update_approval_policy_id,
                )
                .await?;
        }

        account.validate()?;

        account.last_modification_timestamp = next_time();
        self.account_repository
            .insert(account.to_key(), account.to_owned());

        // Updates the access policies for the account.
        if let Some(read_access_policy) = input.read_access_policy {
            self.access_policy_service
                .edit_access_policy(EditAccessPolicyOperationInput {
                    auth_scope: Some(read_access_policy.auth_scope),
                    users: Some(read_access_policy.users),
                    user_groups: Some(read_access_policy.user_groups),
                    resource: Resource::Account(AccountResourceAction::Read(ResourceId::Id(
                        account.id,
                    ))),
                })
                .await?;
        }

        if let Some(update_access_policy) = input.update_access_policy {
            self.access_policy_service
                .edit_access_policy(EditAccessPolicyOperationInput {
                    auth_scope: Some(update_access_policy.auth_scope),
                    users: Some(update_access_policy.users),
                    user_groups: Some(update_access_policy.user_groups),
                    resource: Resource::Account(AccountResourceAction::Update(ResourceId::Id(
                        account.id,
                    ))),
                })
                .await?;
        }

        if let Some(transfer_access_policy) = input.transfer_access_policy {
            self.access_policy_service
                .edit_access_policy(EditAccessPolicyOperationInput {
                    auth_scope: Some(transfer_access_policy.auth_scope),
                    users: Some(transfer_access_policy.users),
                    user_groups: Some(transfer_access_policy.user_groups),
                    resource: Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(
                        account.id,
                    ))),
                })
                .await?;
        }

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
                    let balance_age_ns = next_time() - balance.last_modification_timestamp;
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
                        last_modification_timestamp: next_time(),
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
            access_policy::Allow, account_test_utils::mock_account, criteria::Criteria,
            user_test_utils::mock_user, AddAccountOperation, AddAccountOperationInput, Blockchain,
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
        test_utils::init_canister_system();

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
        let account = mock_account();

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
                blockchain: Blockchain::InternetComputer,
                standard: BlockchainStandard::Native,
                metadata: Metadata::default(),
                read_access_policy: Allow::users(vec![ctx.caller_user.id]),
                update_access_policy: Allow::users(vec![ctx.caller_user.id]),
                transfer_access_policy: Allow::users(vec![ctx.caller_user.id]),
                update_approval_policy: Some(Criteria::AutoAdopted),
                transfer_approval_policy: Some(Criteria::AutoAdopted),
            },
        };

        let result = ctx.service.create_account(operation.input).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn add_account_with_existing_name_should_fail() {
        let ctx = setup();
        let mut account = mock_account();
        account.name = "foo".to_string();

        ctx.repository.insert(account.to_key(), account.clone());

        let operation = AddAccountOperation {
            account_id: None,
            input: AddAccountOperationInput {
                name: account.name,
                blockchain: Blockchain::InternetComputer,
                standard: BlockchainStandard::Native,
                metadata: Metadata::default(),
                read_access_policy: Allow::users(vec![ctx.caller_user.id]),
                update_access_policy: Allow::users(vec![ctx.caller_user.id]),
                transfer_access_policy: Allow::users(vec![ctx.caller_user.id]),
                update_approval_policy: Some(Criteria::AutoAdopted),
                transfer_approval_policy: Some(Criteria::AutoAdopted),
            },
        };

        let result = ctx.service.create_account(operation.input).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn add_account_with_missing_policy_should_fail() {
        let ctx = setup();

        let operation = AddAccountOperation {
            account_id: None,
            input: AddAccountOperationInput {
                name: "test".to_string(),
                blockchain: Blockchain::InternetComputer,
                standard: BlockchainStandard::Native,
                metadata: Metadata::default(),
                read_access_policy: Allow::users(vec![ctx.caller_user.id]),
                update_access_policy: Allow::users(vec![ctx.caller_user.id]),
                transfer_access_policy: Allow::users(vec![ctx.caller_user.id]),
                update_approval_policy: Some(Criteria::AutoAdopted),
                transfer_approval_policy: Some(Criteria::AutoAdopted),
            },
        };

        let result = ctx.service.create_account(operation.input).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn edit_account() {
        let ctx = setup();
        let account = mock_account();

        ctx.repository.insert(account.to_key(), account.clone());

        let operation = EditAccountOperationInput {
            account_id: account.id,
            name: Some("test_edit".to_string()),
            read_access_policy: None,
            transfer_access_policy: None,
            update_access_policy: None,
            transfer_approval_policy: None,
            update_approval_policy: None,
        };

        let result = ctx.service.edit_account(operation).await;

        assert!(result.is_ok());

        let updated_account = result.unwrap();

        assert_eq!(updated_account.name, "test_edit");
    }

    #[tokio::test]
    async fn edit_account_with_duplicate_name_should_fail() {
        let ctx = setup();
        let mut account = mock_account();
        account.name = "foo".to_string();
        let mut second_account = mock_account();
        second_account.name = "bar".to_string();

        ACCOUNT_REPOSITORY.insert(account.to_key(), account.clone());
        ACCOUNT_REPOSITORY.insert(second_account.to_key(), second_account.clone());

        let operation = EditAccountOperationInput {
            account_id: account.id,
            name: Some("bar".to_string()),
            read_access_policy: None,
            transfer_access_policy: None,
            update_access_policy: None,
            transfer_approval_policy: None,
            update_approval_policy: None,
        };

        let result = ctx.service.edit_account(operation).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn fail_create_account_invalid_blockchain_standard() {
        let ctx = setup();
        let operation = AddAccountOperation {
            account_id: None,
            input: AddAccountOperationInput {
                name: "foo".to_string(),
                blockchain: Blockchain::InternetComputer,
                standard: BlockchainStandard::ERC20,
                metadata: Metadata::default(),
                read_access_policy: Allow::users(vec![ctx.caller_user.id]),
                update_access_policy: Allow::users(vec![ctx.caller_user.id]),
                transfer_access_policy: Allow::users(vec![ctx.caller_user.id]),
                update_approval_policy: Some(Criteria::AutoAdopted),
                transfer_approval_policy: Some(Criteria::AutoAdopted),
            },
        };

        let result = ctx.service.create_account(operation.input).await;

        assert!(result.is_err());
    }
}
