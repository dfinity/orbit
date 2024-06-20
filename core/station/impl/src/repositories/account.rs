use super::indexes::name_to_account_id_index::NameToAccountIdIndexRepository;
use crate::{
    core::{
        metrics::ACCOUNT_METRICS, observer::Observer, with_memory_manager, Memory,
        ACCOUNT_MEMORY_ID,
    },
    models::{
        indexes::name_to_account_id_index::NameToAccountIdIndexCriteria, Account, AccountId,
        AccountKey,
    },
    services::disaster_recovery_observes_insert_account,
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use orbit_essentials::repository::IndexRepository;
use orbit_essentials::repository::{RefreshIndexMode, Repository};
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
#[derive(Debug)]
pub struct AccountRepository {
    name_index: NameToAccountIdIndexRepository,
    change_observer: Observer<(Account, Option<Account>)>,
}

impl Default for AccountRepository {
    fn default() -> Self {
        let mut change_observer = Observer::default();
        disaster_recovery_observes_insert_account(&mut change_observer);

        Self {
            change_observer,
            name_index: NameToAccountIdIndexRepository::default(),
        }
    }
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

            // Update metrics when an account is upserted.
            ACCOUNT_METRICS.with(|metrics| {
                metrics
                    .iter()
                    .for_each(|metric| metric.borrow_mut().sum(&value, prev.as_ref()))
            });

            self.name_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev.clone().map(|prev| prev.to_index_by_name()),
                    current: Some(value.to_index_by_name()),
                });

            let args = (value, prev);
            self.change_observer.notify(&args);

            args.1
        })
    }

    fn remove(&self, key: &AccountKey) -> Option<Account> {
        DB.with(|m| {
            let prev = m.borrow_mut().remove(key);

            // Update metrics when an account is removed.
            if let Some(prev) = &prev {
                ACCOUNT_METRICS.with(|metrics| {
                    metrics
                        .iter()
                        .for_each(|metric| metric.borrow_mut().sub(prev))
                });
            }

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

        accounts.sort_by(|a, b| a.name.cmp(&b.name));

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

    pub fn with_empty_observers() -> Self {
        Self {
            change_observer: Observer::default(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct AccountWhereClause {
    pub search_term: Option<String>,
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
