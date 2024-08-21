use crate::core::ic_cdk::api::{print, trap};
use crate::core::{read_system_info, write_system_info, Memory};
use crate::models::permission::{Permission, PermissionKey};
use crate::models::request_specifier::RequestSpecifier;
use crate::models::resource::{ExternalCanisterResourceAction, Resource, SystemResourceAction};
use crate::models::{
    Account, AccountKey, AddressBookEntry, AddressBookEntryKey, ExternalCanister,
    ExternalCanisterKey, Request, RequestKey, RequestOperation, RequestPolicy, User, UserGroup,
    UserKey,
};
use crate::repositories::permission::{PermissionRepository, PERMISSION_REPOSITORY};
use crate::repositories::{
    AccountRepository, AddressBookRepository, ExternalCanisterRepository, RequestPolicyRepository,
    RequestRepository, UserGroupRepository, UserRepository, ACCOUNT_REPOSITORY,
    ADDRESS_BOOK_REPOSITORY, EXTERNAL_CANISTER_REPOSITORY, REQUEST_POLICY_REPOSITORY,
    USER_GROUP_REPOSITORY, USER_REPOSITORY,
};
use crate::{concat_str_arrays, STABLE_MEMORY_VERSION};
use crate::{core::with_memory_manager, repositories::REQUEST_REPOSITORY};
use ic_stable_structures::memory_manager::{MemoryId, VirtualMemory};
use ic_stable_structures::Memory as DefaultMemoryTrait;
use orbit_essentials::repository::{IndexedRepository, RebuildRepository, Repository, StableDb};
use orbit_essentials::storable;
use orbit_essentials::types::UUID;
use serde::de::{self, EnumAccess, VariantAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;
use strum::VariantNames;
use uuid::Uuid;

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
            return;
        }

        if stored_version > STABLE_MEMORY_VERSION {
            trap(&format!(
                "Cannot downgrade the station from memory layout version {} to {}",
                stored_version, STABLE_MEMORY_VERSION
            ));
        }

        apply_migration();

        // Update the stable memory version to the latest version.
        system_info.set_stable_memory_version(STABLE_MEMORY_VERSION);
        write_system_info(system_info);
    }
}

/// The migration to apply to the station canister stable memory.
///
/// Please include the migration steps in the `apply_migration` function.
fn apply_migration() {
    // step 1: clear unused memory ids
    with_memory_manager(|memory_manager| {
        for memory_id in [
            MemoryId::new(3),  // USER_IDENTITY_INDEX_MEMORY_ID,
            MemoryId::new(5),  // REQUEST_EXPIRATION_TIME_INDEX_MEMORY_ID
            MemoryId::new(8),  // REQUEST_APPROVER_INDEX_MEMORY_ID
            MemoryId::new(9),  // REQUEST_STATUS_INDEX_MEMORY_ID
            MemoryId::new(10), // REQUEST_SCHEDULED_INDEX_MEMORY_ID
            MemoryId::new(15), // USER_GROUP_NAME_INDEX_MEMORY_ID
            MemoryId::new(18), // USER_STATUS_GROUP_INDEX_MEMORY_ID
            MemoryId::new(20), // ADDRESS_BOOK_INDEX_MEMORY_ID
            MemoryId::new(21), // REQUEST_REQUESTER_INDEX_MEMORY_ID
            MemoryId::new(22), // REQUEST_CREATION_TIME_INDEX_MEMORY_ID
            MemoryId::new(23), // REQUEST_KEY_CREATION_TIME_INDEX_MEMORY_ID
            MemoryId::new(24), // REQUEST_KEY_EXPIRATION_TIME_INDEX_MEMORY_ID
            MemoryId::new(25), // REQUEST_SORT_INDEX_MEMORY_ID
            MemoryId::new(26), // REQUEST_STATUS_MODIFICATION_INDEX_MEMORY_ID
            MemoryId::new(27), // NAME_TO_ACCOUNT_ID_INDEX_MEMORY_ID
            MemoryId::new(28), // NAME_TO_USER_ID_INDEX_MEMORY_ID
            MemoryId::new(29), // OPERATION_TYPE_TO_REQUEST_ID_INDEX_MEMORY_ID
            MemoryId::new(34), // EXTERNAL_CANISTER_INDEX_MEMORY_ID
            // The following memory ids are still in use for the same purpose, but the datatype
            // have changed and the memory needs to be cleaned up and rebuilt later.
            MemoryId::new(30), // REQUEST_RESOURCE_INDEX_MEMORY_ID
            MemoryId::new(31), // POLICY_RESOURCE_INDEX_MEMORY_ID
        ] {
            // This cleans up the memory by writing a single zero byte to the memory id,
            // this will make the memory id available for reuse in the future.
            //
            // This makes sure that if `init` is called on the memory id, it will make sure
            // it can be reused with a different datatype.
            let memory = memory_manager.get(memory_id);
            if memory.size() > 0 {
                // This marks the memory as unused, this is because the StableBTreeMap
                // implementation uses the first three bytes of the memory to store the MAGIC value [66, 84, 82]
                // that indicates that the memory is used by the StableBTreeMap, so adding a single different byte
                // in those first three bytes will make the memory available for reuse.
                memory.write(0, &[0]);
            }
        }
    });

    // step 2: rebuilds the repositories to ensure the data is up-to-date
    USER_GROUP_REPOSITORY.rebuild();
    USER_REPOSITORY.rebuild();
    ACCOUNT_REPOSITORY.rebuild();
    EXTERNAL_CANISTER_REPOSITORY.rebuild();
    ADDRESS_BOOK_REPOSITORY.rebuild();
    PERMISSION_REPOSITORY.rebuild();
    REQUEST_POLICY_REPOSITORY.rebuild();
    REQUEST_REPOSITORY.rebuild();
}

impl<'de> Deserialize<'de> for Resource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const ENUM_NAME: &str = "Resource";

        const CURRENT_VARIANTS: &[&str] = Resource::VARIANTS;
        const REMOVED_VARIANTS: [&str; 1] = ["ChangeCanister"];

        // IMPORTANT: The size of the array must be hardcoded, to make sure it can be checked at compile-time.
        static EXPECTED_VARIANTS: [&str; 10] = {
            let variants: [&str; CURRENT_VARIANTS.len() + REMOVED_VARIANTS.len()] =
                concat_str_arrays!(CURRENT_VARIANTS, REMOVED_VARIANTS);

            variants
        };

        // Define the old version of the types for migration purposes
        #[storable]
        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum OldExternalCanisterResourceAction {
            Create(OldCreateCanisterTarget),
        }

        #[storable]
        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum OldCreateCanisterTarget {
            Any,
        }

        #[storable]
        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum OldChangeCanisterResourceAction {
            Create,
        }

        /// This enum facilitates the deserialization of the ExternalCanisterResourceAction enum.
        ///
        /// By creating it as an untagged enum, we can handle both the old and new formats of the enum and
        /// serde will automatically choose the correct format based on the input data.
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum ExternalCanisterActionWrapper {
            NewFormat(ExternalCanisterResourceAction),
            OldFormat(OldExternalCanisterResourceAction),
        }

        struct ResourceVisitor;

        impl<'de> Visitor<'de> for ResourceVisitor {
            type Value = Resource;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(&format!("a valid {} enum variant", ENUM_NAME))
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: EnumAccess<'de>,
            {
                let (variant, variant_access) = data.variant::<String>()?;

                // Due to the fact that serde serialization uses a string representation of the enum variant,
                // it is not possible to do a compile-time check for all variants of the enum.
                match variant.as_str() {
                    // First the new formats
                    "ExternalCanister" => {
                        // Deserialize into the wrapper, which can handle both formats
                        let wrapper =
                            variant_access.newtype_variant::<ExternalCanisterActionWrapper>()?;

                        // Try deserializing as the new format
                        match wrapper {
                            ExternalCanisterActionWrapper::NewFormat(new_format) => {
                                Ok(Resource::ExternalCanister(new_format))
                            }
                            ExternalCanisterActionWrapper::OldFormat(_) => Ok(
                                Resource::ExternalCanister(ExternalCanisterResourceAction::Create),
                            ),
                        }
                    }
                    // `ChangeCanister` does not exist anymore, so we need to handle it here
                    "ChangeCanister" => {
                        // Consume the old format variant, this is to make sure there is no
                        // trailing data is left in the end of the deserialization, which would cause an error.
                        //
                        // The use of `Option<String>`` is to make sure that the deserialization is successful.
                        let _ = variant_access.newtype_variant::<OldChangeCanisterResourceAction>();
                        // The `ChangeCanister` variant was removed, so we need to handle it here
                        // and map it to the correct variant.
                        Ok(Resource::System(SystemResourceAction::Upgrade))
                    }
                    // Then all the default cases
                    "Permission" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(Resource::Permission(value))
                    }
                    "Account" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(Resource::Account(value))
                    }
                    "AddressBook" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(Resource::AddressBook(value))
                    }
                    "Request" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(Resource::Request(value))
                    }
                    "RequestPolicy" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(Resource::RequestPolicy(value))
                    }
                    "System" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(Resource::System(value))
                    }
                    "User" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(Resource::User(value))
                    }
                    "UserGroup" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(Resource::UserGroup(value))
                    }
                    _ => Err(de::Error::unknown_variant(&variant, &EXPECTED_VARIANTS)),
                }
            }
        }

        deserializer.deserialize_enum(ENUM_NAME, &EXPECTED_VARIANTS, ResourceVisitor)
    }
}

impl<'de> Deserialize<'de> for RequestSpecifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const ENUM_NAME: &str = "RequestSpecifier";

        const CURRENT_VARIANTS: &[&str] = RequestSpecifier::VARIANTS;
        const REMOVED_VARIANTS: [&str; 1] = ["ChangeCanister"];

        // IMPORTANT: The size of the array must be hardcoded, to make sure it can be checked at compile-time.
        static EXPECTED_VARIANTS: [&str; 23] = {
            let variants: [&str; CURRENT_VARIANTS.len() + REMOVED_VARIANTS.len()] =
                concat_str_arrays!(CURRENT_VARIANTS, REMOVED_VARIANTS);

            variants
        };

        // Define the old version of the types for migration purposes
        #[derive(Deserialize)]
        enum OldCreateExternalCanisterTarget {
            Any,
        }

        struct RequestSpecifierVisitor;

        impl<'de> Visitor<'de> for RequestSpecifierVisitor {
            type Value = RequestSpecifier;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(&format!("a valid {} enum variant", ENUM_NAME))
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: EnumAccess<'de>,
            {
                let (variant, variant_access) = data.variant::<String>()?;

                // Due to the fact that serde serialization uses a string representation of the enum variant,
                // it is not possible to do a compile-time check for all variants of the enum.
                match variant.as_str() {
                    // First the new formats
                    "CreateExternalCanister" => {
                        // Even though the value of the variant is not used, we still need to consume it
                        // to make sure there is no trailing data left in the end of the deserialization.
                        let _ = variant_access
                            .newtype_variant::<Option<OldCreateExternalCanisterTarget>>();

                        Ok(RequestSpecifier::CreateExternalCanister)
                    }
                    // `ChangeCanister` does not exist anymore, so we need to handle it here
                    "ChangeCanister" => Ok(RequestSpecifier::SystemUpgrade),
                    // Then all the default cases
                    "AddAccount" => Ok(RequestSpecifier::AddAccount),
                    "AddUser" => Ok(RequestSpecifier::AddUser),
                    "EditAccount" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestSpecifier::EditAccount(value))
                    }
                    "EditUser" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestSpecifier::EditUser(value))
                    }
                    "AddAddressBookEntry" => Ok(RequestSpecifier::AddAddressBookEntry),
                    "EditAddressBookEntry" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestSpecifier::EditAddressBookEntry(value))
                    }
                    "RemoveAddressBookEntry" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestSpecifier::RemoveAddressBookEntry(value))
                    }
                    "Transfer" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestSpecifier::Transfer(value))
                    }
                    "SystemUpgrade" => Ok(RequestSpecifier::SystemUpgrade),
                    "SetDisasterRecovery" => Ok(RequestSpecifier::SetDisasterRecovery),
                    "ChangeExternalCanister" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestSpecifier::ChangeExternalCanister(value))
                    }
                    "CallExternalCanister" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestSpecifier::CallExternalCanister(value))
                    }
                    "EditPermission" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestSpecifier::EditPermission(value))
                    }
                    "AddRequestPolicy" => Ok(RequestSpecifier::AddRequestPolicy),
                    "EditRequestPolicy" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestSpecifier::EditRequestPolicy(value))
                    }
                    "RemoveRequestPolicy" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestSpecifier::RemoveRequestPolicy(value))
                    }
                    "AddUserGroup" => Ok(RequestSpecifier::AddUserGroup),
                    "EditUserGroup" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestSpecifier::EditUserGroup(value))
                    }
                    "RemoveUserGroup" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestSpecifier::RemoveUserGroup(value))
                    }
                    "ManageSystemInfo" => Ok(RequestSpecifier::ManageSystemInfo),
                    "FundExternalCanister" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestSpecifier::FundExternalCanister(value))
                    }
                    _ => Err(de::Error::unknown_variant(&variant, &EXPECTED_VARIANTS)),
                }
            }
        }

        deserializer.deserialize_enum(ENUM_NAME, &EXPECTED_VARIANTS, RequestSpecifierVisitor)
    }
}

impl<'de> Deserialize<'de> for RequestOperation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const ENUM_NAME: &str = "RequestOperation";

        const CURRENT_VARIANTS: &[&str] = RequestOperation::VARIANTS;
        const REMOVED_VARIANTS: [&str; 1] = ["ChangeCanister"];

        // IMPORTANT: The size of the array must be hardcoded, to make sure it can be checked at compile-time.
        static EXPECTED_VARIANTS: [&str; 24] = {
            let variants: [&str; CURRENT_VARIANTS.len() + REMOVED_VARIANTS.len()] =
                concat_str_arrays!(CURRENT_VARIANTS, REMOVED_VARIANTS);

            variants
        };

        struct RequestOperationVisitor;

        impl<'de> Visitor<'de> for RequestOperationVisitor {
            type Value = RequestOperation;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(&format!("a valid {} enum variant", ENUM_NAME))
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: EnumAccess<'de>,
            {
                let (variant, variant_access) = data.variant::<String>()?;

                // Due to the fact that serde serialization uses a string representation of the enum variant,
                // it is not possible to do a compile-time check for all variants of the enum.
                match variant.as_str() {
                    // First the new formats
                    // `ChangeCanister` does not exist anymore, so we need to handle it here
                    "ChangeCanister" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestOperation::SystemUpgrade(value))
                    }
                    // Then all the default cases
                    "Transfer" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestOperation::Transfer(value))
                    }
                    "AddAccount" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestOperation::AddAccount(value))
                    }
                    "EditAccount" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestOperation::EditAccount(value))
                    }
                    "AddAddressBookEntry" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestOperation::AddAddressBookEntry(value))
                    }
                    "EditAddressBookEntry" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestOperation::EditAddressBookEntry(value))
                    }
                    "RemoveAddressBookEntry" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestOperation::RemoveAddressBookEntry(value))
                    }
                    "AddUser" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestOperation::AddUser(value))
                    }
                    "EditUser" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestOperation::EditUser(value))
                    }
                    "EditPermission" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestOperation::EditPermission(value))
                    }
                    "AddUserGroup" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestOperation::AddUserGroup(value))
                    }
                    "EditUserGroup" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestOperation::EditUserGroup(value))
                    }
                    "RemoveUserGroup" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestOperation::RemoveUserGroup(value))
                    }
                    "SystemUpgrade" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestOperation::SystemUpgrade(value))
                    }
                    "ChangeExternalCanister" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestOperation::ChangeExternalCanister(value))
                    }
                    "ConfigureExternalCanister" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestOperation::ConfigureExternalCanister(value))
                    }
                    "CreateExternalCanister" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestOperation::CreateExternalCanister(value))
                    }
                    "CallExternalCanister" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestOperation::CallExternalCanister(value))
                    }
                    "FundExternalCanister" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestOperation::FundExternalCanister(value))
                    }
                    "AddRequestPolicy" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestOperation::AddRequestPolicy(value))
                    }
                    "EditRequestPolicy" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestOperation::EditRequestPolicy(value))
                    }
                    "RemoveRequestPolicy" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestOperation::RemoveRequestPolicy(value))
                    }
                    "ManageSystemInfo" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestOperation::ManageSystemInfo(value))
                    }
                    "SetDisasterRecovery" => {
                        let value = variant_access.newtype_variant()?;
                        Ok(RequestOperation::SetDisasterRecovery(value))
                    }
                    _ => Err(de::Error::unknown_variant(&variant, &EXPECTED_VARIANTS)),
                }
            }
        }

        deserializer.deserialize_enum(ENUM_NAME, &EXPECTED_VARIANTS, RequestOperationVisitor)
    }
}

// Repositories should only implement the `RebuildRepository` trait if they are affected by the migration,
// otherwise, they should not implement the trait.
//
// The ones affected should have the implementation here.

impl RebuildRepository<RequestKey, Request, VirtualMemory<Memory>> for RequestRepository {
    fn rebuild(&self) {
        let keys = Self::with_db(|db| db.iter().map(|(k, _)| k).collect::<Vec<_>>());

        for key in keys {
            match self.get(&key) {
                Some(mut request) => {
                    // First make sure there is no dangling index for the entry.
                    self.remove_entry_indexes(&request);
                    // Then add the updated indexes.
                    self.add_entry_indexes(&request);
                    // Clear the module field if the request is finalized to save memory.
                    if request.is_finalized() {
                        if let RequestOperation::SystemUpgrade(operation) = &mut request.operation {
                            operation.input.module = Vec::new();
                        }
                    }

                    Self::with_db(|db| db.insert(key, request));
                }
                None => print(format!(
                    "Unexpected Request Id({}) not found in the repository",
                    Uuid::from_bytes(key.id)
                )),
            }
        }
    }
}

impl RebuildRepository<PermissionKey, Permission, VirtualMemory<Memory>> for PermissionRepository {
    fn rebuild(&self) {
        let permissions = Self::with_db(|db| db.iter().map(|(_, v)| v).collect::<Vec<_>>());

        // Then clear the repository because the resource is a key that was updated and has dropped the
        // `ChangeCanister` variant, and Resource is a complex type that is serialized and deserialized.
        Self::with_db(|db| db.clear_new());

        Self::with_db(|db| {
            for permission in permissions {
                db.insert(permission.resource.clone(), permission);
            }
        });
    }
}

impl RebuildRepository<AccountKey, Account, VirtualMemory<Memory>> for AccountRepository {}
impl RebuildRepository<AddressBookEntryKey, AddressBookEntry, VirtualMemory<Memory>>
    for AddressBookRepository
{
}
impl RebuildRepository<ExternalCanisterKey, ExternalCanister, VirtualMemory<Memory>>
    for ExternalCanisterRepository
{
}
impl RebuildRepository<UUID, UserGroup, VirtualMemory<Memory>> for UserGroupRepository {}
impl RebuildRepository<UserKey, User, VirtualMemory<Memory>> for UserRepository {}
impl RebuildRepository<UUID, RequestPolicy, VirtualMemory<Memory>> for RequestPolicyRepository {}
