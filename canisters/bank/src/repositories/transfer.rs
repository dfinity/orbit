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
pub struct TransferRepository {}

impl Repository<TransferKey, Transfer> for TransferRepository {
    fn get(&self, key: &TransferKey) -> Option<Transfer> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: TransferKey, value: Transfer) -> Option<Transfer> {
        DB.with(|m| {
            TransferWalletIndexRepository::default().insert(value.to_index_by_wallet());
            TransferExecutionTimeIndexRepository::default()
                .insert(value.to_index_by_execution_dt());

            m.borrow_mut().insert(key, value)
        })
    }

    fn remove(&self, key: &TransferKey) -> Option<Transfer> {
        DB.with(|m| m.borrow_mut().remove(key))
    }
}
