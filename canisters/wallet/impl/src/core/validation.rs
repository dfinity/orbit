use std::sync::Arc;

#[cfg(test)]
use std::cell::RefCell;

use ic_canister_core::types::UUID;
use uuid::Uuid;

use crate::{
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
use ic_canister_core::repository::Repository;

thread_local! {
  /// Switch for tests to enable validation if needed.
  #[cfg(test)]
  static MOCK_VALIDATION_ON: RefCell<bool> = RefCell::new(true);
}

#[cfg(test)]
pub fn disable_mock_validation() {
    MOCK_VALIDATION_ON.with(|v| *v.borrow_mut() = false);
}

#[cfg(test)]
pub fn enable_mock_validation() {
    MOCK_VALIDATION_ON.with(|v| *v.borrow_mut() = true);
}

#[derive(Debug)]
pub struct RecordNotFoundError {
    pub model_name: String,
    pub id: String,
}

fn ensure_entry_exists<K, V>(repository: Arc<dyn Repository<K, V>>, key: K) -> Option<()> {
    #[cfg(test)]
    if MOCK_VALIDATION_ON.with(|v| *v.borrow()) {
        return Some(());
    }

    repository.get(&key).map(|_| ())
}

pub fn ensure_user_exists(user_id: &UUID) -> Result<(), RecordNotFoundError> {
    ensure_entry_exists(USER_REPOSITORY.to_owned(), UserKey { id: *user_id }).ok_or(
        RecordNotFoundError {
            model_name: "User".to_string(),
            id: Uuid::from_bytes(*user_id).hyphenated().to_string(),
        },
    )
}

pub fn ensure_user_resource_id_exists(resource_id: &ResourceId) -> Result<(), RecordNotFoundError> {
    match resource_id {
        ResourceId::Any => Ok(()),
        ResourceId::Id(id) => ensure_user_exists(id),
    }
}

pub fn ensure_user_resource_ids_exist(resource: &ResourceIds) -> Result<(), RecordNotFoundError> {
    match resource {
        ResourceIds::Any => Ok(()),
        ResourceIds::Ids(ids) => {
            for id in ids {
                ensure_user_exists(id)?;
            }
            Ok(())
        }
    }
}

pub fn ensure_user_group_exists(id: &UUID) -> Result<(), RecordNotFoundError> {
    ensure_entry_exists(USER_GROUP_REPOSITORY.to_owned(), *id).ok_or(RecordNotFoundError {
        model_name: "UserGroup".to_string(),
        id: Uuid::from_bytes(*id).hyphenated().to_string(),
    })
}

pub fn ensure_user_group_resource_id_exists(
    resource_id: &ResourceId,
) -> Result<(), RecordNotFoundError> {
    match resource_id {
        ResourceId::Any => Ok(()),
        ResourceId::Id(id) => ensure_user_group_exists(id),
    }
}

pub fn ensure_user_group_resource_ids_exist(
    resource: &ResourceIds,
) -> Result<(), RecordNotFoundError> {
    match resource {
        ResourceIds::Any => Ok(()),
        ResourceIds::Ids(ids) => {
            for id in ids {
                ensure_user_group_exists(id)?;
            }
            Ok(())
        }
    }
}

pub fn ensure_account_exists(id: &UUID) -> Result<(), RecordNotFoundError> {
    ensure_entry_exists(ACCOUNT_REPOSITORY.to_owned(), AccountKey { id: *id }).ok_or(
        RecordNotFoundError {
            model_name: "Account".to_string(),
            id: Uuid::from_bytes(*id).hyphenated().to_string(),
        },
    )
}

pub fn ensure_account_resource_id_exists(
    resource_id: &ResourceId,
) -> Result<(), RecordNotFoundError> {
    match resource_id {
        ResourceId::Any => Ok(()),
        ResourceId::Id(id) => ensure_account_exists(id),
    }
}

pub fn ensure_account_resource_ids_exist(
    resource: &ResourceIds,
) -> Result<(), RecordNotFoundError> {
    match resource {
        ResourceIds::Any => Ok(()),
        ResourceIds::Ids(ids) => {
            for id in ids {
                ensure_account_exists(id)?;
            }
            Ok(())
        }
    }
}

pub fn ensure_address_book_entry_exists(id: &UUID) -> Result<(), RecordNotFoundError> {
    ensure_entry_exists(
        ADDRESS_BOOK_REPOSITORY.to_owned(),
        AddressBookEntryKey { id: *id },
    )
    .ok_or(RecordNotFoundError {
        model_name: "AddressBookEntry".to_string(),
        id: Uuid::from_bytes(*id).hyphenated().to_string(),
    })
}

pub fn ensure_address_book_entry_resource_id_exists(
    resource_id: &ResourceId,
) -> Result<(), RecordNotFoundError> {
    match resource_id {
        ResourceId::Any => Ok(()),
        ResourceId::Id(id) => ensure_address_book_entry_exists(id),
    }
}

pub fn ensure_address_book_entry_resource_ids_exist(
    resource: &ResourceIds,
) -> Result<(), RecordNotFoundError> {
    match resource {
        ResourceIds::Any => Ok(()),
        ResourceIds::Ids(ids) => {
            for id in ids {
                ensure_address_book_entry_exists(id)?;
            }
            Ok(())
        }
    }
}

pub fn ensure_proposal_exists(id: &UUID) -> Result<(), RecordNotFoundError> {
    ensure_entry_exists(PROPOSAL_REPOSITORY.to_owned(), ProposalKey { id: *id }).ok_or(
        RecordNotFoundError {
            model_name: "Proposal".to_string(),
            id: Uuid::from_bytes(*id).hyphenated().to_string(),
        },
    )
}

pub fn ensure_proposal_resource_id_exists(
    resource_id: &ResourceId,
) -> Result<(), RecordNotFoundError> {
    match resource_id {
        ResourceId::Any => Ok(()),
        ResourceId::Id(id) => ensure_proposal_exists(id),
    }
}

pub fn ensure_proposal_resource_ids_exist(
    resource: &ResourceIds,
) -> Result<(), RecordNotFoundError> {
    match resource {
        ResourceIds::Any => Ok(()),
        ResourceIds::Ids(ids) => {
            for id in ids {
                ensure_proposal_exists(id)?;
            }
            Ok(())
        }
    }
}

pub fn ensure_proposal_policy_exists(id: &UUID) -> Result<(), RecordNotFoundError> {
    ensure_entry_exists(PROPOSAL_POLICY_REPOSITORY.to_owned(), *id).ok_or(RecordNotFoundError {
        model_name: "ProposalPolicy".to_string(),
        id: Uuid::from_bytes(*id).hyphenated().to_string(),
    })
}

pub fn ensure_proposal_policy_resource_id_exists(
    resource_id: &ResourceId,
) -> Result<(), RecordNotFoundError> {
    match resource_id {
        ResourceId::Any => Ok(()),
        ResourceId::Id(id) => ensure_proposal_policy_exists(id),
    }
}

pub fn ensure_proposal_policy_resource_ids_exist(
    resource: &ResourceIds,
) -> Result<(), RecordNotFoundError> {
    match resource {
        ResourceIds::Any => Ok(()),
        ResourceIds::Ids(ids) => {
            for id in ids {
                ensure_proposal_policy_exists(id)?;
            }
            Ok(())
        }
    }
}

pub fn ensure_access_policy_exists(key: &Resource) -> Result<(), RecordNotFoundError> {
    ensure_entry_exists(ACCESS_POLICY_REPOSITORY.to_owned(), key.to_owned()).ok_or(
        RecordNotFoundError {
            model_name: "AccessPolicy".to_string(),
            id: key.to_string(),
        },
    )
}
