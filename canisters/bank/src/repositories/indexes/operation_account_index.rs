use crate::{
    core::{with_memory_manager, Memory, OPERATION_ACCOUNT_INDEX_MEMORY_ID},
    models::{
        indexes::operation_account_index::{OperationAccountIndex, OperationAccountIndexCriteria},
        OperationId,
    },
};
use ic_canister_core::repository::IndexRepository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<OperationAccountIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(OPERATION_ACCOUNT_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables finding operations based on the account in stable memory.
#[derive(Default, Debug)]
pub struct OperationAccountIndexRepository {}

impl IndexRepository<OperationAccountIndex, OperationId> for OperationAccountIndexRepository {
    type FindByCriteria = OperationAccountIndexCriteria;

    fn exists(&self, index: &OperationAccountIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: OperationAccountIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &OperationAccountIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<OperationId> {
        DB.with(|db| {
            let start_key = OperationAccountIndex {
                account_id: criteria.account_id.to_owned(),
                created_at: criteria.from_dt.to_owned().unwrap_or(u64::MIN),
                id: [u8::MIN; 16],
            };
            let end_key = OperationAccountIndex {
                account_id: criteria.account_id.to_owned(),
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
