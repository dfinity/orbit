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
    repository::{IndexRepository, RefreshIndexMode, Repository},
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

    /// The observer that listens to changes in the transfer repository.
    static CHANGE_OBSERVER: RefCell<Observer<(Transfer, Option<Transfer>)>> = Default::default();

    /// The observer that listens to removals in the transfer repository.
    static REMOVE_OBSERVER: RefCell<Observer<Transfer>> = Default::default();
}

lazy_static! {
    pub static ref TRANSFER_REPOSITORY: TransferRepository = TransferRepository::default();
}

/// A repository that enables managing transfer in stable memory.
#[derive(Debug)]
pub struct TransferRepository {
    account_index: TransferAccountIndexRepository,
    status_index: TransferStatusIndexRepository,
}

impl Default for TransferRepository {
    fn default() -> Self {
        CHANGE_OBSERVER.with(|observer| {
            metrics_observe_insert_transfer(&mut observer.borrow_mut());
            jobs_observe_insert_transfer(&mut observer.borrow_mut());
        });

        REMOVE_OBSERVER.with(|observer| {
            metrics_observe_remove_transfer(&mut observer.borrow_mut());
        });

        Self {
            account_index: TransferAccountIndexRepository::default(),
            status_index: TransferStatusIndexRepository::default(),
        }
    }
}

impl Repository<TransferKey, Transfer> for TransferRepository {
    fn list(&self) -> Vec<Transfer> {
        DB.with(|m| m.borrow().iter().map(|(_, v)| v).collect())
    }

    fn get(&self, key: &TransferKey) -> Option<Transfer> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: TransferKey, value: Transfer) -> Option<Transfer> {
        DB.with(|m| {
            let prev = m.borrow_mut().insert(key, value.clone());

            self.account_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev.clone().map(|prev| prev.to_index_by_account()),
                    current: Some(value.to_index_by_account()),
                });
            self.status_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev.clone().map(|prev| prev.to_index_by_status()),
                    current: Some(value.to_index_by_status()),
                });

            let args = (value, prev);
            CHANGE_OBSERVER.with(|observer| observer.borrow().notify(&args));

            args.1
        })
    }

    fn remove(&self, key: &TransferKey) -> Option<Transfer> {
        DB.with(|m| {
            let prev = m.borrow_mut().remove(key);

            self.account_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev.clone().map(|prev| prev.to_index_by_account()),
                });
            self.status_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev.clone().map(|prev| prev.to_index_by_status()),
                });

            if let Some(prev) = &prev {
                REMOVE_OBSERVER.with(|observer| observer.borrow().notify(prev));
            }

            prev
        })
    }

    fn len(&self) -> usize {
        DB.with(|m| m.borrow().len()) as usize
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
