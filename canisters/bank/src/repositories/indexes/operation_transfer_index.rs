use crate::{
    core::{with_memory_manager, Memory, OPERATION_TRANSFER_INDEX_MEMORY_ID},
    models::{
        indexes::operation_transfer_index::{
            OperationTransferIndex, OperationTransferIndexCriteria,
        },
        Operation, OperationCode,
    },
};
use ic_canister_core::repository::IndexRepository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::cell::RefCell;

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

impl IndexRepository<OperationTransferIndex, Operation> for OperationTransferIndexRepository {
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

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> Vec<Operation> {
        DB.with(|db| {
            let start_key = OperationTransferIndex {
                transfer_id: criteria.transfer_id.to_owned(),
                code: criteria
                    .code
                    .to_owned()
                    .unwrap_or(OperationCode::ApproveTransfer),
                id: [u8::MIN; 16],
            };
            let end_key = OperationTransferIndex {
                transfer_id: criteria.transfer_id.to_owned(),
                code: criteria
                    .code
                    .to_owned()
                    .unwrap_or(OperationCode::ApproveTransfer),
                id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .take_while(|(index, _)| {
                    let operation = index.to_operation();
                    let mut code_matches_criteria = true;
                    let mut status_matches_criteria = true;
                    let mut read_matches_criteria = true;
                    if let Some(code) = &criteria.code {
                        code_matches_criteria = index.code == *code;
                    }
                    if let Some(status) = criteria.status.as_ref() {
                        status_matches_criteria = *status == operation.status;
                    }
                    if let Some(read) = criteria.read {
                        read_matches_criteria = read == operation.read;
                    }

                    code_matches_criteria && status_matches_criteria && read_matches_criteria
                })
                .map(|(index, _)| index.to_operation())
                .collect::<Vec<Operation>>()
        })
    }
}
