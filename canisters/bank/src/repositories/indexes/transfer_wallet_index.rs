use crate::{
    core::{with_memory_manager, Memory, TRANSFER_LIST_INDEX_MEMORY_ID},
    errors::RepositoryError,
    models::{
        indexes::transfer_wallet_index::{TransferWalletIndex, TransferWalletIndexCriteria},
        Transfer,
    },
};
use ic_canister_core::{
    cdk::api::{time, trap},
    repository::IndexRepository,
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::cell::RefCell;

thread_local! {
  /// The memory reference to the Transfer repository.
  static DB: RefCell<StableBTreeMap<TransferWalletIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(TRANSFER_LIST_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables managing transfer in stable memory.
#[derive(Default, Debug)]
pub struct TransferWalletIndexRepository {}

impl IndexRepository<TransferWalletIndex, Transfer> for TransferWalletIndexRepository {
    type FindByCriteria = TransferWalletIndexCriteria;

    fn exists(&self, index: &TransferWalletIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: TransferWalletIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &TransferWalletIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> Vec<Transfer> {
        DB.with(|db| {
            let (from_dt, to_dt) = match (criteria.from_dt, criteria.to_dt) {
                (Some(start), Some(end)) => (start, end),
                (Some(start), None) => (start, time()),
                (None, Some(end)) => (end - TransferWalletIndex::DEFAULT_CRITERIA_INTERVAL_NS, end),
                _ => (
                    time() - TransferWalletIndex::DEFAULT_CRITERIA_INTERVAL_NS,
                    time(),
                ),
            };
            if from_dt > to_dt {
                trap(RepositoryError::CriteriaOutOfRange.to_string().as_str());
            }

            let start_key = TransferWalletIndex {
                wallet_id: criteria.wallet_id,
                created_timestamp: from_dt,
                transfer_id: [u8::MIN; 16],
            };
            let end_key = TransferWalletIndex {
                wallet_id: criteria.wallet_id,
                created_timestamp: to_dt,
                transfer_id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .take_while(|(index, _)| {
                    let transfer = index.to_transfer();
                    let mut criteria_matches_status = true;

                    if let Some(status) = &criteria.status {
                        criteria_matches_status = transfer.status.to_string() == *status
                    }

                    criteria_matches_status
                })
                .map(|(index, _)| index.to_transfer())
                .collect::<Vec<Transfer>>()
        })
    }
}
