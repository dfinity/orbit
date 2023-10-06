use crate::{
    core::{with_memory_manager, Memory, TRANSFER_EXECUTION_TIME_INDEX_MEMORY_ID},
    models::{
        indexes::transfer_execution_time_index::{
            TransferExecutionTimeIndex, TransferExecutionTimeIndexCriteria,
        },
        Transfer,
    },
};
use ic_canister_core::repository::IndexRepository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::cell::RefCell;

thread_local! {
  /// The memory reference to the Transfer repository.
  static DB: RefCell<StableBTreeMap<TransferExecutionTimeIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(TRANSFER_EXECUTION_TIME_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables managing transfer in stable memory.
#[derive(Default, Debug)]
pub struct TransferExecutionTimeIndexRepository {}

impl IndexRepository<TransferExecutionTimeIndex, Transfer>
    for TransferExecutionTimeIndexRepository
{
    type FindByCriteria = TransferExecutionTimeIndexCriteria;

    fn exists(&self, index: &TransferExecutionTimeIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: TransferExecutionTimeIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &TransferExecutionTimeIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> Vec<Transfer> {
        DB.with(|db| {
            let start_key = TransferExecutionTimeIndex {
                execution_dt: criteria.from_dt.to_owned().unwrap_or(u64::MIN),
                transfer_id: [std::u8::MIN; 16],
            };
            let end_key = TransferExecutionTimeIndex {
                execution_dt: criteria.to_dt,
                transfer_id: [std::u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .filter(|(index, _)| {
                    let transfer = index.to_transfer();
                    let mut matches_criteria_status = true;
                    if let Some(status) = criteria.status.as_ref() {
                        matches_criteria_status = transfer.status.to_string() == *status;
                    }

                    matches_criteria_status
                })
                .map(|(index, _)| index.to_transfer())
                .collect::<Vec<Transfer>>()
        })
    }
}
