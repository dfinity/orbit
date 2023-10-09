use super::indexes::{
    transfer_execution_time_index::TransferExecutionTimeIndexRepository,
    transfer_wallet_index::TransferWalletIndexRepository,
};
use crate::{
    core::{with_memory_manager, Memory, TRANSFER_MEMORY_ID},
    models::{Transfer, TransferKey},
};
use ic_canister_core::repository::{IndexRepository, Repository};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::cell::RefCell;

thread_local! {
  /// The memory reference to the Transfer repository.
  static DB: RefCell<StableBTreeMap<TransferKey, Transfer, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(TRANSFER_MEMORY_ID))
    )
  })
}

/// A repository that enables managing transfer in stable memory.
#[derive(Default, Debug)]
pub struct TransferRepository {
    wallet_index: TransferWalletIndexRepository,
    execution_dt_index: TransferExecutionTimeIndexRepository,
}

impl Repository<TransferKey, Transfer> for TransferRepository {
    fn get(&self, key: &TransferKey) -> Option<Transfer> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: TransferKey, value: Transfer) -> Option<Transfer> {
        DB.with(|m| match m.borrow_mut().insert(key, value.clone()) {
            Some(prev) => {
                let prev_wallet_index = prev.to_index_by_wallet();
                if prev_wallet_index != value.to_index_by_wallet() {
                    self.wallet_index.remove(&prev_wallet_index);
                    self.wallet_index.insert(value.to_index_by_wallet());
                }
                let prev_execution_dt_index = prev.to_index_by_execution_dt();
                if prev_execution_dt_index != value.to_index_by_execution_dt() {
                    self.execution_dt_index.remove(&prev_execution_dt_index);
                    self.execution_dt_index
                        .insert(value.to_index_by_execution_dt());
                }

                Some(prev)
            }
            None => {
                self.wallet_index.insert(value.to_index_by_wallet());
                self.execution_dt_index
                    .insert(value.to_index_by_execution_dt());

                None
            }
        })
    }

    fn remove(&self, key: &TransferKey) -> Option<Transfer> {
        DB.with(|m| match m.borrow_mut().remove(key) {
            Some(prev) => {
                let wallet_index = TransferWalletIndexRepository::default();
                let execution_dt_index = TransferExecutionTimeIndexRepository::default();

                wallet_index.remove(&prev.to_index_by_wallet());
                execution_dt_index.remove(&prev.to_index_by_execution_dt());

                Some(prev)
            }
            None => None,
        })
    }
}
