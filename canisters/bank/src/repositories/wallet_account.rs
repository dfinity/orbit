use crate::{
    core::{with_memory_manager, Memory, WALLET_ACCOUNT_MEMORY_ID},
    models::{AccountId, WalletAccount, WalletAccountKey},
};
use ic_canister_core::repository::Repository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::cell::RefCell;

thread_local! {
  /// The memory reference to the wallet account repository.
  static DB: RefCell<StableBTreeMap<WalletAccountKey, WalletAccount, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(WALLET_ACCOUNT_MEMORY_ID))
    )
  })
}

/// A repository that enables managing wallets and accounts in stable memory.
#[derive(Default, Debug)]
pub struct WalletAccountRepository {}

impl Repository<WalletAccountKey, WalletAccount> for WalletAccountRepository {
    fn get(&self, key: &WalletAccountKey) -> Option<WalletAccount> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: WalletAccountKey, value: WalletAccount) -> Option<WalletAccount> {
        DB.with(|m| m.borrow_mut().insert(key, value))
    }

    fn remove(&self, key: &WalletAccountKey) -> Option<WalletAccount> {
        DB.with(|m| m.borrow_mut().remove(key))
    }
}

impl WalletAccountRepository {
    pub fn find_by_account_id(&self, account_id: &AccountId) -> Vec<WalletAccount> {
        DB.with(|db| {
            let start_key = WalletAccount::key(account_id, &[std::u8::MIN; 16]);
            let end_key = WalletAccount::key(account_id, &[std::u8::MAX; 16]);

            db.borrow()
                .range(start_key..=end_key)
                .map(|(_, wallet_account)| wallet_account)
                .collect::<Vec<WalletAccount>>()
        })
    }
}
