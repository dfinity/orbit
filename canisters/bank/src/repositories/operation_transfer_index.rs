use super::OperationRepository;
use crate::{
    core::{with_memory_manager, Memory, OPERATION_TRANSFER_INDEX_MEMORY_ID},
    models::{Operation, OperationCode, OperationStatus, OperationTransferIndex, TransferId},
};
use ic_canister_core::repository::Repository;
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

impl Repository<OperationTransferIndex, ()> for OperationTransferIndexRepository {
    fn get(&self, key: &OperationTransferIndex) -> Option<()> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: OperationTransferIndex, value: ()) -> Option<()> {
        DB.with(|m| m.borrow_mut().insert(key, value))
    }

    fn remove(&self, key: &OperationTransferIndex) -> Option<()> {
        DB.with(|m| m.borrow_mut().remove(key))
    }
}

impl OperationTransferIndexRepository {
    pub fn find_all_within_criteria(
        &self,
        transfer_id: TransferId,
        code: Option<OperationCode>,
        status: Option<OperationStatus>,
        read: Option<bool>,
    ) -> Vec<Operation> {
        DB.with(|db| {
            let start_key = OperationTransferIndex {
                transfer_id: transfer_id.to_owned(),
                code: code.to_owned().unwrap_or(OperationCode::ApproveTransfer),
                id: [u8::MIN; 16],
            };
            let end_key = OperationTransferIndex {
                transfer_id: transfer_id.to_owned(),
                code: code.to_owned().unwrap_or(OperationCode::ApproveTransfer),
                id: [u8::MAX; 16],
            };

            let operation_repository = OperationRepository::default();
            db.borrow()
                .range(start_key..=end_key)
                .take_while(|(index, _)| {
                    let operation = operation_repository
                        .get(&Operation::key(index.id))
                        .expect("Operation not found");

                    let mut code_matches_criteria = true;
                    let mut status_matches_criteria = true;
                    let mut read_matches_criteria = true;
                    if let Some(code) = &code {
                        code_matches_criteria = index.code == *code;
                    }
                    if let Some(status) = status.as_ref() {
                        status_matches_criteria = *status == operation.status;
                    }
                    if let Some(read) = read {
                        read_matches_criteria = read == operation.read;
                    }

                    code_matches_criteria && status_matches_criteria && read_matches_criteria
                })
                .map(|(index, _)| {
                    operation_repository
                        .get(&Operation::key(index.id))
                        .expect("Operation not found")
                })
                .collect::<Vec<Operation>>()
        })
    }
}
