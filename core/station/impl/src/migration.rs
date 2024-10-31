use crate::core::ic_cdk::api::trap;
use crate::core::{read_system_info, write_system_info, Memory};
use crate::factories::blockchains::InternetComputer;
use crate::models::permission::{Allow, AuthScope};
use crate::models::request_specifier::RequestSpecifier;
use crate::models::resource::{Resource, SystemResourceAction};
use crate::models::resource::{ResourceAction, ResourceId, ResourceIds};
use crate::models::{
    Account, AccountAddress, AccountAsset, AccountBalance, AccountId, AccountKey, AccountSeed,
    AddAccountOperationInput, AddAddressBookEntryOperationInput, AddRequestPolicyOperationInput,
    AddressBookEntry, AddressBookEntryId, AddressBookEntryKey, AddressFormat, Asset, AssetId,
    Blockchain, EditPermissionOperationInput, Metadata, MetadataItem, Request, RequestKey,
    RequestPolicyRule, TokenStandard, Transfer, TransferId, TransferKey, TransferOperation,
    TransferOperationInput, TransferStatus, UserId,
};
use crate::repositories::permission::PERMISSION_REPOSITORY;
use crate::repositories::{
    AccountRepository, AddressBookRepository, RequestRepository, TransferRepository,
    REQUEST_POLICY_REPOSITORY, USER_GROUP_REPOSITORY, USER_REPOSITORY,
};
use crate::repositories::{
    ACCOUNT_REPOSITORY, ADDRESS_BOOK_REPOSITORY, ASSET_REPOSITORY, REQUEST_REPOSITORY,
    TRANSFER_REPOSITORY,
};
use crate::services::permission::PERMISSION_SERVICE;
use crate::services::{INITIAL_ICP_ASSET, INITIAL_ICP_ASSET_ID, REQUEST_POLICY_SERVICE};
use crate::STABLE_MEMORY_VERSION;
use ic_stable_structures::memory_manager::VirtualMemory;
use orbit_essentials::model::ModelKey;
use orbit_essentials::repository::{RebuildRepository, Repository};
use orbit_essentials::types::{Timestamp, UUID};
use serde::{Deserialize, Deserializer};

/// Handles stable memory schema migrations for the station canister.
///
/// Stable memory migration conditions:
///
/// - The migration is only applied once per each version.
/// - Stable memory versions can't be skipped, the upgrade must be sequential.
/// - The migration is applied is the previous version is `STABLE_MEMORY_VERSION - 1`.
pub struct MigrationHandler;

impl MigrationHandler {
    /// Run migrations for the station canister to ensure the stable memory schema is up-to-date.
    ///
    /// WARNING: This needs to be run before any other access to stable memory happens.
    pub fn run() {
        let mut system_info = read_system_info();
        let stored_version = system_info.get_stable_memory_version();

        if stored_version == STABLE_MEMORY_VERSION {
            // Run the post-run checks that need to be run on every upgrade.
            post_run();
            return;
        }

        if stored_version > STABLE_MEMORY_VERSION {
            trap(&format!(
                "Cannot downgrade the station from memory layout version {} to {}",
                stored_version, STABLE_MEMORY_VERSION
            ));
        }

        if stored_version != STABLE_MEMORY_VERSION - 1 {
            trap(&format!(
                "Cannot skip upgrades between station memory layout version {} to {}",
                stored_version, STABLE_MEMORY_VERSION
            ));
        }

        apply_migration();

        // Update the stable memory version to the latest version.
        system_info.set_stable_memory_version(STABLE_MEMORY_VERSION);
        write_system_info(system_info);

        // Run the post-run checks that need to be run on every upgrade.
        post_run();
    }
}

/// If there is a check that needs to be run on every upgrade, regardless if the memory version has changed,
/// it should be added here.
fn post_run() {}

/// The migration to apply to the station canister stable memory.
///
/// Please include the migration steps in the `apply_migration` function.
fn apply_migration() {
    // add new asset permissions: resources available to all users
    let public_resources = [
        Resource::Asset(ResourceAction::List),
        Resource::Asset(ResourceAction::Read(ResourceId::Any)),
    ];

    // build cache so that model validation can pass
    USER_GROUP_REPOSITORY.build_cache();
    USER_REPOSITORY.build_cache();
    PERMISSION_REPOSITORY.build_cache();

    for resource in public_resources {
        let _ = PERMISSION_SERVICE.edit_permission(EditPermissionOperationInput {
            resource,
            auth_scope: Some(AuthScope::Authenticated),
            user_groups: None,
            users: None,
        });
    }

    // add new asset permissions: inherit config from ManageSystemInfo
    let manage_system_info_permissions_allow = PERMISSION_SERVICE
        .get_permission(&Resource::System(SystemResourceAction::ManageSystemInfo))
        .allow;

    let sensitive_resources = [
        Resource::Asset(ResourceAction::Create),
        Resource::Asset(ResourceAction::Update(ResourceId::Any)),
        Resource::Asset(ResourceAction::Delete(ResourceId::Any)),
    ];

    for resource in sensitive_resources {
        if let Err(err) = PERMISSION_SERVICE.edit_permission(EditPermissionOperationInput {
            resource,
            auth_scope: Some(manage_system_info_permissions_allow.auth_scope.clone()),
            user_groups: Some(manage_system_info_permissions_allow.user_groups.clone()),
            users: Some(manage_system_info_permissions_allow.users.clone()),
        }) {
            ic_cdk::println!("Failed to create new asset permission: {:?}", err);
        }
    }

    // add new asset policies
    let policy_specifiers = [
        RequestSpecifier::AddAsset,
        RequestSpecifier::EditAsset(ResourceIds::Any),
        RequestSpecifier::RemoveAsset(ResourceIds::Any),
    ];

    let policies_to_copy = REQUEST_POLICY_REPOSITORY
        .find_by_resource(Resource::System(SystemResourceAction::ManageSystemInfo));

    for policy in policies_to_copy {
        for specifier in policy_specifiers.iter() {
            if let Err(err) =
                REQUEST_POLICY_SERVICE.add_request_policy(AddRequestPolicyOperationInput {
                    specifier: specifier.clone(),
                    rule: policy.rule.clone(),
                })
            {
                ic_cdk::println!("Failed to create new asset policy: {:?}", err);
            }
        }
    }

    ASSET_REPOSITORY.insert(INITIAL_ICP_ASSET.key(), INITIAL_ICP_ASSET.clone());

    // rebuild repositories to apply the changes
    ADDRESS_BOOK_REPOSITORY.rebuild();
    TRANSFER_REPOSITORY.rebuild();
    ACCOUNT_REPOSITORY.rebuild();
    REQUEST_REPOSITORY.rebuild();
}

#[cfg(test)]
thread_local! {
    pub static MIGRATED_ENTRIES: std::cell::RefCell<u64> = const { std::cell::RefCell::new(0) };

    pub static MIGRATED_ACCOUNTS: std::cell::RefCell<Vec<(UUID, String)>> = const { std::cell::RefCell::new(vec![]) };
}

#[derive(Debug, Deserialize)]
pub enum BlockchainStandard {
    Native,
    ICRC1,
    ERC20,
}

impl<'de> Deserialize<'de> for AddressBookEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct PreMigrationAddressBookEntry {
            pub id: AddressBookEntryId,
            pub address_owner: String,
            pub address: String,
            pub blockchain: Blockchain,
            pub address_format: Option<AddressFormat>,
            pub metadata: Metadata,
            #[serde(default)]
            pub labels: Vec<String>,
            pub last_modification_timestamp: Timestamp,
        }

        let pre_migration_entry = PreMigrationAddressBookEntry::deserialize(deserializer)?;

        #[cfg(test)]
        if pre_migration_entry.address_format.is_none() {
            MIGRATED_ENTRIES.with(|entries| {
                *entries.borrow_mut() += 1;
            });
        }

        Ok(AddressBookEntry {
            id: pre_migration_entry.id,
            address_owner: pre_migration_entry.address_owner,
            address: pre_migration_entry.address,
            blockchain: pre_migration_entry.blockchain,
            address_format: pre_migration_entry
                .address_format
                .unwrap_or(AddressFormat::ICPAccountIdentifier),
            metadata: pre_migration_entry.metadata,
            labels: pre_migration_entry.labels,
            last_modification_timestamp: pre_migration_entry.last_modification_timestamp,
        })
    }
}

impl<'de> Deserialize<'de> for Transfer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct PreMigrationTransfer {
            pub id: TransferId,
            pub initiator_user: UserId,
            pub from_account: AccountId,
            pub to_address: String,
            pub status: TransferStatus,
            pub amount: candid::Nat,
            pub request_id: UUID,
            pub fee: candid::Nat,
            pub blockchain_network: String,
            pub metadata: Metadata,
            pub last_modification_timestamp: Timestamp,
            pub created_timestamp: Timestamp,
            pub from_asset: Option<AssetId>,
            pub with_standard: Option<TokenStandard>,
        }

        let pre_migration_entry = PreMigrationTransfer::deserialize(deserializer)?;

        #[cfg(test)]
        if pre_migration_entry.from_asset.is_none() || pre_migration_entry.with_standard.is_none() {
            MIGRATED_ENTRIES.with(|entries| {
                *entries.borrow_mut() += 1;
            });
        }

        Ok(Transfer {
            id: pre_migration_entry.id,
            initiator_user: pre_migration_entry.initiator_user,
            from_account: pre_migration_entry.from_account,
            to_address: pre_migration_entry.to_address,
            status: pre_migration_entry.status,
            amount: pre_migration_entry.amount,
            request_id: pre_migration_entry.request_id,
            fee: pre_migration_entry.fee,
            blockchain_network: pre_migration_entry.blockchain_network,
            metadata: pre_migration_entry.metadata,
            last_modification_timestamp: pre_migration_entry.last_modification_timestamp,
            created_timestamp: pre_migration_entry.created_timestamp,
            from_asset: pre_migration_entry
                .from_asset
                .unwrap_or(INITIAL_ICP_ASSET_ID),
            with_standard: pre_migration_entry
                .with_standard
                .unwrap_or(TokenStandard::InternetComputerNative),
        })
    }
}

impl<'de> Deserialize<'de> for Account {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[allow(dead_code)]
        #[derive(Debug, Deserialize)]
        struct PreMigrationAccount {
            pub id: AccountId,
            pub name: String,
            pub metadata: Metadata,
            pub transfer_request_policy_id: Option<UUID>,
            pub configs_request_policy_id: Option<UUID>,
            pub last_modification_timestamp: Timestamp,

            // removed fields
            pub balance: Option<Option<AccountBalance>>,
            pub blockchain: Option<Blockchain>,
            pub address: Option<String>,
            pub standard: Option<BlockchainStandard>,
            pub symbol: Option<String>,
            pub decimals: Option<u32>,

            // new fields
            pub seed: Option<AccountSeed>,
            pub assets: Option<Vec<AccountAsset>>,
            pub addresses: Option<Vec<AccountAddress>>,
        }

        let pre_migration_entry = PreMigrationAccount::deserialize(deserializer)?;

        #[cfg(test)]
        if pre_migration_entry.seed.is_none() {
            MIGRATED_ENTRIES.with(|entries| {
                *entries.borrow_mut() += 1;
            });

            if let Some(address) = &pre_migration_entry.address {
                MIGRATED_ACCOUNTS.with(|accounts| {
                    accounts
                        .borrow_mut()
                        .push((pre_migration_entry.id, address.clone()));
                });
            }
        }

        let seed = pre_migration_entry.seed.unwrap_or(pre_migration_entry.id);

        Ok(Account {
            id: pre_migration_entry.id,
            name: pre_migration_entry.name,
            metadata: pre_migration_entry.metadata,
            transfer_request_policy_id: pre_migration_entry.transfer_request_policy_id,
            configs_request_policy_id: pre_migration_entry.configs_request_policy_id,
            last_modification_timestamp: pre_migration_entry.last_modification_timestamp,
            seed,
            assets: pre_migration_entry.assets.unwrap_or(vec![AccountAsset {
                asset_id: INITIAL_ICP_ASSET_ID,
                balance: pre_migration_entry.balance.unwrap_or(None),
            }]),
            addresses: pre_migration_entry.addresses.unwrap_or_else(|| {
                vec![
                    AccountAddress {
                        address: pre_migration_entry.address.unwrap_or(
                            InternetComputer::create().generate_account_identifier(&seed),
                        ),
                        format: AddressFormat::ICPAccountIdentifier,
                    },
                    AccountAddress {
                        address: InternetComputer::create().generate_icrc1_address(&seed),
                        format: AddressFormat::ICRC1Account,
                    },
                ]
            }),
        })
    }
}

impl<'de> Deserialize<'de> for TransferOperationInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct PreMigrationTransferOperationInput {
            pub from_account_id: AccountId,
            pub to: String,
            pub amount: candid::Nat,
            pub metadata: Metadata,
            pub network: String,
            pub fee: Option<candid::Nat>,

            pub from_asset_id: Option<AssetId>,
            pub with_standard: Option<TokenStandard>,
        }

        let pre_migration_entry = PreMigrationTransferOperationInput::deserialize(deserializer)?;

        #[cfg(test)]
        if pre_migration_entry.from_asset_id.is_none() {
            MIGRATED_ENTRIES.with(|entries| {
                *entries.borrow_mut() += 1;
            });
        }

        Ok(TransferOperationInput {
            from_account_id: pre_migration_entry.from_account_id,
            to: pre_migration_entry.to,
            amount: pre_migration_entry.amount,
            metadata: pre_migration_entry.metadata,
            network: pre_migration_entry.network,
            fee: pre_migration_entry.fee,
            from_asset_id: pre_migration_entry
                .from_asset_id
                .unwrap_or(INITIAL_ICP_ASSET_ID),
            with_standard: pre_migration_entry
                .with_standard
                .unwrap_or(TokenStandard::InternetComputerNative),
        })
    }
}

impl<'de> Deserialize<'de> for TransferOperation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct PreMigrationTransferOperation {
            pub transfer_id: Option<UUID>,
            pub input: TransferOperationInput,
            pub fee: Option<candid::Nat>,

            pub asset: Option<Asset>,
        }

        let pre_migration_entry = PreMigrationTransferOperation::deserialize(deserializer)?;

        #[cfg(test)]
        if pre_migration_entry.asset.is_none() {
            MIGRATED_ENTRIES.with(|entries| {
                *entries.borrow_mut() += 1;
            });
        }

        Ok(TransferOperation {
            transfer_id: pre_migration_entry.transfer_id,
            input: pre_migration_entry.input,
            fee: pre_migration_entry.fee,
            asset: pre_migration_entry
                .asset
                .unwrap_or_else(|| INITIAL_ICP_ASSET.clone()),
        })
    }
}

impl<'de> Deserialize<'de> for AddAccountOperationInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[allow(dead_code)]
        #[derive(Debug, Deserialize)]
        struct PreMigrationAddAccountOperationInput {
            pub name: String,
            pub metadata: Metadata,
            pub read_permission: Allow,
            pub configs_permission: Allow,
            pub transfer_permission: Allow,
            pub configs_request_policy: Option<RequestPolicyRule>,
            pub transfer_request_policy: Option<RequestPolicyRule>,

            // removed fields
            pub blockchain: Option<Blockchain>,
            pub standard: Option<BlockchainStandard>,

            // new fields
            pub assets: Option<Vec<AssetId>>,
        }

        let pre_migration_entry = PreMigrationAddAccountOperationInput::deserialize(deserializer)?;

        #[cfg(test)]
        if pre_migration_entry.assets.is_none() {
            MIGRATED_ENTRIES.with(|entries| {
                *entries.borrow_mut() += 1;
            });
        }

        Ok(AddAccountOperationInput {
            name: pre_migration_entry.name,
            metadata: pre_migration_entry.metadata,
            read_permission: pre_migration_entry.read_permission,
            configs_permission: pre_migration_entry.configs_permission,
            transfer_permission: pre_migration_entry.transfer_permission,
            configs_request_policy: pre_migration_entry.configs_request_policy,
            transfer_request_policy: pre_migration_entry.transfer_request_policy,
            assets: pre_migration_entry
                .assets
                .unwrap_or_else(|| vec![INITIAL_ICP_ASSET_ID]),
        })
    }
}

impl<'de> Deserialize<'de> for AddAddressBookEntryOperationInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[allow(dead_code)]
        #[derive(Debug, Deserialize)]
        struct PreMigrationAddAddressBookEntryOperationInput {
            pub address_owner: String,
            pub address: String,
            pub blockchain: Blockchain,
            #[serde(default)]
            pub labels: Vec<String>,
            pub metadata: Vec<MetadataItem>,

            // added fields
            pub address_format: Option<AddressFormat>,
        }

        let pre_migration_entry =
            PreMigrationAddAddressBookEntryOperationInput::deserialize(deserializer)?;

        #[cfg(test)]
        if pre_migration_entry.address_format.is_none() {
            MIGRATED_ENTRIES.with(|entries| {
                *entries.borrow_mut() += 1;
            });
        }

        Ok(AddAddressBookEntryOperationInput {
            address_owner: pre_migration_entry.address_owner,
            address: pre_migration_entry.address,
            blockchain: pre_migration_entry.blockchain,
            labels: pre_migration_entry.labels,
            metadata: pre_migration_entry.metadata,
            address_format: pre_migration_entry
                .address_format
                .unwrap_or(AddressFormat::ICPAccountIdentifier),
        })
    }
}

impl RebuildRepository<AddressBookEntryKey, AddressBookEntry, VirtualMemory<Memory>>
    for AddressBookRepository
{
}

impl RebuildRepository<TransferKey, Transfer, VirtualMemory<Memory>> for TransferRepository {}
impl RebuildRepository<AccountKey, Account, VirtualMemory<Memory>> for AccountRepository {}
impl RebuildRepository<RequestKey, Request, VirtualMemory<Memory>> for RequestRepository {}

#[cfg(test)]
mod test {
    use std::{borrow::BorrowMut, fs};

    use ic_stable_structures::{memory_manager::MemoryId, Memory};
    use orbit_essentials::repository::{RebuildRepository, Repository};

    use crate::{
        core::{
            ACCOUNT_MEMORY_ID, ADDRESS_BOOK_MEMORY_ID, MEMORY_MANAGER, REQUEST_MEMORY_ID,
            TRANSFER_MEMORY_ID, WASM_PAGE_SIZE,
        },
        migration::{INITIAL_ICP_ASSET_ID, MIGRATED_ACCOUNTS, MIGRATED_ENTRIES},
        models::AddressFormat,
        repositories::{
            address_book, ACCOUNT_REPOSITORY, ADDRESS_BOOK_REPOSITORY, REQUEST_REPOSITORY,
            TRANSFER_REPOSITORY,
        },
        STABLE_MEMORY_VERSION,
    };

    fn restore_snapshot(label: &str, memory_id: MemoryId) {
        let snapshot = fs::read(format!(
            "src/migration_tests/snapshots/{}_v{}.bin",
            label,
            STABLE_MEMORY_VERSION - 1
        ))
        .unwrap();

        let mut memory = MEMORY_MANAGER.with(|mm| mm.borrow_mut().get(memory_id));
        memory.grow(snapshot.len() as u64 / WASM_PAGE_SIZE as u64 + 1u64);
        memory.borrow_mut().write(0, &snapshot);
    }

    #[test]
    fn test_address_book_migration() {
        restore_snapshot("address_book_repository", ADDRESS_BOOK_MEMORY_ID);

        address_book::ADDRESS_BOOK_REPOSITORY.list();
        assert!(MIGRATED_ENTRIES.with(|entries| *entries.borrow_mut()) > 0);

        ADDRESS_BOOK_REPOSITORY.rebuild();

        MIGRATED_ENTRIES.with(|entries| {
            *entries.borrow_mut() = 0;
        });

        address_book::ADDRESS_BOOK_REPOSITORY.list();
        assert!(MIGRATED_ENTRIES.with(|entries| *entries.borrow_mut()) == 0);
    }

    #[test]
    fn test_transfer_migration() {
        restore_snapshot("transfer_repository", TRANSFER_MEMORY_ID);

        TRANSFER_REPOSITORY.list();
        assert!(MIGRATED_ENTRIES.with(|entries| *entries.borrow_mut()) > 0);

        TRANSFER_REPOSITORY.rebuild();

        MIGRATED_ENTRIES.with(|entries| {
            *entries.borrow_mut() = 0;
        });

        TRANSFER_REPOSITORY.list();
        assert!(MIGRATED_ENTRIES.with(|entries| *entries.borrow_mut()) == 0);
    }

    #[test]
    fn test_account_migration() {
        restore_snapshot("account_repository", ACCOUNT_MEMORY_ID);

        ACCOUNT_REPOSITORY.list();
        assert!(MIGRATED_ACCOUNTS.with(|entries| entries.borrow_mut().len()) > 0);

        ACCOUNT_REPOSITORY.rebuild();

        let accounts = ACCOUNT_REPOSITORY.list();
        for account in accounts {
            assert!(account.seed == account.id);
            assert!(
                account.assets.first().expect("No assets found").asset_id == INITIAL_ICP_ASSET_ID
            );

            assert!(account.addresses.len() == 2);

            let migrated_account = MIGRATED_ACCOUNTS.with(|accounts| {
                accounts
                    .borrow()
                    .iter()
                    .find(|(id, _)| *id == account.id)
                    .expect("Account not found in migrated accounts")
                    .clone()
            });

            assert!(account
                .addresses
                .iter()
                .any(|address| address.address == migrated_account.1
                    && address.format == AddressFormat::ICPAccountIdentifier));
            assert!(account
                .addresses
                .iter()
                .any(|address| address.format == AddressFormat::ICRC1Account));
        }

        MIGRATED_ACCOUNTS.with(|entries| {
            entries.borrow_mut().clear();
        });

        ACCOUNT_REPOSITORY.list();
        assert!(MIGRATED_ACCOUNTS.with(|entries| entries.borrow_mut().len()) == 0);
    }

    #[test]
    fn test_request_migration() {
        restore_snapshot("request_repository", REQUEST_MEMORY_ID);

        REQUEST_REPOSITORY.list();
        assert!(MIGRATED_ENTRIES.with(|entries| *entries.borrow_mut()) > 0);

        REQUEST_REPOSITORY.rebuild();

        MIGRATED_ENTRIES.with(|entries| {
            *entries.borrow_mut() = 0;
        });

        REQUEST_REPOSITORY.list();
        assert!(MIGRATED_ENTRIES.with(|entries| *entries.borrow_mut()) == 0);
    }
}
