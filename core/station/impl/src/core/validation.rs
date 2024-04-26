use std::sync::Arc;

#[cfg(test)]
use std::cell::RefCell;

use orbit_essentials::types::UUID;
use uuid::Uuid;

use crate::{
    errors::RecordValidationError,
    models::{
        resource::{Resource, ResourceId, ResourceIds},
        AccountKey, AddressBookEntryKey, ProposalKey, UserKey,
    },
    repositories::{
        access_policy::ACCESS_POLICY_REPOSITORY, policy::PROPOSAL_POLICY_REPOSITORY,
        ACCOUNT_REPOSITORY, ADDRESS_BOOK_REPOSITORY, PROPOSAL_REPOSITORY, USER_GROUP_REPOSITORY,
        USER_REPOSITORY,
    },
};
use orbit_essentials::repository::Repository;

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

pub struct EnsureProposal {}

impl EnsureIdExists<UUID> for EnsureProposal {
    fn id_exists(id: &UUID) -> Result<(), RecordValidationError> {
        ensure_entry_exists(PROPOSAL_REPOSITORY.to_owned(), ProposalKey { id: *id }).ok_or(
            RecordValidationError::NotFound {
                model_name: "Proposal".to_string(),
                id: Uuid::from_bytes(*id).hyphenated().to_string(),
            },
        )
    }
}

impl EnsureResourceIdExists for EnsureProposal {}

pub struct EnsureProposalPolicy {}

impl EnsureIdExists<UUID> for EnsureProposalPolicy {
    fn id_exists(id: &UUID) -> Result<(), RecordValidationError> {
        ensure_entry_exists(PROPOSAL_POLICY_REPOSITORY.to_owned(), *id).ok_or(
            RecordValidationError::NotFound {
                model_name: "ProposalPolicy".to_string(),
                id: Uuid::from_bytes(*id).hyphenated().to_string(),
            },
        )
    }
}

impl EnsureResourceIdExists for EnsureProposalPolicy {}

pub struct EnsureAccessPolicy {}

impl EnsureIdExists<Resource> for EnsureAccessPolicy {
    fn id_exists(key: &Resource) -> Result<(), RecordValidationError> {
        ensure_entry_exists(ACCESS_POLICY_REPOSITORY.to_owned(), key.to_owned()).ok_or(
            RecordValidationError::NotFound {
                model_name: "AccessPolicy".to_string(),
                id: key.to_string(),
            },
        )
    }
}
