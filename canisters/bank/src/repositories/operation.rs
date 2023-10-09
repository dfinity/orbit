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
        AccountId, Operation, OperationCode, OperationKey, OperationStatus, TransferId, WalletId,
    },
};
use ic_canister_core::{
    repository::{IndexRepository, Repository},
    types::Timestamp,
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::{cell::RefCell, collections::HashSet};

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
        DB.with(|m| match m.borrow_mut().insert(key, value.clone()) {
            Some(prev) => {
                let prev_account_index = prev.to_index_for_account();
                if prev_account_index != value.to_index_for_account() {
                    self.account_index.remove(&prev_account_index);
                    self.account_index.insert(value.to_index_for_account());
                }

                let prev_wallet_index = prev.to_index_for_wallet();
                if prev_wallet_index != value.to_index_for_wallet() {
                    self.wallet_index.remove(&prev_wallet_index);
                    self.wallet_index.insert(value.to_index_for_wallet());
                }

                let prev_transfer_index = prev.to_index_for_transfer();
                if prev_transfer_index != value.to_index_for_transfer() {
                    self.transfer_index.remove(&prev_transfer_index);
                    self.transfer_index.insert(value.to_index_for_transfer());
                }

                Some(prev)
            }
            None => {
                self.account_index.insert(value.to_index_for_account());
                self.wallet_index.insert(value.to_index_for_wallet());
                self.transfer_index.insert(value.to_index_for_transfer());

                None
            }
        })
    }

    fn remove(&self, key: &OperationKey) -> Option<Operation> {
        DB.with(|m| match m.borrow_mut().remove(key) {
            Some(prev) => {
                self.account_index.remove(&prev.to_index_for_account());
                self.wallet_index.remove(&prev.to_index_for_wallet());
                self.transfer_index.remove(&prev.to_index_for_transfer());

                Some(prev)
            }
            None => None,
        })
    }
}

impl OperationRepository {
    pub fn find_by_transfer_id(&self, transfer_id: TransferId) -> Vec<Operation> {
        self.transfer_index
            .find_by_criteria(OperationTransferIndexCriteria {
                transfer_id: transfer_id.to_owned(),
                from_dt: None,
                to_dt: None,
            })
            .iter()
            .filter_map(|id| self.get(&Operation::key(*id)))
            .collect()
    }

    pub fn find_by_account_id(&self, account_id: AccountId) -> Vec<Operation> {
        self.account_index
            .find_by_criteria(OperationAccountIndexCriteria {
                account_id: account_id.to_owned(),
                from_dt: None,
                to_dt: None,
            })
            .iter()
            .filter_map(|id| self.get(&Operation::key(*id)))
            .collect()
    }

    pub fn find_by_wallet_and_account_id(
        &self,
        wallet_id: WalletId,
        account_id: AccountId,
        created_from_dt: Option<Timestamp>,
        created_to_dt: Option<Timestamp>,
    ) -> Vec<Operation> {
        let filtered_by_wallets =
            self.wallet_index
                .find_by_criteria(OperationWalletIndexCriteria {
                    wallet_id: wallet_id.to_owned(),
                    from_dt: created_from_dt.to_owned(),
                    to_dt: created_to_dt.to_owned(),
                });
        let filtered_by_accounts =
            self.account_index
                .find_by_criteria(OperationAccountIndexCriteria {
                    account_id: account_id.to_owned(),
                    from_dt: created_from_dt,
                    to_dt: created_to_dt,
                });

        let results = filtered_by_wallets
            .intersection(&filtered_by_accounts)
            .copied()
            .collect::<HashSet<_>>();

        results
            .iter()
            .filter_map(|id| self.get(&Operation::key(*id)))
            .collect()
    }

    pub fn find_by_wallet_where(
        &self,
        key: (AccountId, WalletId),
        condition: OperationWhereClause,
    ) -> Vec<Operation> {
        let (account_id, wallet_id) = key;
        let operations = self.find_by_wallet_and_account_id(
            wallet_id,
            account_id,
            condition.created_dt_from,
            condition.created_dt_to,
        );

        operations
            .iter()
            .filter(|operation| {
                let mut match_code = true;
                let mut match_read = true;
                let mut match_status = true;

                if let Some(code) = condition.code.clone() {
                    match_code = operation.code == code;
                }

                if let Some(read) = condition.read {
                    match_read = operation.read == read;
                }

                if let Some(status) = condition.status.clone() {
                    match_status = operation.status == status;
                }

                match_code && match_read && match_status
            })
            .map(|o| o.to_owned())
            .collect::<Vec<_>>()
    }

    pub fn find_by_account_where(
        &self,
        account_id: AccountId,
        condition: OperationWhereClause,
    ) -> Vec<Operation> {
        self.account_index
            .find_by_criteria(OperationAccountIndexCriteria {
                account_id: account_id.to_owned(),
                from_dt: condition.created_dt_from,
                to_dt: condition.created_dt_to,
            })
            .iter()
            .filter_map(|id| match self.get(&Operation::key(*id)) {
                Some(operation) => {
                    let mut match_code = true;
                    let mut match_read = true;
                    let mut match_status = true;

                    if let Some(code) = condition.code.clone() {
                        match_code = operation.code == code;
                    }

                    if let Some(read) = condition.read {
                        match_read = operation.read == read;
                    }

                    if let Some(status) = condition.status.clone() {
                        match_status = operation.status == status;
                    }

                    match match_code && match_read && match_status {
                        true => Some(operation),
                        false => None,
                    }
                }
                None => None,
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct OperationWhereClause {
    pub created_dt_from: Option<Timestamp>,
    pub created_dt_to: Option<Timestamp>,
    pub code: Option<OperationCode>,
    pub read: Option<bool>,
    pub status: Option<OperationStatus>,
}
