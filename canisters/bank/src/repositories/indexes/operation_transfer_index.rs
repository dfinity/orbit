use crate::{
    core::{with_memory_manager, Memory, OPERATION_TRANSFER_INDEX_MEMORY_ID},
    models::{
        indexes::operation_transfer_index::{
            OperationTransferIndex, OperationTransferIndexCriteria,
        },
        OperationId,
    },
};
use ic_canister_core::repository::IndexRepository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<OperationTransferIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(OPERATION_TRANSFER_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables finding operations based on the transfer in stable memory.
#[derive(Default, Debug)]
pub struct OperationTransferIndexRepository {}

impl IndexRepository<OperationTransferIndex, OperationId> for OperationTransferIndexRepository {
    type FindByCriteria = OperationTransferIndexCriteria;

    fn exists(&self, key: &OperationTransferIndex) -> bool {
        DB.with(|m| m.borrow().get(key).is_some())
    }

    fn insert(&self, key: OperationTransferIndex) {
        DB.with(|m| m.borrow_mut().insert(key, ()));
    }

    fn remove(&self, key: &OperationTransferIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(key).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<OperationId> {
        DB.with(|db| {
            let start_key = OperationTransferIndex {
                transfer_id: criteria.transfer_id.to_owned(),
                created_at: criteria.from_dt.to_owned().unwrap_or(u64::MIN),
                id: [u8::MIN; 16],
            };
            let end_key = OperationTransferIndex {
                transfer_id: criteria.transfer_id.to_owned(),
                created_at: criteria.to_dt.to_owned().unwrap_or(u64::MAX),
                id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.id)
                .collect::<HashSet<OperationId>>()
        })
    }
}
