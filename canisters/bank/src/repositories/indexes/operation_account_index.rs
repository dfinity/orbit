use crate::{
    core::{with_memory_manager, Memory, OPERATION_ACCOUNT_INDEX_MEMORY_ID},
    models::{
        indexes::operation_account_index::{OperationAccountIndex, OperationAccountIndexCriteria},
        Operation, OperationCode,
    },
};
use ic_canister_core::repository::IndexRepository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::cell::RefCell;

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

impl IndexRepository<OperationAccountIndex, Operation> for OperationAccountIndexRepository {
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

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> Vec<Operation> {
        DB.with(|db| {
            let start_key = OperationAccountIndex {
                account_id: criteria.account_id.to_owned(),
                code: criteria
                    .code
                    .to_owned()
                    .unwrap_or(OperationCode::ApproveTransfer),
                id: [u8::MIN; 16],
            };
            let end_key = OperationAccountIndex {
                account_id: criteria.account_id.to_owned(),
                code: criteria
                    .code
                    .to_owned()
                    .unwrap_or(OperationCode::ApproveTransfer),
                id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .filter(|(index, _)| {
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
