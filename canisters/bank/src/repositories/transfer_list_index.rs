use crate::{
    core::{with_memory_manager, Memory, TRANSFER_LIST_INDEX_MEMORY_ID},
    errors::RepositoryError,
    models::{TransferListIndex, TransferListIndexKey, WalletId},
};
use ic_canister_core::{cdk::api::time, repository::Repository, types::Timestamp};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::cell::RefCell;

thread_local! {
  /// The memory reference to the Transfer repository.
  static DB: RefCell<StableBTreeMap<TransferListIndexKey, TransferListIndex, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(TRANSFER_LIST_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables managing transfer in stable memory.
#[derive(Default, Debug)]
pub struct TransferListIndexRepository {}

impl Repository<TransferListIndexKey, TransferListIndex> for TransferListIndexRepository {
    fn get(&self, key: &TransferListIndexKey) -> Option<TransferListIndex> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(
        &self,
        key: TransferListIndexKey,
        value: TransferListIndex,
    ) -> Option<TransferListIndex> {
        DB.with(|m| m.borrow_mut().insert(key, value))
    }

    fn remove(&self, key: &TransferListIndexKey) -> Option<TransferListIndex> {
        DB.with(|m| m.borrow_mut().remove(key))
    }
}

impl TransferListIndexRepository {
    pub fn find_all_within_criteria(
        &self,
        wallet_id: WalletId,
        from_dt: Option<Timestamp>,
        to_dt: Option<Timestamp>,
        status: Option<String>,
    ) -> Result<Vec<TransferListIndex>, RepositoryError> {
        DB.with(|db| {
            let (from_dt, to_dt) = match (from_dt, to_dt) {
                (Some(start), Some(end)) => (start, end),
                (Some(start), None) => (start, time()),
                (None, Some(end)) => (end - TransferListIndex::DEFAULT_CRITERIA_INTERVAL_NS, end),
                _ => (
                    time() - TransferListIndex::DEFAULT_CRITERIA_INTERVAL_NS,
                    time(),
                ),
            };
            if from_dt > to_dt {
                return Err(RepositoryError::CriteriaOutOfRange);
            }

            let start_key = TransferListIndex::key(wallet_id, from_dt);
            let end_key = TransferListIndex::key(wallet_id, to_dt);

            let results = db
                .borrow()
                .range(start_key..=end_key)
                .take_while(|(_, item)| {
                    if let Some(status) = &status {
                        item.status == *status
                    } else {
                        true
                    }
                })
                .map(|(_, item)| item)
                .collect::<Vec<TransferListIndex>>();

            Ok(results)
        })
    }
}
