use crate::{
    core::{with_memory_manager, Memory, Repository, ACCOUNT_MEMORY_ID, UUID},
    entities::{Account, AccountKey},
    errors::AccountRepositoryError,
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::cell::RefCell;

/// The storage schema for the Account repository.
pub type AccountStorageSchema = StableBTreeMap<AccountKey, Account, VirtualMemory<Memory>>;

thread_local! {
  /// The memory reference to the Account repository.
  static DB: RefCell<AccountStorageSchema> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(ACCOUNT_MEMORY_ID))
    )
  })
}

/// A repository that enables managing accounts in stable memory.
#[derive(Default)]
pub struct AccountRepository {}

/// Enables the initialization of the AccountRepository repository.
impl AccountRepository {
    pub fn find_by_id(&self, account_id: &UUID) -> Result<Option<Account>, AccountRepositoryError> {
        DB.with(|m| {
            let start_key = AccountKey { id: *account_id };
            let end_key = AccountKey { id: *account_id };

            let results = m
                .borrow_mut()
                .range(start_key..=end_key)
                .map(|(_, account)| account)
                .collect::<Vec<Account>>();

            match results.len() {
                0 => Ok(None),
                1 => Ok(Some(results.first().unwrap().clone())),
                _ => Err(AccountRepositoryError::NotAllowedMultipleAccountsWithSameId),
            }
        })
    }
}

impl Repository<AccountKey, Account> for AccountRepository {
    fn get(&self, key: &AccountKey) -> Option<Account> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: AccountKey, value: Account) -> Option<Account> {
        DB.with(|m| m.borrow_mut().insert(key, value))
    }

    fn remove(&self, key: &AccountKey) -> Option<Account> {
        DB.with(|m| m.borrow_mut().remove(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn init_account_repository() {
        let repository = AccountRepository::default();
        let record = repository.get(&AccountKey {
            id: Uuid::new_v4().as_bytes().to_owned(),
        });
        assert!(record.is_none());
    }

    #[test]
    fn insert_account() {
        let repository = AccountRepository::default();
        let key = AccountKey::default();
        let record = Account::default();
        let result = repository.insert(key.clone(), record.clone());
        let inserted_record = repository.get(&key);

        assert!(result.is_none());
        assert!(inserted_record.is_some());
        assert_eq!(record, inserted_record.unwrap());
    }

    #[test]
    fn insert_account_with_same_key_overrides_and_returns_previous() {
        let repository = AccountRepository::default();
        let key = AccountKey::default();
        let previous_record = Account::default();

        repository.insert(key.clone(), previous_record.clone());

        let new_record = Account {
            name: Some(String::from("new name")),
            ..Default::default()
        };
        let result = repository.insert(key.clone(), new_record.clone());

        assert!(result.is_some());
        assert_eq!(previous_record, result.unwrap());
    }

    #[test]
    fn removes_inserted_account() {
        let repository = AccountRepository::default();
        let key = AccountKey::default();
        let record = Account::default();

        repository.insert(key.clone(), record.clone());

        let before_delete_result = repository.get(&key);
        let removed_value = repository.remove(&key);
        let after_delete_result = repository.get(&key);

        assert!(before_delete_result.is_some());
        assert!(after_delete_result.is_none());
        assert_eq!(removed_value.unwrap(), record);
    }
}
