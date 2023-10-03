use super::indexes::{
    operation_account_index::OperationAccountIndexRepository,
    operation_transfer_index::OperationTransferIndexRepository,
    operation_wallet_index::OperationWalletIndexRepository,
};
use crate::{
    core::{with_memory_manager, Memory, OPERATION_MEMORY_ID},
    models::{Operation, OperationKey},
};
use ic_canister_core::repository::{IndexRepository, Repository};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::cell::RefCell;

thread_local! {
  static DB: RefCell<StableBTreeMap<OperationKey, Operation, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(OPERATION_MEMORY_ID))
    )
  })
}

/// A repository that enables managing system operations in stable memory.
#[derive(Default, Debug)]
pub struct OperationRepository {}

impl Repository<OperationKey, Operation> for OperationRepository {
    fn get(&self, key: &OperationKey) -> Option<Operation> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: OperationKey, value: Operation) -> Option<Operation> {
        DB.with(|m| {
            OperationAccountIndexRepository::default().insert(value.as_index_for_account());
            OperationWalletIndexRepository::default().insert(value.as_index_for_wallet());
            OperationTransferIndexRepository::default().insert(value.as_index_for_transfer());

            m.borrow_mut().insert(key, value)
        })
    }

    fn remove(&self, key: &OperationKey) -> Option<Operation> {
        DB.with(|m| m.borrow_mut().remove(key))
    }
}
