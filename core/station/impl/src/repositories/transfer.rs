use super::indexes::{
    transfer_account_index::TransferAccountIndexRepository,
    transfer_status_index::TransferStatusIndexRepository,
};
use crate::{
    core::{
        metrics::{metrics_observe_insert_transfer, metrics_observe_remove_transfer},
        observer::Observer,
        with_memory_manager, Memory, TRANSFER_MEMORY_ID,
    },
    jobs::jobs_observe_insert_transfer,
    models::{
        indexes::{
            transfer_account_index::TransferAccountIndexCriteria,
            transfer_status_index::TransferStatusIndexCriteria,
        },
        AccountId, Transfer, TransferKey,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use orbit_essentials::{
    repository::{IndexRepository, IndexedRepository, Repository, StableDb},
    types::Timestamp,
};
use station_api::TransferStatusTypeDTO;
use std::cell::RefCell;

thread_local! {
    /// The memory reference to the Transfer repository.
    static DB: RefCell<StableBTreeMap<TransferKey, Transfer, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
        RefCell::new(
            StableBTreeMap::init(memory_manager.get(TRANSFER_MEMORY_ID))
        )
    });
}

lazy_static! {
    pub static ref TRANSFER_REPOSITORY: TransferRepository = TransferRepository::default();
}

/// A repository that enables managing transfer in stable memory.
#[derive(Debug)]
pub struct TransferRepository {
    account_index: TransferAccountIndexRepository,
    status_index: TransferStatusIndexRepository,
    change_observer: Observer<(Transfer, Option<Transfer>)>,
    remove_observer: Observer<Transfer>,
}

impl Default for TransferRepository {
    fn default() -> Self {
        let mut change_observer = Observer::default();
        metrics_observe_insert_transfer(&mut change_observer);
        jobs_observe_insert_transfer(&mut change_observer);

        let mut remove_observer = Observer::default();
        metrics_observe_remove_transfer(&mut remove_observer);

        Self {
            account_index: TransferAccountIndexRepository::default(),
            status_index: TransferStatusIndexRepository::default(),
            change_observer,
            remove_observer,
        }
    }
}

impl StableDb<TransferKey, Transfer, VirtualMemory<Memory>> for TransferRepository {
    fn with_db<F, R>(f: F) -> R
    where
        F: FnOnce(&mut StableBTreeMap<TransferKey, Transfer, VirtualMemory<Memory>>) -> R,
    {
        DB.with(|m| f(&mut m.borrow_mut()))
    }
}

impl IndexedRepository<TransferKey, Transfer, VirtualMemory<Memory>> for TransferRepository {
    fn remove_entry_indexes(&self, entry: &Transfer) {
        self.account_index.remove(&entry.to_index_by_account());
        self.status_index.remove(&entry.to_index_by_status());
    }

    fn add_entry_indexes(&self, entry: &Transfer) {
        self.account_index.insert(entry.to_index_by_account());
        self.status_index.insert(entry.to_index_by_status());
    }
}

impl Repository<TransferKey, Transfer, VirtualMemory<Memory>> for TransferRepository {
    fn insert(&self, key: TransferKey, value: Transfer) -> Option<Transfer> {
        DB.with(|m| {
            let prev = m.borrow_mut().insert(key, value.clone());

            self.save_entry_indexes(&value, prev.as_ref());

            let args = (value, prev);
            self.change_observer.notify(&args);

            args.1
        })
    }

    fn remove(&self, key: &TransferKey) -> Option<Transfer> {
        DB.with(|m| {
            let prev = m.borrow_mut().remove(key);

            if let Some(prev) = &prev {
                self.remove_entry_indexes(prev);
            }

            if let Some(prev) = &prev {
                self.remove_observer.notify(prev);
            }

            prev
        })
    }
}

impl TransferRepository {
    pub fn find_by_account(
        &self,
        account_id: AccountId,
        created_dt_from: Option<Timestamp>,
        created_dt_to: Option<Timestamp>,
        status: Option<TransferStatusTypeDTO>,
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
                    if status == transfer.status.clone().into() {
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

    pub fn find_by_status(
        &self,
        status: String,
        from_last_update_dt: Option<Timestamp>,
        to_last_update_dt: Option<Timestamp>,
    ) -> Vec<Transfer> {
        let transfers = self
            .status_index
            .find_by_criteria(TransferStatusIndexCriteria {
                status: status.to_owned(),
                from_dt: from_last_update_dt,
                to_dt: to_last_update_dt,
            });

        transfers
            .iter()
            .filter_map(|id| self.get(&Transfer::key(*id)))
            .collect::<Vec<Transfer>>()
    }

    #[cfg(test)]
    pub fn with_empty_observers() -> Self {
        Self {
            change_observer: Observer::default(),
            remove_observer: Observer::default(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::transfer_test_utils;

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
