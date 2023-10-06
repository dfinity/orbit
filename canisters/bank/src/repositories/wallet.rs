use super::indexes::wallet_account_index::WalletAccountIndexRepository;
use crate::{
    core::{with_memory_manager, Memory, WALLET_MEMORY_ID},
    models::{
        indexes::wallet_account_index::WalletAccountIndexCriteria, AccountId, Wallet, WalletKey,
    },
};
use ic_canister_core::repository::IndexRepository;
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
pub struct WalletRepository {
    account_index: WalletAccountIndexRepository,
}

impl Repository<WalletKey, Wallet> for WalletRepository {
    fn get(&self, key: &WalletKey) -> Option<Wallet> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: WalletKey, value: Wallet) -> Option<Wallet> {
        DB.with(|m| {
            let account_index = WalletAccountIndexRepository::default();
            value.to_index_by_accounts().iter().for_each(|index| {
                account_index.insert(index.to_owned());
            });

            m.borrow_mut().insert(key, value)
        })
    }

    fn remove(&self, key: &WalletKey) -> Option<Wallet> {
        DB.with(|m| m.borrow_mut().remove(key))
    }
}

impl WalletRepository {
    pub fn find_by_account_id(&self, account_id: AccountId) -> Vec<Wallet> {
        self.account_index
            .find_by_criteria(WalletAccountIndexCriteria {
                account_id: account_id.to_owned(),
            })
    }
}
