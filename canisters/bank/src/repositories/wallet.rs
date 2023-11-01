use super::indexes::wallet_account_index::WalletAccountIndexRepository;
use crate::{
    core::{with_memory_manager, Memory, WALLET_MEMORY_ID},
    models::{
        indexes::wallet_account_index::WalletAccountIndexCriteria, AccountId, Wallet, WalletId,
        WalletKey,
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
        DB.with(|m| match m.borrow_mut().insert(key, value.clone()) {
            Some(prev) => {
                let prev_accounts = prev.to_index_by_accounts();
                let curr_accounts = value.to_index_by_accounts();

                if prev_accounts != curr_accounts {
                    prev_accounts.iter().for_each(|index| {
                        self.account_index.remove(index);
                    });
                    curr_accounts.iter().for_each(|index| {
                        self.account_index.insert(index.to_owned());
                    });
                }

                Some(prev)
            }
            None => {
                value.to_index_by_accounts().iter().for_each(|index| {
                    self.account_index.insert(index.to_owned());
                });

                None
            }
        })
    }

    fn remove(&self, key: &WalletKey) -> Option<Wallet> {
        DB.with(|m| match m.borrow_mut().remove(key) {
            Some(wallet) => {
                wallet.to_index_by_accounts().iter().for_each(|index| {
                    self.account_index.remove(index);
                });

                Some(wallet)
            }
            None => None,
        })
    }
}

impl WalletRepository {
    pub fn find_by_account_id(&self, account_id: AccountId) -> Vec<Wallet> {
        let wallet_ids = self
            .account_index
            .find_by_criteria(WalletAccountIndexCriteria {
                account_id: account_id.to_owned(),
            });

        wallet_ids
            .iter()
            .filter_map(|id| self.get(&Wallet::key(*id)))
            .collect::<Vec<_>>()
    }

    pub fn find_by_ids(&self, ids: Vec<WalletId>) -> Vec<Wallet> {
        ids.iter()
            .filter_map(|id| self.get(&Wallet::key(*id)))
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::wallet_test_utils;

    #[test]
    fn test_crud() {
        let repository = WalletRepository::default();
        let wallet = wallet_test_utils::mock_wallet();

        assert!(repository.get(&wallet.to_key()).is_none());

        repository.insert(wallet.to_key(), wallet.clone());

        assert!(repository.get(&wallet.to_key()).is_some());
        assert!(repository.remove(&wallet.to_key()).is_some());
        assert!(repository.get(&wallet.to_key()).is_none());
    }

    #[test]
    fn test_find_by_account_id() {
        let repository = WalletRepository::default();
        let mut wallet = wallet_test_utils::mock_wallet();
        wallet.owners = vec![[1; 16]];

        repository.insert(wallet.to_key(), wallet.clone());

        assert_eq!(repository.find_by_account_id([1; 16]), vec![wallet]);
    }

    #[test]
    fn test_find_by_ids() {
        let repository = WalletRepository::default();
        let mut wallet1 = wallet_test_utils::mock_wallet();
        let mut wallet2 = wallet_test_utils::mock_wallet();
        wallet1.id = [1; 16];
        wallet2.id = [2; 16];

        repository.insert(wallet1.to_key(), wallet1.clone());
        repository.insert(wallet2.to_key(), wallet2.clone());

        assert_eq!(
            repository.find_by_ids(vec![wallet1.id, wallet2.id]),
            vec![wallet1, wallet2]
        );
    }
}
