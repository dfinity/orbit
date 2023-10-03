use crate::{
    core::{with_memory_manager, Memory, WALLET_MEMORY_ID},
    models::{Wallet, WalletKey},
};
use ic_canister_core::repository::Repository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::cell::RefCell;

thread_local! {
  /// The memory reference to the Wallet repository.
  static DB: RefCell<StableBTreeMap<WalletKey, Wallet, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(WALLET_MEMORY_ID))
    )
  })
}

/// A repository that enables managing wallets in stable memory.
#[derive(Default, Debug)]
pub struct WalletRepository {}

impl Repository<WalletKey, Wallet> for WalletRepository {
    fn get(&self, key: &WalletKey) -> Option<Wallet> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: WalletKey, value: Wallet) -> Option<Wallet> {
        DB.with(|m| m.borrow_mut().insert(key, value))
    }

    fn remove(&self, key: &WalletKey) -> Option<Wallet> {
        DB.with(|m| m.borrow_mut().remove(key))
    }
}
