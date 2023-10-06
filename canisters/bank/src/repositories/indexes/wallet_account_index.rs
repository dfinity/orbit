use crate::{
    core::{with_memory_manager, Memory, WALLET_ACCOUNT_INDEX_MEMORY_ID},
    models::{
        indexes::wallet_account_index::{WalletAccountIndex, WalletAccountIndexCriteria},
        Wallet,
    },
};
use ic_canister_core::repository::IndexRepository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::cell::RefCell;

thread_local! {
  static DB: RefCell<StableBTreeMap<WalletAccountIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(WALLET_ACCOUNT_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables finding wallets based on the id of the account in stable memory.
#[derive(Default, Debug)]
pub struct WalletAccountIndexRepository {}

impl IndexRepository<WalletAccountIndex, Wallet> for WalletAccountIndexRepository {
    type FindByCriteria = WalletAccountIndexCriteria;

    fn exists(&self, index: &WalletAccountIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: WalletAccountIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &WalletAccountIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> Vec<Wallet> {
        DB.with(|db| {
            let start_key = WalletAccountIndex {
                account_id: criteria.account_id,
                wallet_id: [u8::MIN; 16],
            };
            let end_key = WalletAccountIndex {
                account_id: criteria.account_id,
                wallet_id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.to_wallet())
                .collect::<Vec<Wallet>>()
        })
    }
}
