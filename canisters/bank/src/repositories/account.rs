use crate::{
    core::{with_memory_manager, Memory, ACCOUNT_MEMORY_ID},
    models::{indexes::account_identity_index::AccountIdentityIndexCriteria, Account, AccountKey},
};
use candid::Principal;
use ic_canister_core::repository::IndexRepository;
use ic_canister_core::repository::Repository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::cell::RefCell;

use super::indexes::account_identity_index::AccountIdentityIndexRepository;

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
    account_identity_index: AccountIdentityIndexRepository,
}

impl Repository<AccountKey, Account> for AccountRepository {
    fn get(&self, key: &AccountKey) -> Option<Account> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: AccountKey, value: Account) -> Option<Account> {
        DB.with(|m| {
            let identity_index = AccountIdentityIndexRepository::default();
            value.to_index_for_identities().iter().for_each(|index| {
                identity_index.insert(index.to_owned());
            });

            m.borrow_mut().insert(key, value)
        })
    }

    fn remove(&self, key: &AccountKey) -> Option<Account> {
        DB.with(|m| m.borrow_mut().remove(key))
    }
}

impl AccountRepository {
    /// Returns the account associated with the given identity if it exists.
    pub fn find_account_by_identity(&self, identity: &Principal) -> Option<Account> {
        let results = self
            .account_identity_index
            .find_by_criteria(AccountIdentityIndexCriteria {
                identity_id: identity.to_owned(),
                role: None,
            });

        results.first().map(|account| account.to_owned())
    }
}
