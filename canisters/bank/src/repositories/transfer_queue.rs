use crate::{
    core::{with_memory_manager, Memory, TRANSFER_QUEUE_MEMORY_ID},
    models::{TransferQueue, TransferQueueKey},
};
use ic_canister_core::{repository::Repository, types::Timestamp};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::cell::RefCell;

thread_local! {
  /// The memory reference to the Transfer repository.
  static DB: RefCell<StableBTreeMap<TransferQueueKey, TransferQueue, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(TRANSFER_QUEUE_MEMORY_ID))
    )
  })
}

/// A repository that enables managing transfer in stable memory.
#[derive(Default, Debug)]
pub struct TransferQueueRepository {}

impl Repository<TransferQueueKey, TransferQueue> for TransferQueueRepository {
    fn get(&self, key: &TransferQueueKey) -> Option<TransferQueue> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: TransferQueueKey, value: TransferQueue) -> Option<TransferQueue> {
        DB.with(|m| m.borrow_mut().insert(key, value))
    }

    fn remove(&self, key: &TransferQueueKey) -> Option<TransferQueue> {
        DB.with(|m| m.borrow_mut().remove(key))
    }
}

impl TransferQueueRepository {
    pub fn find_all_until_execution_dt(&self, execution_dt: &Timestamp) -> Vec<TransferQueue> {
        DB.with(|db| {
            let end_key = TransferQueue::key(*execution_dt, [std::u8::MAX; 16]);

            db.borrow()
                .range(..=end_key)
                .map(|(_, queue_item)| queue_item)
                .collect::<Vec<TransferQueue>>()
        })
    }
}
