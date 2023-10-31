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
                let prev_accounts_index = prev.to_index_for_accounts();
                if prev_accounts_index != value.to_index_for_accounts() {
                    prev_accounts_index.iter().for_each(|index| {
                        self.account_index.remove(index);
                    });
                    value.to_index_for_accounts().iter().for_each(|index| {
                        self.account_index.insert(index.to_owned());
                    });
                }

                match (prev.to_index_for_wallet(), value.to_index_for_wallet()) {
                    (Some(prev), Some(current)) => {
                        if prev != current {
                            self.wallet_index.remove(&prev);
                            self.wallet_index.insert(current);
                        }
                    }
                    (Some(prev), None) => {
                        self.wallet_index.remove(&prev);
                    }
                    (None, Some(current)) => {
                        self.wallet_index.insert(current);
                    }
                    _ => {}
                }

                match (prev.to_index_for_transfer(), value.to_index_for_transfer()) {
                    (Some(prev), Some(current)) => {
                        if prev != current {
                            self.transfer_index.remove(&prev);
                            self.transfer_index.insert(current);
                        }
                    }
                    (Some(prev), None) => {
                        self.transfer_index.remove(&prev);
                    }
                    (None, Some(current)) => {
                        self.transfer_index.insert(current);
                    }
                    _ => {}
                }

                Some(prev)
            }
            None => {
                value.to_index_for_accounts().iter().for_each(|index| {
                    self.account_index.insert(index.to_owned());
                });
                if let Some(wallet_index) = value.to_index_for_wallet() {
                    self.wallet_index.insert(wallet_index);
                }
                if let Some(transfer_index) = value.to_index_for_transfer() {
                    self.transfer_index.insert(transfer_index);
                }

                None
            }
        })
    }

    fn remove(&self, key: &OperationKey) -> Option<Operation> {
        DB.with(|m| match m.borrow_mut().remove(key) {
            Some(prev) => {
                prev.to_index_for_accounts().iter().for_each(|index| {
                    self.account_index.remove(index);
                });
                if let Some(wallet_index) = prev.to_index_for_wallet() {
                    self.wallet_index.remove(&wallet_index);
                }
                if let Some(transfer_index) = prev.to_index_for_transfer() {
                    self.transfer_index.remove(&transfer_index);
                }

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
                let mut match_status = true;

                if let Some(code) = condition.code.clone() {
                    match_code = operation.code == code;
                }

                if let Some(status) = condition.status.clone() {
                    match_status = operation.status == status;
                }

                match_code && match_status
            })
            .map(|o| o.to_owned())
            .collect::<Vec<_>>()
    }

    pub fn find_by_account_where(
        &self,
        account_id: AccountId,
        condition: OperationFindByAccountWhereClause,
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
                        match_read = operation.decisions.iter().any(|operation| {
                            operation.account_id == account_id && operation.read == read
                        });
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
    pub status: Option<OperationStatus>,
}

#[derive(Debug)]
pub struct OperationFindByAccountWhereClause {
    pub created_dt_from: Option<Timestamp>,
    pub created_dt_to: Option<Timestamp>,
    pub code: Option<OperationCode>,
    pub status: Option<OperationStatus>,
    pub read: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::operation_test_utils;

    #[test]
    fn test_crud() {
        let repository = OperationRepository::default();
        let operation = operation_test_utils::mock_operation();

        assert!(repository.get(&operation.to_key()).is_none());

        repository.insert(operation.to_key(), operation.clone());

        assert!(repository.get(&operation.to_key()).is_some());
        assert!(repository.remove(&operation.to_key()).is_some());
        assert!(repository.get(&operation.to_key()).is_none());
    }
}
