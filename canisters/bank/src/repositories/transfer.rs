use super::indexes::{
    transfer_execution_time_index::TransferExecutionTimeIndexRepository,
    transfer_expiration_time_index::TransferExpirationTimeIndexRepository,
    transfer_wallet_index::TransferWalletIndexRepository,
};
use crate::{
    core::{with_memory_manager, Memory, TRANSFER_MEMORY_ID},
    models::{
        indexes::{
            transfer_execution_time_index::TransferExecutionTimeIndexCriteria,
            transfer_expiration_time_index::TransferExpirationTimeIndexCriteria,
            transfer_wallet_index::TransferWalletIndexCriteria,
        },
        Transfer, TransferKey, WalletId,
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
    wallet_index: TransferWalletIndexRepository,
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
                let prev_wallet_index = prev.to_index_by_wallet();
                if prev_wallet_index != value.to_index_by_wallet() {
                    self.wallet_index.remove(&prev_wallet_index);
                    self.wallet_index.insert(value.to_index_by_wallet());
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
                self.wallet_index.insert(value.to_index_by_wallet());
                self.execution_dt_index
                    .insert(value.to_index_by_execution_dt());

                None
            }
        })
    }

    fn remove(&self, key: &TransferKey) -> Option<Transfer> {
        DB.with(|m| match m.borrow_mut().remove(key) {
            Some(prev) => {
                self.wallet_index.remove(&prev.to_index_by_wallet());
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
                    if transfer.status.to_string() == status {
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
                    if transfer.status.to_string() == status {
                        Some(transfer)
                    } else {
                        None
                    }
                }
                None => None,
            })
            .collect::<Vec<Transfer>>()
    }

    pub fn find_by_wallet(
        &self,
        wallet_id: WalletId,
        created_dt_from: Option<Timestamp>,
        created_dt_to: Option<Timestamp>,
        status: Option<String>,
    ) -> Vec<Transfer> {
        let transfers = self
            .wallet_index
            .find_by_criteria(TransferWalletIndexCriteria {
                wallet_id,
                from_dt: created_dt_from,
                to_dt: created_dt_to,
            });

        transfers
            .iter()
            .filter_map(|id| match (self.get(&Transfer::key(*id)), status.clone()) {
                (Some(transfer), Some(status)) => {
                    if transfer.status.to_string() == status {
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
