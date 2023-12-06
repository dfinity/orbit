use super::indexes::account_user_index::AccountUserIndexRepository;
use crate::{
    core::{with_memory_manager, Memory, ACCOUNT_MEMORY_ID},
    models::{
        indexes::account_user_index::AccountUserIndexCriteria, Account, AccountId, AccountKey,
        UserId,
    },
};
use ic_canister_core::repository::IndexRepository;
use ic_canister_core::repository::Repository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use std::cell::RefCell;

thread_local! {
  /// The memory reference to the Account repository.
  static DB: RefCell<StableBTreeMap<AccountKey, Account, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(ACCOUNT_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref ACCOUNT_REPOSITORY: AccountRepository = AccountRepository::default();
}

/// A repository that enables managing accounts in stable memory.
#[derive(Default, Debug)]
pub struct AccountRepository {
    user_index: AccountUserIndexRepository,
}

impl Repository<AccountKey, Account> for AccountRepository {
    fn list(&self) -> Vec<Account> {
        DB.with(|m| m.borrow().iter().map(|(_, v)| v).collect())
    }

    fn get(&self, key: &AccountKey) -> Option<Account> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: AccountKey, value: Account) -> Option<Account> {
        DB.with(|m| match m.borrow_mut().insert(key, value.clone()) {
            Some(prev) => {
                let prev_users = prev.to_index_by_users();
                let curr_users = value.to_index_by_users();

                if prev_users != curr_users {
                    prev_users.iter().for_each(|index| {
                        self.user_index.remove(index);
                    });
                    curr_users.iter().for_each(|index| {
                        self.user_index.insert(index.to_owned());
                    });
                }

                Some(prev)
            }
            None => {
                value.to_index_by_users().iter().for_each(|index| {
                    self.user_index.insert(index.to_owned());
                });

                None
            }
        })
    }

    fn remove(&self, key: &AccountKey) -> Option<Account> {
        DB.with(|m| match m.borrow_mut().remove(key) {
            Some(account) => {
                account.to_index_by_users().iter().for_each(|index| {
                    self.user_index.remove(index);
                });

                Some(account)
            }
            None => None,
        })
    }
}

impl AccountRepository {
    pub fn find_by_user_id(&self, user_id: UserId) -> Vec<Account> {
        let account_ids = self.user_index.find_by_criteria(AccountUserIndexCriteria {
            user_id: user_id.to_owned(),
        });

        account_ids
            .iter()
            .filter_map(|id| self.get(&Account::key(*id)))
            .collect::<Vec<_>>()
    }

    pub fn find_by_ids(&self, ids: Vec<AccountId>) -> Vec<Account> {
        ids.iter()
            .filter_map(|id| self.get(&Account::key(*id)))
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::account_test_utils;

    #[test]
    fn test_crud() {
        let repository = AccountRepository::default();
        let account = account_test_utils::mock_account();

        assert!(repository.get(&account.to_key()).is_none());

        repository.insert(account.to_key(), account.clone());

        assert!(repository.get(&account.to_key()).is_some());
        assert!(repository.remove(&account.to_key()).is_some());
        assert!(repository.get(&account.to_key()).is_none());
    }

    #[test]
    fn test_find_by_user_id() {
        let repository = AccountRepository::default();
        let mut account = account_test_utils::mock_account();
        account.owners = vec![[1; 16]];

        repository.insert(account.to_key(), account.clone());

        assert_eq!(repository.find_by_user_id([1; 16]), vec![account]);
    }

    #[test]
    fn test_find_by_ids() {
        let repository = AccountRepository::default();
        let mut account1 = account_test_utils::mock_account();
        let mut account2 = account_test_utils::mock_account();
        account1.id = [1; 16];
        account2.id = [2; 16];

        repository.insert(account1.to_key(), account1.clone());
        repository.insert(account2.to_key(), account2.clone());

        assert_eq!(
            repository.find_by_ids(vec![account1.id, account2.id]),
            vec![account1, account2]
        );
    }
}
