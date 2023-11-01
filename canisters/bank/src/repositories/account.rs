use super::indexes::account_identity_index::AccountIdentityIndexRepository;
use crate::{
    core::{with_memory_manager, Memory, ACCOUNT_MEMORY_ID},
    models::{indexes::account_identity_index::AccountIdentityIndexCriteria, Account, AccountKey},
};
use candid::Principal;
use ic_canister_core::repository::IndexRepository;
use ic_canister_core::repository::Repository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::cell::RefCell;

thread_local! {
  /// The memory reference to the Account repository.
  static DB: RefCell<StableBTreeMap<AccountKey, Account, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(ACCOUNT_MEMORY_ID))
    )
  })
}

/// A repository that enables managing accounts in stable memory.
#[derive(Default, Debug)]
pub struct AccountRepository {
    identity_index: AccountIdentityIndexRepository,
}

impl Repository<AccountKey, Account> for AccountRepository {
    fn get(&self, key: &AccountKey) -> Option<Account> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: AccountKey, value: Account) -> Option<Account> {
        DB.with(|m| match m.borrow_mut().insert(key, value.clone()) {
            Some(prev) => {
                let prev_identities = prev.to_index_for_identities();
                let curr_identities = value.to_index_for_identities();
                if prev_identities != curr_identities {
                    prev_identities.iter().for_each(|index| {
                        self.identity_index.remove(index);
                    });
                    curr_identities.iter().for_each(|index| {
                        self.identity_index.insert(index.to_owned());
                    });
                }

                Some(prev)
            }
            None => {
                value.to_index_for_identities().iter().for_each(|index| {
                    self.identity_index.insert(index.to_owned());
                });

                None
            }
        })
    }

    fn remove(&self, key: &AccountKey) -> Option<Account> {
        DB.with(|m| match m.borrow_mut().remove(key) {
            Some(prev) => {
                prev.to_index_for_identities().iter().for_each(|index| {
                    self.identity_index.remove(index);
                });

                Some(prev)
            }
            None => None,
        })
    }
}

impl AccountRepository {
    /// Returns the account associated with the given identity if it exists.
    pub fn find_account_by_identity(&self, identity: &Principal) -> Option<Account> {
        self.identity_index
            .find_by_criteria(AccountIdentityIndexCriteria {
                identity_id: identity.to_owned(),
            })
            .iter()
            .find_map(|id| self.get(&Account::key(*id)))
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
    fn test_find_by_identity() {
        let repository = AccountRepository::default();
        let mut account = account_test_utils::mock_account();
        account.identities = vec![Principal::anonymous()];
        repository.insert(account.to_key(), account.clone());

        let result = repository.find_account_by_identity(&Principal::anonymous());

        assert!(result.is_some());
    }
}
