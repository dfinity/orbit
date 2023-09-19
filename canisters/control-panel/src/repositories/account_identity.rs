use crate::{
    core::{with_memory_manager, Memory, Repository, ACCOUNT_IDENTITY_MEMOTY_ID},
    entities::{AccountIdentity, AccountIdentityKey},
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::cell::RefCell;

/// The storage schema for the AccountIdentity repository.
pub type AccountIdentityStorageSchema =
    StableBTreeMap<AccountIdentityKey, AccountIdentity, VirtualMemory<Memory>>;

thread_local! {
  /// The memory reference to the AccountIdentity repository.
  static DB: RefCell<AccountIdentityStorageSchema> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(ACCOUNT_IDENTITY_MEMOTY_ID))
    )
  })
}

/// A repository that enables managing domain zones in stable memory.
pub struct AccountIdentityRepository {}

/// Enables the initialization of the AccountIdentityRepository repository.
impl AccountIdentityRepository {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for AccountIdentityRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl Repository<AccountIdentityKey, AccountIdentity> for AccountIdentityRepository {
    fn get(&self, key: &AccountIdentityKey) -> Option<AccountIdentity> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: AccountIdentityKey, value: AccountIdentity) -> Option<AccountIdentity> {
        DB.with(|m| m.borrow_mut().insert(key, value))
    }

    fn remove(&self, key: &AccountIdentityKey) -> Option<AccountIdentity> {
        DB.with(|m| m.borrow_mut().remove(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::AccountIdentityStatus;
    use candid::Principal;

    #[test]
    fn init_account_identity_repository() {
        let repository = AccountIdentityRepository::default();
        let identity = repository.get(&AccountIdentityKey {
            id: Principal::anonymous(),
        });
        assert!(identity.is_none());
    }

    #[test]
    fn insert_account_identity() {
        let repository = AccountIdentityRepository::default();
        let key = AccountIdentityKey {
            id: Principal::anonymous(),
        };
        let identity = AccountIdentity {
            id: key.id,
            name: None,
            status: AccountIdentityStatus::PendingActivation,
            last_update_timestamp: Default::default(),
        };

        let result = repository.insert(key.clone(), identity.clone());
        let inserted_record = repository.get(&key);

        assert!(result.is_none());
        assert!(inserted_record.is_some());
        assert_eq!(identity, inserted_record.unwrap());
    }

    #[test]
    fn insert_identity_with_same_key_overrides_and_returns_previous() {
        let repository = AccountIdentityRepository::default();
        let key = AccountIdentityKey {
            id: Principal::anonymous(),
        };
        let previous_identity = AccountIdentity {
            id: key.id,
            name: None,
            status: AccountIdentityStatus::PendingActivation,
            last_update_timestamp: Default::default(),
        };

        repository.insert(key.clone(), previous_identity.clone());

        let new_identity = AccountIdentity {
            id: key.id,
            name: Some(String::from("test")),
            status: AccountIdentityStatus::PendingActivation,
            last_update_timestamp: Default::default(),
        };

        let result = repository.insert(key.clone(), new_identity.clone());

        assert!(result.is_some());
        assert_eq!(previous_identity, result.unwrap());
    }

    #[test]
    fn removes_inserted_identity() {
        let repository = AccountIdentityRepository::default();
        let key = AccountIdentityKey {
            id: Principal::anonymous(),
        };
        let identity = AccountIdentity {
            id: key.id,
            name: None,
            status: AccountIdentityStatus::PendingActivation,
            last_update_timestamp: Default::default(),
        };

        repository.insert(key.clone(), identity.clone());

        let before_delete_result = repository.get(&key);
        let removed_value = repository.remove(&key);
        let after_delete_result = repository.get(&key);

        assert!(before_delete_result.is_some());
        assert!(after_delete_result.is_none());
        assert_eq!(removed_value.unwrap(), identity);
    }
}
