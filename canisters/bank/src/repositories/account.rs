use crate::{
    core::{with_memory_manager, Memory, ACCOUNT_MEMORY_ID},
    models::{Account, AccountKey},
};
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
#[derive(Default)]
pub struct AccountRepository {}

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
