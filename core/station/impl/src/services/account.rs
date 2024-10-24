use crate::{
    core::{
        authorization::Authorization,
        generate_uuid_v4,
        ic_cdk::next_time,
        read_system_info,
        utils::{paginated_items, retain_accessible_resources, PaginatedData, PaginatedItemsArgs},
        write_system_info, CallContext, ACCOUNT_BALANCE_FRESHNESS_IN_MS,
    },
    errors::AccountError,
    factories::blockchains::BlockchainApiFactory,
    mappers::{account::AccountMapper, HelperMapper},
    models::{
        request_policy_rule::RequestPolicyRuleInput,
        request_specifier::RequestSpecifier,
        resource::{AccountResourceAction, Resource, ResourceId, ResourceIds},
        Account, AccountAddress, AccountBalance, AccountCallerPrivileges, AccountId,
        AddAccountOperationInput, AddRequestPolicyOperationInput, AddressFormat, Blockchain,
        CycleObtainStrategy, EditAccountOperationInput, EditPermissionOperationInput, MetadataItem,
        TokenStandard,
    },
    repositories::{
        AccountRepository, AccountWhereClause, AssetRepository, ACCOUNT_REPOSITORY,
        ASSET_REPOSITORY,
    },
    services::{
        permission::{PermissionService, PERMISSION_SERVICE},
        RequestPolicyService, REQUEST_POLICY_SERVICE,
    },
};
use ic_ledger_types::MAINNET_LEDGER_CANISTER_ID;
use lazy_static::lazy_static;
use orbit_essentials::{
    api::ServiceResult, model::ModelValidator, repository::Repository, types::UUID,
};
use station_api::{AccountBalanceDTO, FetchAccountBalancesInput, ListAccountsInput};
use std::{collections::HashSet, sync::Arc};
use uuid::Uuid;

use super::SYSTEM_SERVICE;

lazy_static! {
    pub static ref ACCOUNT_SERVICE: Arc<AccountService> = Arc::new(AccountService::new(
        Arc::clone(&REQUEST_POLICY_SERVICE),
        Arc::clone(&PERMISSION_SERVICE),
        Arc::clone(&ACCOUNT_REPOSITORY),
        Arc::clone(&ASSET_REPOSITORY)
    ));
}

#[derive(Default, Debug)]
pub struct AccountService {
    request_policy_service: Arc<RequestPolicyService>,
    permission_service: Arc<PermissionService>,
    account_repository: Arc<AccountRepository>,
    asset_repository: Arc<AssetRepository>,
}

impl AccountService {
    const DEFAULT_ACCOUNT_LIST_LIMIT: u16 = 50;
    const MAX_ACCOUNT_LIST_LIMIT: u16 = 1000;

    pub fn new(
        request_policy_service: Arc<RequestPolicyService>,
        permission_service: Arc<PermissionService>,
        account_repository: Arc<AccountRepository>,
        asset_repository: Arc<AssetRepository>,
    ) -> Self {
        Self {
            request_policy_service,
            permission_service,
            account_repository,
            asset_repository,
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
    pub async fn create_account(
        &self,
        input: AddAccountOperationInput,
        with_account_id: Option<UUID>,
    ) -> ServiceResult<Account> {
        if self.account_repository.find_by_name(&input.name).is_some() {
            Err(AccountError::AccountNameAlreadyExists)?
        }

        let uuid = match with_account_id {
            Some(id) => Uuid::from_bytes(id),
            None => generate_uuid_v4().await,
        };
        let key = Account::key(*uuid.as_bytes());
        if self.account_repository.get(&key).is_some() {
            Err(AccountError::ValidationError {
                info: format!("Account with id {} already exists", uuid.hyphenated()),
            })?
        }

        let mut new_account =
            AccountMapper::from_create_input(input.to_owned(), *uuid.as_bytes(), None)?;

        let dedulicated_asset_ids = input.assets.iter().cloned().collect::<HashSet<_>>();

        for asset_id in dedulicated_asset_ids.iter() {
            let asset = self.asset_repository.get(asset_id).ok_or_else(|| {
                AccountError::ValidationError {
                    info: format!(
                        "Asset with id {} not found",
                        Uuid::from_bytes(*asset_id).hyphenated()
                    ),
                }
            })?;

            for standard in asset.standards.iter() {
                let blockchain_api = BlockchainApiFactory::build(&asset.blockchain.clone())?;

                let mut account_addresses = Vec::<AccountAddress>::new();

                for address_format in standard.get_info().address_formats.into_iter() {
                    if account_addresses
                        .iter()
                        .any(|address| address.format == address_format)
                        || new_account
                            .addresses
                            .iter()
                            .any(|address| address.format == address_format)
                    {
                        // the account already has this address
                        continue;
                    }

                    let address = blockchain_api
                        .generate_address(&new_account.seed, address_format.clone())
                        .await?;

                    account_addresses.push(address);
                }

                new_account.addresses.extend(account_addresses);
            }
        }

        if let Some(criteria) = &input.transfer_request_policy {
            criteria.validate()?;
        };
        if let Some(criteria) = &input.configs_request_policy {
            criteria.validate()?;
        };

        input.read_permission.validate()?;
        input.configs_permission.validate()?;
        input.transfer_permission.validate()?;

        // Validate here before database operations.
        new_account.validate()?;

        // Insert the account into the repository already to avoid subsequent policy validators erroring
        // out with invalid request specifier.
        self.account_repository
            .insert(key.clone(), new_account.clone());

        // adds the associated transfer policy based on the transfer criteria
        if let Some(policy_rule) = &input.transfer_request_policy {
            let transfer_request_policy =
                self.request_policy_service
                    .add_request_policy(AddRequestPolicyOperationInput {
                        specifier: RequestSpecifier::Transfer(ResourceIds::Ids(vec![
                            *uuid.as_bytes()
                        ])),
                        rule: policy_rule.clone(),
                    })?;

            new_account.transfer_request_policy_id = Some(transfer_request_policy.id);
        }

        // adds the associated edit policy based on the edit criteria
        if let Some(policy_rule) = &input.configs_request_policy {
            let configs_request_policy =
                self.request_policy_service
                    .add_request_policy(AddRequestPolicyOperationInput {
                        specifier: RequestSpecifier::EditAccount(ResourceIds::Ids(vec![
                            *uuid.as_bytes()
                        ])),
                        rule: policy_rule.to_owned(),
                    })?;

            new_account.configs_request_policy_id = Some(configs_request_policy.id);
        }

        // Inserting the account into the repository and its associations is the last step of the account creation
        // process to avoid potential consistency issues due to the fact that some of the calls to create the account
        // happen in an asynchronous way.
        self.account_repository.insert(key, new_account.clone());

        // Adds the access policies for the account.
        self.permission_service
            .edit_permission(EditPermissionOperationInput {
                auth_scope: Some(input.read_permission.auth_scope),
                users: Some(input.read_permission.users),
                user_groups: Some(input.read_permission.user_groups),
                resource: Resource::Account(AccountResourceAction::Read(ResourceId::Id(
                    *uuid.as_bytes(),
                ))),
            })?;

        self.permission_service
            .edit_permission(EditPermissionOperationInput {
                auth_scope: Some(input.configs_permission.auth_scope),
                users: Some(input.configs_permission.users),
                user_groups: Some(input.configs_permission.user_groups),
                resource: Resource::Account(AccountResourceAction::Update(ResourceId::Id(
                    *uuid.as_bytes(),
                ))),
            })?;

        self.permission_service
            .edit_permission(EditPermissionOperationInput {
                auth_scope: Some(input.transfer_permission.auth_scope),
                users: Some(input.transfer_permission.users),
                user_groups: Some(input.transfer_permission.user_groups),
                resource: Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(
                    *uuid.as_bytes(),
                ))),
            })?;

        if SYSTEM_SERVICE.is_healthy() {
            let mut system_info = read_system_info();

            // if this is the first account created, and there is no cycle minting account set, set this account as the cycle minting account
            if system_info.get_cycle_obtain_strategy() == &CycleObtainStrategy::Disabled
                && ACCOUNT_REPOSITORY.len() == 1
            {
                // find the mainnet ICP asset that minting can be done from
                if let Some(icp_asset) = self.asset_repository.list().iter().find(|asset| {
                    asset.blockchain == Blockchain::InternetComputer
                        && asset
                            .standards
                            .contains(&TokenStandard::InternetComputerNative)
                        && asset.metadata.contains(&MetadataItem {
                            key: TokenStandard::METADATA_KEY_LEDGER_CANISTER_ID.to_string(),
                            value: MAINNET_LEDGER_CANISTER_ID.to_string(),
                        })
                }) {
                    // check if the new account has the ICP asset
                    if new_account
                        .assets
                        .iter()
                        .any(|account_asset| account_asset.asset_id == icp_asset.id)
                    {
                        ic_cdk::println!("Setting cycle minting account to {}", uuid);

                        system_info.set_cycle_obtain_strategy(
                            CycleObtainStrategy::MintFromNativeToken {
                                account_id: *uuid.as_bytes(),
                            },
                        );
                        write_system_info(system_info);

                        #[cfg(target_arch = "wasm32")]
                        crate::services::SYSTEM_SERVICE.set_fund_manager_obtain_cycles(
                            &CycleObtainStrategy::MintFromNativeToken {
                                account_id: new_account.id,
                            },
                        );
                    }
                }
            }
        }

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
                .find_by_name(name)
                .is_some_and(|id| id != account.id)
            {
                Err(AccountError::AccountNameAlreadyExists)?
            }
        }

        if let Some(change_assets) = input.change_assets {
            change_assets.apply(&mut account.assets);

            // get all supported address formats of the account
            let mut current_address_formats: HashSet<(Blockchain, AddressFormat)> = HashSet::new();

            for account_asset in account.assets.iter() {
                let Some(asset) = self.asset_repository.get(&account_asset.asset_id) else {
                    ic_cdk::println!(
                        "Asset `{}` does not exist in account `{}`",
                        Uuid::from_bytes(account_asset.asset_id).hyphenated(),
                        Uuid::from_bytes(account.id).hyphenated()
                    );
                    continue;
                };

                for standard in asset.standards.iter() {
                    standard.get_info().address_formats.iter().for_each(|f| {
                        current_address_formats.insert((asset.blockchain.clone(), f.to_owned()));
                    });
                }
            }

            // remove addresses which don't belong to any account_assets any more
            account.addresses.retain(|account_address| {
                current_address_formats
                    .iter()
                    .any(|(_, format)| &account_address.format == format)
            });

            for (blockchain, address_format) in current_address_formats {
                if account
                    .addresses
                    .iter()
                    .any(|address| address.format == address_format)
                {
                    // the account already has this address
                    continue;
                }

                let blockchain_api = BlockchainApiFactory::build(&blockchain)?;

                let address = blockchain_api
                    .generate_address(&account.seed, address_format.clone())
                    .await?;

                account.addresses.push(address);
            }
        }

        if let Some(RequestPolicyRuleInput::Set(criteria)) = &input.transfer_request_policy {
            criteria.validate()?;
        };
        if let Some(RequestPolicyRuleInput::Set(criteria)) = &input.configs_request_policy {
            criteria.validate()?;
        };
        if let Some(permission) = &input.read_permission {
            permission.validate()?;
        };
        if let Some(permission) = &input.configs_permission {
            permission.validate()?;
        };
        if let Some(permission) = &input.transfer_permission {
            permission.validate()?;
        };

        if let Some(transfer_request_policy_input) = input.transfer_request_policy {
            self.request_policy_service.handle_policy_change(
                RequestSpecifier::Transfer(ResourceIds::Ids(vec![account.id])),
                transfer_request_policy_input,
                &mut account.transfer_request_policy_id,
            )?;
        }

        if let Some(configs_request_policy_input) = input.configs_request_policy {
            self.request_policy_service.handle_policy_change(
                RequestSpecifier::EditAccount(ResourceIds::Ids(vec![account.id])),
                configs_request_policy_input,
                &mut account.configs_request_policy_id,
            )?;
        }

        account.validate()?;

        account.last_modification_timestamp = next_time();
        self.account_repository
            .insert(account.to_key(), account.to_owned());

        // Updates the access policies for the account.
        if let Some(read_permission) = input.read_permission {
            self.permission_service
                .edit_permission(EditPermissionOperationInput {
                    auth_scope: Some(read_permission.auth_scope),
                    users: Some(read_permission.users),
                    user_groups: Some(read_permission.user_groups),
                    resource: Resource::Account(AccountResourceAction::Read(ResourceId::Id(
                        account.id,
                    ))),
                })?;
        }

        if let Some(configs_permission) = input.configs_permission {
            self.permission_service
                .edit_permission(EditPermissionOperationInput {
                    auth_scope: Some(configs_permission.auth_scope),
                    users: Some(configs_permission.users),
                    user_groups: Some(configs_permission.user_groups),
                    resource: Resource::Account(AccountResourceAction::Update(ResourceId::Id(
                        account.id,
                    ))),
                })?;
        }

        if let Some(transfer_permission) = input.transfer_permission {
            self.permission_service
                .edit_permission(EditPermissionOperationInput {
                    auth_scope: Some(transfer_permission.auth_scope),
                    users: Some(transfer_permission.users),
                    user_groups: Some(transfer_permission.user_groups),
                    resource: Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(
                        account.id,
                    ))),
                })?;
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
            .collect::<Result<HashSet<Uuid>, _>>()?;

        let accounts = self
            .account_repository
            .find_by_ids(account_ids.iter().map(|id| *id.as_bytes()).collect());

        let mut balances = Vec::new();
        for mut account in accounts {
            let account_key = account.to_key();
            for account_asset in account.assets.iter_mut() {
                let balance_considered_fresh = match &account_asset.balance {
                    Some(balance) => {
                        let balance_age_ns = next_time() - balance.last_modification_timestamp;
                        (balance_age_ns / 1_000_000) < ACCOUNT_BALANCE_FRESHNESS_IN_MS
                    }
                    None => false,
                };

                let Some(asset) = self.asset_repository.get(&account_asset.asset_id) else {
                    continue;
                };

                let balance: AccountBalance =
                    match (&account_asset.balance, balance_considered_fresh) {
                        (None, _) | (_, false) => {
                            let blockchain_api = BlockchainApiFactory::build(&asset.blockchain)?;
                            let fetched_balance =
                                blockchain_api.balance(&asset, &account.addresses).await?;
                            let new_balance = AccountBalance {
                                balance: candid::Nat(fetched_balance),
                                last_modification_timestamp: next_time(),
                            };

                            account_asset.balance = Some(new_balance.clone());

                            new_balance
                        }
                        (Some(balance), _) => balance.to_owned(),
                    };

                balances.push(AccountMapper::to_balance_dto(
                    balance,
                    asset.decimals,
                    account.id,
                ));
            }

            self.account_repository.insert(account_key.clone(), account);
        }

        Ok(balances)
    }
}

#[cfg(test)]
mod tests {
    use candid::Principal;
    use orbit_essentials::model::ModelKey;

    use super::*;
    use crate::{
        core::{test_utils, validation::disable_mock_resource_validation, CallContext},
        models::{
            account_test_utils::mock_account,
            asset_test_utils::{mock_asset, mock_asset_b},
            permission::Allow,
            request_policy_rule::RequestPolicyRule,
            request_specifier::UserSpecifier,
            user_test_utils::mock_user,
            AddAccountOperation, AddAccountOperationInput, ChangeAssets, Metadata, User,
        },
        repositories::UserRepository,
        services::ASSET_SERVICE,
    };

    struct TestContext {
        repository: AccountRepository,
        asset_repository: AssetRepository,
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
            asset_repository: AssetRepository::default(),
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
                assets: vec![],
                metadata: Metadata::default(),
                read_permission: Allow::users(vec![ctx.caller_user.id]),
                configs_permission: Allow::users(vec![ctx.caller_user.id]),
                transfer_permission: Allow::users(vec![ctx.caller_user.id]),
                configs_request_policy: Some(RequestPolicyRule::AutoApproved),
                transfer_request_policy: Some(RequestPolicyRule::AutoApproved),
            },
        };

        let result = ctx.service.create_account(operation.input, None).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn add_account_with_existing_name_should_fail() {
        let ctx = setup();

        disable_mock_resource_validation();

        let mut account = mock_account();
        account.name = "foo".to_string();

        ctx.repository.insert(account.to_key(), account.clone());

        let operation = AddAccountOperation {
            account_id: None,
            input: AddAccountOperationInput {
                name: account.name,
                assets: vec![],
                metadata: Metadata::default(),
                read_permission: Allow::users(vec![ctx.caller_user.id]),
                configs_permission: Allow::users(vec![ctx.caller_user.id]),
                transfer_permission: Allow::users(vec![ctx.caller_user.id]),
                configs_request_policy: Some(RequestPolicyRule::AutoApproved),
                transfer_request_policy: Some(RequestPolicyRule::AutoApproved),
            },
        };

        let result = ctx.service.create_account(operation.input, None).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn add_account_with_missing_policy_should_fail() {
        let ctx = setup();

        disable_mock_resource_validation();

        let base_input = AddAccountOperationInput {
            name: "foo".to_string(),
            assets: vec![],
            metadata: Metadata::default(),
            read_permission: Allow::users(vec![ctx.caller_user.id]),
            configs_permission: Allow::users(vec![ctx.caller_user.id]),
            transfer_permission: Allow::users(vec![ctx.caller_user.id]),
            configs_request_policy: Some(RequestPolicyRule::AutoApproved),
            transfer_request_policy: Some(RequestPolicyRule::AutoApproved),
        };

        assert!(ctx
            .service
            .create_account(base_input.clone(), None)
            .await
            .is_ok());

        ctx.service
            .create_account(
                AddAccountOperationInput {
                    read_permission: Allow::users(vec![[5; 16]]),
                    ..base_input.clone()
                },
                None,
            )
            .await
            .expect_err("read_permission should be invalid");

        ctx.service
            .create_account(
                AddAccountOperationInput {
                    configs_permission: Allow::users(vec![[5; 16]]),
                    ..base_input.clone()
                },
                None,
            )
            .await
            .expect_err("configs_permission should be invalid");

        ctx.service
            .create_account(
                AddAccountOperationInput {
                    transfer_permission: Allow::users(vec![[5; 16]]),
                    ..base_input.clone()
                },
                None,
            )
            .await
            .expect_err("transfer_permission should be invalid");

        ctx.service
            .create_account(
                AddAccountOperationInput {
                    configs_request_policy: Some(RequestPolicyRule::Quorum(
                        UserSpecifier::Id(vec![[5; 16]]),
                        1,
                    )),
                    ..base_input.clone()
                },
                None,
            )
            .await
            .expect_err("configs_request_policy should be invalid");

        ctx.service
            .create_account(
                AddAccountOperationInput {
                    transfer_request_policy: Some(RequestPolicyRule::Quorum(
                        UserSpecifier::Id(vec![[5; 16]]),
                        1,
                    )),
                    ..base_input.clone()
                },
                None,
            )
            .await
            .expect_err("transfer_request_policy should be invalid");
    }

    #[tokio::test]
    async fn add_account_with_same_id_should_fail() {
        let ctx = setup();
        let account = mock_account();

        ctx.repository.insert(account.to_key(), account.clone());

        let input = AddAccountOperationInput {
            name: "foo2".to_string(),
            assets: vec![],
            metadata: Metadata::default(),
            read_permission: Allow::users(vec![ctx.caller_user.id]),
            configs_permission: Allow::users(vec![ctx.caller_user.id]),
            transfer_permission: Allow::users(vec![ctx.caller_user.id]),
            configs_request_policy: Some(RequestPolicyRule::AutoApproved),
            transfer_request_policy: Some(RequestPolicyRule::AutoApproved),
        };

        let result = ctx.service.create_account(input, Some(account.id)).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_json_string().contains(&format!(
            "Account with id {} already exists",
            Uuid::from_bytes(account.id).hyphenated()
        )));
    }

    #[tokio::test]
    async fn edit_account() {
        let ctx = setup();
        let account = mock_account();

        ctx.repository.insert(account.to_key(), account.clone());

        let operation = EditAccountOperationInput {
            account_id: account.id,
            name: Some("test_edit".to_string()),
            change_assets: None,
            read_permission: None,
            transfer_permission: None,
            configs_permission: None,
            transfer_request_policy: None,
            configs_request_policy: None,
        };

        let result = ctx.service.edit_account(operation).await;

        assert!(result.is_ok());

        let updated_account = result.unwrap();

        assert_eq!(updated_account.name, "test_edit");
    }

    #[tokio::test]
    async fn edit_account_assets() {
        let ctx = setup();

        let asset_a = mock_asset();
        ctx.asset_repository.insert(asset_a.key(), asset_a.clone());

        let asset_b = mock_asset_b();
        ctx.asset_repository.insert(asset_b.key(), asset_b.clone());

        let mut account = mock_account();
        account.assets = vec![];
        account.addresses = vec![];
        ctx.repository.insert(account.to_key(), account.clone());

        let operation = EditAccountOperationInput {
            account_id: account.id,
            name: None,
            change_assets: Some(ChangeAssets::Change {
                add_assets: vec![asset_a.id],
                remove_assets: vec![],
            }),
            read_permission: None,
            transfer_permission: None,
            configs_permission: None,
            transfer_request_policy: None,
            configs_request_policy: None,
        };

        let updated_account = ctx
            .service
            .edit_account(operation)
            .await
            .expect("edit account should be successful");
        assert_eq!(updated_account.assets.len(), 1);
        assert_eq!(updated_account.assets[0].asset_id, asset_a.id);

        let operation = EditAccountOperationInput {
            account_id: account.id,
            name: None,
            change_assets: Some(ChangeAssets::Change {
                add_assets: vec![asset_b.id],
                remove_assets: vec![asset_a.id],
            }),
            read_permission: None,
            transfer_permission: None,
            configs_permission: None,
            transfer_request_policy: None,
            configs_request_policy: None,
        };

        let updated_account = ctx
            .service
            .edit_account(operation)
            .await
            .expect("edit account should be successful");
        assert_eq!(updated_account.assets.len(), 1);
        assert_eq!(updated_account.assets[0].asset_id, asset_b.id);

        let operation = EditAccountOperationInput {
            account_id: account.id,
            name: None,
            change_assets: Some(ChangeAssets::ReplaceWith {
                assets: vec![asset_a.id, asset_b.id],
            }),
            read_permission: None,
            transfer_permission: None,
            configs_permission: None,
            transfer_request_policy: None,
            configs_request_policy: None,
        };

        let updated_account = ctx
            .service
            .edit_account(operation)
            .await
            .expect("edit account should be successful");

        assert_eq!(updated_account.assets.len(), 2);
        assert_eq!(updated_account.assets[0].asset_id, asset_a.id);
        assert_eq!(updated_account.assets[1].asset_id, asset_b.id);
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
            change_assets: None,
            read_permission: None,
            transfer_permission: None,
            configs_permission: None,
            transfer_request_policy: None,
            configs_request_policy: None,
        };

        let result = ctx.service.edit_account(operation).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn edit_account_with_missing_policy_should_fail() {
        let ctx = setup();

        disable_mock_resource_validation();

        let account = mock_account();

        let asset = mock_asset();
        ASSET_REPOSITORY.insert(asset.key(), asset.clone());

        ctx.repository.insert(account.to_key(), account.clone());

        let base_input = EditAccountOperationInput {
            account_id: account.id,
            name: Some("test_edit".to_string()),
            change_assets: None,
            read_permission: None,
            transfer_permission: None,
            configs_permission: None,
            transfer_request_policy: None,
            configs_request_policy: None,
        };

        assert!(ctx.service.edit_account(base_input.clone()).await.is_ok());

        ctx.service
            .edit_account(EditAccountOperationInput {
                read_permission: Some(Allow::users(vec![[5; 16]])),
                ..base_input.clone()
            })
            .await
            .expect_err("read_permission should be invalid");

        ctx.service
            .edit_account(EditAccountOperationInput {
                configs_permission: Some(Allow::users(vec![[5; 16]])),
                ..base_input.clone()
            })
            .await
            .expect_err("configs_permission should be invalid");

        ctx.service
            .edit_account(EditAccountOperationInput {
                transfer_permission: Some(Allow::users(vec![[5; 16]])),
                ..base_input.clone()
            })
            .await
            .expect_err("transfer_permission should be invalid");

        ctx.service
            .edit_account(EditAccountOperationInput {
                configs_request_policy: Some(RequestPolicyRuleInput::Set(
                    RequestPolicyRule::Quorum(UserSpecifier::Id(vec![[5; 16]]), 1),
                )),
                ..base_input.clone()
            })
            .await
            .expect_err("configs_request_policy should be invalid");

        ctx.service
            .edit_account(EditAccountOperationInput {
                transfer_request_policy: Some(RequestPolicyRuleInput::Set(
                    RequestPolicyRule::Quorum(UserSpecifier::Id(vec![[5; 16]]), 1),
                )),
                ..base_input.clone()
            })
            .await
            .expect_err("transfer_request_policy should be invalid");
    }

    #[tokio::test]
    async fn can_add_icrc1_asset() {
        disable_mock_resource_validation();

        let asset = ASSET_SERVICE
            .create(
                crate::models::AddAssetOperationInput {
                    name: "Test ICRC1 token".to_owned(),
                    symbol: "TEST".to_owned(),
                    decimals: 4,
                    metadata: Metadata::default(),
                    blockchain: Blockchain::InternetComputer,
                    standards: vec![TokenStandard::ICRC1],
                },
                None,
            )
            .expect("asset creation should be successful");

        ACCOUNT_SERVICE
            .create_account(
                AddAccountOperationInput {
                    name: "Test account".to_owned(),
                    assets: vec![asset.id],
                    metadata: Metadata::default(),
                    read_permission: Allow::authenticated(),
                    configs_permission: Allow::authenticated(),
                    transfer_permission: Allow::authenticated(),
                    configs_request_policy: Some(RequestPolicyRule::AutoApproved),
                    transfer_request_policy: Some(RequestPolicyRule::AutoApproved),
                },
                None,
            )
            .await
            .expect("account creation should be successful");
    }
}
