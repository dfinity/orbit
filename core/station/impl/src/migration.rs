use std::collections::BTreeSet;

use crate::core::ic_cdk::api::trap;
use crate::core::{read_system_info, write_system_info, Memory};
use crate::factories::blockchains::InternetComputer;
use crate::models::request_specifier::RequestSpecifier;
use crate::models::resource::{Resource, SystemResourceAction};
use crate::repositories::permission::PERMISSION_REPOSITORY;
use crate::repositories::{
    AccountRepository, AddressBookRepository, TransferRepository, REQUEST_POLICY_REPOSITORY,
    USER_GROUP_REPOSITORY, USER_REPOSITORY,
};
use crate::STABLE_MEMORY_VERSION;
use ic_stable_structures::memory_manager::VirtualMemory;
use orbit_essentials::model::ModelKey;
use orbit_essentials::repository::{RebuildRepository, Repository};
use orbit_essentials::types::{Timestamp, UUID};
use serde::{Deserialize, Deserializer};

use crate::models::permission::AuthScope;
use crate::models::resource::{ResourceAction, ResourceId, ResourceIds};
use crate::models::{
    Account, AccountAddress, AccountAsset, AccountBalance, AccountId, AccountKey, AccountSeed,
    AddRequestPolicyOperationInput, AddressBookEntry, AddressBookEntryId, AddressBookEntryKey,
    AddressFormat, Asset, AssetId, Blockchain, EditPermissionOperationInput, Metadata,
    TokenStandard, Transfer, TransferId, TransferKey, TransferStatus, UserId,
};
use crate::repositories::ASSET_REPOSITORY;
use crate::services::permission::PERMISSION_SERVICE;
use crate::services::REQUEST_POLICY_SERVICE;

pub const INITIAL_ICP_ASSET_ID: [u8; 16] = [
    0x78, 0x02, 0xcb, 0xab, 0x22, 0x1d, 0x4e, 0x49, 0xb7, 0x64, 0xa6, 0x95, 0xea, 0x6d, 0xef, 0x1a,
];
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

    // add ICP asset
    let icp_asset = Asset {
        id: INITIAL_ICP_ASSET_ID,
        blockchain: Blockchain::InternetComputer,
        decimals: 8,
        metadata: Metadata::default(),
        name: "Internet Computer".to_string(),
        standards: BTreeSet::from([TokenStandard::InternetComputerNative]),
        symbol: "ICP".to_string(),
    };

    ASSET_REPOSITORY.insert(icp_asset.key(), icp_asset);
}

#[cfg(test)]
thread_local! {
    pub static MIGRATED_ENTRIES: std::cell::RefCell<u64> = const { std::cell::RefCell::new(0) };

    pub static MIGRATED_ACCOUNTS: std::cell::RefCell<Vec<(UUID, String)>> = const { std::cell::RefCell::new(vec![]) };
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
        #[derive(Debug, Deserialize)]
        pub enum BlockchainStandard {
            Native,
            ICRC1,
            ERC20,
        }

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

impl RebuildRepository<AddressBookEntryKey, AddressBookEntry, VirtualMemory<Memory>>
    for AddressBookRepository
{
}

impl RebuildRepository<TransferKey, Transfer, VirtualMemory<Memory>> for TransferRepository {}
impl RebuildRepository<AccountKey, Account, VirtualMemory<Memory>> for AccountRepository {}

#[cfg(test)]
mod test {
    use std::{borrow::BorrowMut, fs};

    use ic_stable_structures::{memory_manager::MemoryId, Memory};
    use orbit_essentials::repository::{RebuildRepository, Repository};

    use crate::{
        core::{
            ACCOUNT_MEMORY_ID, ADDRESS_BOOK_MEMORY_ID, MEMORY_MANAGER, TRANSFER_MEMORY_ID,
            WASM_PAGE_SIZE,
        },
        migration::{INITIAL_ICP_ASSET_ID, MIGRATED_ACCOUNTS, MIGRATED_ENTRIES},
        models::AddressFormat,
        repositories::{
            address_book, ACCOUNT_REPOSITORY, ADDRESS_BOOK_REPOSITORY, TRANSFER_REPOSITORY,
        },
        STABLE_MEMORY_VERSION,
    };

    fn restore_snapshot(name: &str, memory_id: MemoryId) {
        let snapshot = fs::read(format!(
            "src/migration_tests/snapshots/{}_repo_snapshot_v{}.bin",
            name,
            STABLE_MEMORY_VERSION - 1
        ))
        .unwrap();

        let mut memory = MEMORY_MANAGER.with(|mm| mm.borrow_mut().get(memory_id));
        memory.grow(snapshot.len() as u64 / WASM_PAGE_SIZE as u64 + 1u64);
        memory.borrow_mut().write(0, &snapshot);
    }

    #[test]
    fn test_address_book_migration() {
        restore_snapshot("address_book", ADDRESS_BOOK_MEMORY_ID);

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
        restore_snapshot("transfer", TRANSFER_MEMORY_ID);

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
        restore_snapshot("account", ACCOUNT_MEMORY_ID);

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
}
