use std::sync::Arc;

#[cfg(test)]
use std::cell::RefCell;

use crate::{
    errors::{ExternalCanisterValidationError, RecordValidationError},
    models::{
        resource::{Resource, ResourceId, ResourceIds},
        AccountKey, AddressBookEntryKey, RequestKey, UserKey,
    },
    repositories::{
        permission::PERMISSION_REPOSITORY, request_policy::REQUEST_POLICY_REPOSITORY,
        ACCOUNT_REPOSITORY, ADDRESS_BOOK_REPOSITORY, REQUEST_REPOSITORY, USER_GROUP_REPOSITORY,
        USER_REPOSITORY,
    },
    services::SYSTEM_SERVICE,
};
use candid::Principal;
#[cfg(not(test))]
pub use orbit_essentials::cdk as ic_cdk;
#[cfg(test)]
pub use orbit_essentials::cdk::mocks as ic_cdk;
use orbit_essentials::repository::Repository;
use orbit_essentials::types::UUID;
use uuid::Uuid;

thread_local! {
  /// Switch for tests to enable validation if needed.
  #[cfg(test)]
  static MOCK_RESOURCE_VALIDATION_ON: RefCell<bool> = const { RefCell::new(true) };
}

#[cfg(test)]
pub fn disable_mock_resource_validation() {
    MOCK_RESOURCE_VALIDATION_ON.with(|v| *v.borrow_mut() = false);
}

#[cfg(test)]
pub fn enable_mock_resource_validation() {
    MOCK_RESOURCE_VALIDATION_ON.with(|v| *v.borrow_mut() = true);
}

fn ensure_entry_exists<K, V>(repository: Arc<dyn Repository<K, V>>, key: K) -> Option<()> {
    #[cfg(test)]
    if MOCK_RESOURCE_VALIDATION_ON.with(|v| *v.borrow()) {
        return Some(());
    }

    repository.get(&key).map(|_| ())
}

pub trait EnsureIdExists<Key> {
    fn id_exists(id: &Key) -> Result<(), RecordValidationError>;

    fn id_list_exists(ids: &[Key]) -> Result<(), RecordValidationError> {
        for id in ids {
            Self::id_exists(id)?;
        }
        Ok(())
    }
}

pub trait EnsureResourceIdExists: EnsureIdExists<UUID> {
    fn resource_id_exists(resource_id: &ResourceId) -> Result<(), RecordValidationError> {
        match resource_id {
            ResourceId::Any => Ok(()),
            ResourceId::Id(id) => Self::id_exists(id),
        }
    }
    fn resource_ids_exist(resource_ids: &ResourceIds) -> Result<(), RecordValidationError> {
        match resource_ids {
            ResourceIds::Any => Ok(()),
            ResourceIds::Ids(ids) => {
                Self::id_list_exists(ids)?;
                Ok(())
            }
        }
    }
}

pub struct EnsureUser {}

impl EnsureIdExists<UUID> for EnsureUser {
    fn id_exists(id: &UUID) -> Result<(), RecordValidationError> {
        ensure_entry_exists(USER_REPOSITORY.to_owned(), UserKey { id: *id }).ok_or(
            RecordValidationError::NotFound {
                model_name: "User".to_string(),
                id: Uuid::from_bytes(*id).hyphenated().to_string(),
            },
        )
    }
}

impl EnsureResourceIdExists for EnsureUser {}

pub struct EnsureUserGroup {}

impl EnsureIdExists<UUID> for EnsureUserGroup {
    fn id_exists(id: &UUID) -> Result<(), RecordValidationError> {
        ensure_entry_exists(USER_GROUP_REPOSITORY.to_owned(), *id).ok_or(
            RecordValidationError::NotFound {
                model_name: "UserGroup".to_string(),
                id: Uuid::from_bytes(*id).hyphenated().to_string(),
            },
        )
    }
}

impl EnsureResourceIdExists for EnsureUserGroup {}

pub struct EnsureAccount {}

impl EnsureIdExists<UUID> for EnsureAccount {
    fn id_exists(id: &UUID) -> Result<(), RecordValidationError> {
        ensure_entry_exists(ACCOUNT_REPOSITORY.to_owned(), AccountKey { id: *id }).ok_or(
            RecordValidationError::NotFound {
                model_name: "Account".to_string(),
                id: Uuid::from_bytes(*id).hyphenated().to_string(),
            },
        )
    }
}

impl EnsureResourceIdExists for EnsureAccount {}

pub struct EnsureAddressBookEntry {}

impl EnsureIdExists<UUID> for EnsureAddressBookEntry {
    fn id_exists(id: &UUID) -> Result<(), RecordValidationError> {
        ensure_entry_exists(
            ADDRESS_BOOK_REPOSITORY.to_owned(),
            AddressBookEntryKey { id: *id },
        )
        .ok_or(RecordValidationError::NotFound {
            model_name: "AddressBookEntry".to_string(),
            id: Uuid::from_bytes(*id).hyphenated().to_string(),
        })
    }
}

impl EnsureResourceIdExists for EnsureAddressBookEntry {}

pub struct EnsureRequest {}

impl EnsureIdExists<UUID> for EnsureRequest {
    fn id_exists(id: &UUID) -> Result<(), RecordValidationError> {
        ensure_entry_exists(REQUEST_REPOSITORY.to_owned(), RequestKey { id: *id }).ok_or(
            RecordValidationError::NotFound {
                model_name: "Request".to_string(),
                id: Uuid::from_bytes(*id).hyphenated().to_string(),
            },
        )
    }
}

impl EnsureResourceIdExists for EnsureRequest {}

pub struct EnsureRequestPolicy {}

impl EnsureIdExists<UUID> for EnsureRequestPolicy {
    fn id_exists(id: &UUID) -> Result<(), RecordValidationError> {
        ensure_entry_exists(REQUEST_POLICY_REPOSITORY.to_owned(), *id).ok_or(
            RecordValidationError::NotFound {
                model_name: "RequestPolicy".to_string(),
                id: Uuid::from_bytes(*id).hyphenated().to_string(),
            },
        )
    }
}

impl EnsureResourceIdExists for EnsureRequestPolicy {}

pub struct EnsurePermission {}

impl EnsureIdExists<Resource> for EnsurePermission {
    fn id_exists(key: &Resource) -> Result<(), RecordValidationError> {
        ensure_entry_exists(PERMISSION_REPOSITORY.to_owned(), key.to_owned()).ok_or(
            RecordValidationError::NotFound {
                model_name: "Permission".to_string(),
                id: key.to_string(),
            },
        )
    }
}

pub struct EnsureExternalCanister {}

impl EnsureExternalCanister {
    // The management canister, the orbit station, and the upgrader are NOT external canisters.
    pub fn is_external_canister(
        principal: Principal,
    ) -> Result<(), ExternalCanisterValidationError> {
        if principal == Principal::management_canister()
            || principal == ic_cdk::api::id()
            || principal == SYSTEM_SERVICE.get_upgrader_canister_id()
        {
            return Err(ExternalCanisterValidationError::InvalidExternalCanister { principal });
        }

        Ok(())
    }
}
