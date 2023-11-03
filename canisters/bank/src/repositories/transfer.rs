use super::indexes::{
    transfer_account_index::TransferAccountIndexRepository,
    transfer_execution_time_index::TransferExecutionTimeIndexRepository,
    transfer_expiration_time_index::TransferExpirationTimeIndexRepository,
};
use crate::{
    core::{with_memory_manager, Memory, TRANSFER_MEMORY_ID},
    models::{
        indexes::{
            transfer_account_index::TransferAccountIndexCriteria,
            transfer_execution_time_index::TransferExecutionTimeIndexCriteria,
            transfer_expiration_time_index::TransferExpirationTimeIndexCriteria,
        },
        AccountId, Transfer, TransferKey,
    },
};
use ic_canister_core::{
    repository::{IndexRepository, Repository},
    types::Timestamp,
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::cell::RefCell;

thread_local! {
  /// The memory reference to the Transfer repository.
  static DB: RefCell<StableBTreeMap<TransferKey, Transfer, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(TRANSFER_MEMORY_ID))
    )
  })
}

/// A repository that enables managing transfer in stable memory.
#[derive(Default, Debug)]
pub struct TransferRepository {
    account_index: TransferAccountIndexRepository,
    execution_dt_index: TransferExecutionTimeIndexRepository,
    expiration_dt_index: TransferExpirationTimeIndexRepository,
}

impl Repository<TransferKey, Transfer> for TransferRepository {
    fn get(&self, key: &TransferKey) -> Option<Transfer> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: TransferKey, value: Transfer) -> Option<Transfer> {
        DB.with(|m| match m.borrow_mut().insert(key, value.clone()) {
            Some(prev) => {
                let prev_account_index = prev.to_index_by_account();
                if prev_account_index != value.to_index_by_account() {
                    self.account_index.remove(&prev_account_index);
                    self.account_index.insert(value.to_index_by_account());
                }
                let prev_execution_dt_index = prev.to_index_by_execution_dt();
                if prev_execution_dt_index != value.to_index_by_execution_dt() {
                    self.execution_dt_index.remove(&prev_execution_dt_index);
                    self.execution_dt_index
                        .insert(value.to_index_by_execution_dt());
                }
                let prev_expiration_dt_index = prev.to_index_by_expiration_dt();
                if prev_expiration_dt_index != value.to_index_by_expiration_dt() {
                    self.expiration_dt_index.remove(&prev_expiration_dt_index);
                    self.expiration_dt_index
                        .insert(value.to_index_by_expiration_dt());
                }

                Some(prev)
            }
            None => {
                self.account_index.insert(value.to_index_by_account());
                self.execution_dt_index
                    .insert(value.to_index_by_execution_dt());
                self.expiration_dt_index
                    .insert(value.to_index_by_expiration_dt());

                None
            }
        })
    }

    fn remove(&self, key: &TransferKey) -> Option<Transfer> {
        DB.with(|m| match m.borrow_mut().remove(key) {
            Some(prev) => {
                self.account_index.remove(&prev.to_index_by_account());
                self.execution_dt_index
                    .remove(&prev.to_index_by_execution_dt());
                self.expiration_dt_index
                    .remove(&prev.to_index_by_expiration_dt());

                Some(prev)
            }
            None => None,
        })
    }
}

impl TransferRepository {
    pub fn find_by_execution_dt_and_status(
        &self,
        execution_dt_from: Option<Timestamp>,
        execution_dt_to: Option<Timestamp>,
        status: String,
    ) -> Vec<Transfer> {
        let transfers =
            self.execution_dt_index
                .find_by_criteria(TransferExecutionTimeIndexCriteria {
                    from_dt: execution_dt_from,
                    to_dt: execution_dt_to,
                });

        transfers
            .iter()
            .filter_map(|id| match self.get(&Transfer::key(*id)) {
                Some(transfer) => {
                    if transfer
                        .status
                        .to_string()
                        .eq_ignore_ascii_case(status.as_str())
                    {
                        Some(transfer)
                    } else {
                        None
                    }
                }
                None => None,
            })
            .collect::<Vec<Transfer>>()
    }

    pub fn find_by_expiration_dt_and_status(
        &self,
        expiration_dt_from: Option<Timestamp>,
        expiration_dt_to: Option<Timestamp>,
        status: String,
    ) -> Vec<Transfer> {
        let transfers =
            self.expiration_dt_index
                .find_by_criteria(TransferExpirationTimeIndexCriteria {
                    from_dt: expiration_dt_from,
                    to_dt: expiration_dt_to,
                });

        transfers
            .iter()
            .filter_map(|id| match self.get(&Transfer::key(*id)) {
                Some(transfer) => {
                    if transfer
                        .status
                        .to_string()
                        .eq_ignore_ascii_case(status.as_str())
                    {
                        Some(transfer)
                    } else {
                        None
                    }
                }
                None => None,
            })
            .collect::<Vec<Transfer>>()
    }

    pub fn find_by_account(
        &self,
        account_id: AccountId,
        created_dt_from: Option<Timestamp>,
        created_dt_to: Option<Timestamp>,
        status: Option<String>,
    ) -> Vec<Transfer> {
        let transfers = self
            .account_index
            .find_by_criteria(TransferAccountIndexCriteria {
                account_id,
                from_dt: created_dt_from,
                to_dt: created_dt_to,
            });

        transfers
            .iter()
            .filter_map(|id| match (self.get(&Transfer::key(*id)), status.clone()) {
                (Some(transfer), Some(status)) => {
                    if transfer
                        .status
                        .to_string()
                        .eq_ignore_ascii_case(status.as_str())
                    {
                        Some(transfer)
                    } else {
                        None
                    }
                }
                (Some(transfer), None) => Some(transfer),
                _ => None,
            })
            .collect::<Vec<Transfer>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{transfer_test_utils, TransferExecutionPlan};

    #[test]
    fn perform_crud() {
        let repository = TransferRepository::default();
        let transfer = transfer_test_utils::mock_transfer();

        assert!(repository.get(&transfer.to_key()).is_none());

        repository.insert(transfer.to_key(), transfer.clone());

        assert!(repository.get(&transfer.to_key()).is_some());
        assert!(repository.remove(&transfer.to_key()).is_some());
        assert!(repository.get(&transfer.to_key()).is_none());
    }

    #[test]
    fn find_transfer_with_execution_dt_and_status() {
        let repository = TransferRepository::default();
        let mut transfer = transfer_test_utils::mock_transfer();
        transfer.execution_plan = TransferExecutionPlan::Scheduled { execution_time: 10 };

        repository.insert(transfer.to_key(), transfer.clone());

        let transfers = repository.find_by_execution_dt_and_status(
            Some(10),
            Some(10),
            transfer.status.to_string(),
        );

        assert_eq!(transfers.len(), 1);
        assert_eq!(transfers[0], transfer);
    }

    #[test]
    fn find_transfer_by_expiration_dt_and_status() {
        let repository = TransferRepository::default();
        let mut transfer = transfer_test_utils::mock_transfer();
        transfer.expiration_dt = 10;

        repository.insert(transfer.to_key(), transfer.clone());

        let transfers = repository.find_by_expiration_dt_and_status(
            Some(10),
            Some(10),
            transfer.status.to_string(),
        );

        assert_eq!(transfers.len(), 1);
        assert_eq!(transfers[0], transfer);
    }

    #[test]
    fn no_transfers_of_future_expiration_dt() {
        let repository = TransferRepository::default();
        let mut transfer = transfer_test_utils::mock_transfer();
        transfer.expiration_dt = 10;

        repository.insert(transfer.to_key(), transfer.clone());

        let transfers = repository.find_by_expiration_dt_and_status(
            Some(20),
            None,
            transfer.status.to_string(),
        );

        assert!(transfers.is_empty());
    }

    #[test]
    fn find_by_account() {
        let repository = TransferRepository::default();
        let mut transfer = transfer_test_utils::mock_transfer();
        transfer.from_account = [1; 16];

        repository.insert(transfer.to_key(), transfer.clone());

        let transfers = repository.find_by_account([1; 16], None, None, None);

        assert_eq!(transfers.len(), 1);
        assert_eq!(transfers[0], transfer);
    }

    #[test]
    fn no_transfer_from_unknown_account() {
        let repository = TransferRepository::default();
        let mut transfer = transfer_test_utils::mock_transfer();
        transfer.from_account = [1; 16];

        repository.insert(transfer.to_key(), transfer.clone());

        let transfers = repository.find_by_account([0; 16], None, None, None);

        assert!(transfers.is_empty());
    }
}
