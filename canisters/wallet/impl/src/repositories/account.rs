use super::indexes::{
    account_user_index::AccountUserIndexRepository,
    name_to_account_id_index::NameToAccountIdIndexRepository,
};
use crate::{
    core::{with_memory_manager, Memory, ACCOUNT_MEMORY_ID},
    models::{
        indexes::{
            account_user_index::AccountUserIndexCriteria,
            name_to_account_id_index::NameToAccountIdIndexCriteria,
        },
        Account, AccountId, AccountKey, UserId,
    },
};
use ic_canister_core::repository::{RefreshIndexMode, Repository};
use ic_canister_core::{repository::IndexRepository, types::UUID};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use std::{cell::RefCell, sync::Arc};

thread_local! {
  /// The memory reference to the Account repository.
  static DB: RefCell<StableBTreeMap<AccountKey, Account, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(ACCOUNT_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref ACCOUNT_REPOSITORY: Arc<AccountRepository> =
        Arc::new(AccountRepository::default());
}

/// A repository that enables managing accounts in stable memory.
#[derive(Default, Debug)]
pub struct AccountRepository {
    user_index: AccountUserIndexRepository,
    name_index: NameToAccountIdIndexRepository,
}

impl Repository<AccountKey, Account> for AccountRepository {
    fn list(&self) -> Vec<Account> {
        DB.with(|m| m.borrow().iter().map(|(_, v)| v).collect())
    }

    fn get(&self, key: &AccountKey) -> Option<Account> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: AccountKey, value: Account) -> Option<Account> {
        DB.with(|m| {
            let prev = m.borrow_mut().insert(key, value.clone());

            self.user_index
                .refresh_index_on_modification(RefreshIndexMode::List {
                    previous: prev
                        .clone()
                        .map_or(Vec::new(), |prev| prev.to_index_by_users()),
                    current: value.to_index_by_users(),
                });

            self.name_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev.clone().map(|prev| prev.to_index_by_name()),
                    current: Some(value.to_index_by_name()),
                });

            prev
        })
    }

    fn remove(&self, key: &AccountKey) -> Option<Account> {
        DB.with(|m| {
            let prev = m.borrow_mut().remove(key);

            self.user_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupList {
                    current: prev
                        .clone()
                        .map_or(Vec::new(), |prev| prev.to_index_by_users()),
                });
            self.name_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev.clone().map(|prev| prev.to_index_by_name()),
                });

            prev
        })
    }

    fn len(&self) -> usize {
        DB.with(|m| m.borrow().len()) as usize
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

    pub fn find_where(&self, where_clause: AccountWhereClause) -> Vec<Account> {
        let mut accounts = self.list();

        if let Some(search_term) = where_clause.search_term {
            accounts.retain(|account| {
                account
                    .name
                    .to_lowercase()
                    .contains(&search_term.to_lowercase())
            });
        }

        if let Some(owner_user_ids) = where_clause.owner_user_ids {
            accounts.retain(|account| {
                account
                    .owners
                    .iter()
                    .any(|owner| owner_user_ids.contains(owner))
            });
        }

        accounts.sort();

        accounts
    }

    pub fn find_account_id_by_name(&self, name: &str) -> Option<AccountId> {
        self.name_index
            .find_by_criteria(NameToAccountIdIndexCriteria {
                name: name.to_owned(),
            })
            .into_iter()
            .next()
    }
}

#[derive(Debug, Clone)]
pub struct AccountWhereClause {
    pub search_term: Option<String>,
    pub owner_user_ids: Option<Vec<UUID>>,
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
