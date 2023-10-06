use super::indexes::{
    operation_account_index::OperationAccountIndexRepository,
    operation_transfer_index::OperationTransferIndexRepository,
    operation_wallet_index::OperationWalletIndexRepository,
};
use crate::{
    core::{with_memory_manager, Memory, OPERATION_MEMORY_ID},
    models::{
        indexes::{
            operation_account_index::OperationAccountIndexCriteria,
            operation_transfer_index::OperationTransferIndexCriteria,
            operation_wallet_index::OperationWalletIndexCriteria,
        },
        AccountId, Operation, OperationKey, TransferId, WalletId,
    },
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
pub struct OperationRepository {
    account_index: OperationAccountIndexRepository,
    wallet_index: OperationWalletIndexRepository,
    transfer_index: OperationTransferIndexRepository,
}

impl Repository<OperationKey, Operation> for OperationRepository {
    fn get(&self, key: &OperationKey) -> Option<Operation> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: OperationKey, value: Operation) -> Option<Operation> {
        DB.with(|m| {
            OperationAccountIndexRepository::default().insert(value.to_index_for_account());
            OperationWalletIndexRepository::default().insert(value.to_index_for_wallet());
            OperationTransferIndexRepository::default().insert(value.to_index_for_transfer());

            m.borrow_mut().insert(key, value)
        })
    }

    fn remove(&self, key: &OperationKey) -> Option<Operation> {
        DB.with(|m| m.borrow_mut().remove(key))
    }
}

impl OperationRepository {
    pub fn find_by_transfer_id(&self, transfer_id: TransferId) -> Vec<Operation> {
        self.transfer_index
            .find_by_criteria(OperationTransferIndexCriteria {
                transfer_id: transfer_id.to_owned(),
                code: None,
                status: None,
                read: None,
                from_dt: None,
                to_dt: None,
            })
    }

    pub fn find_by_account_id(&self, account_id: AccountId) -> Vec<Operation> {
        self.account_index
            .find_by_criteria(OperationAccountIndexCriteria {
                account_id: account_id.to_owned(),
                code: None,
                status: None,
                read: None,
                from_dt: None,
                to_dt: None,
            })
    }

    pub fn find_by_wallet_id(&self, wallet_id: WalletId) -> Vec<Operation> {
        self.wallet_index
            .find_by_criteria(OperationWalletIndexCriteria {
                wallet_id: wallet_id.to_owned(),
                code: None,
                status: None,
                read: None,
                from_dt: None,
                to_dt: None,
            })
    }
}
