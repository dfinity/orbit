use crate::{
    core::ic_cdk::{api::time, spawn},
    errors::AccountError,
    factories::blockchains::BlockchainApiFactory,
    models::{Account, Transfer, TransferStatus},
    repositories::{AccountRepository, TransferRepository},
};
use futures::future;
use ic_canister_core::{api::ApiError, repository::Repository};
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct Job {
    transfer_repository: TransferRepository,
    account_repository: AccountRepository,
}

/// This job is responsible for executing the transfers that have been created and
/// are ready to be submitted to the blockchain.
impl Job {
    pub const INTERVAL_SECS: u64 = 5;
    pub const MAX_BATCH_SIZE: usize = 20;

    pub fn register() {
        let interval = Duration::from_secs(Self::INTERVAL_SECS);
        ic_cdk_timers::set_timer_interval(interval, || {
            spawn(Self::run());
        });
    }

    pub async fn run() {
        Self::default()
            .execute_created_transfers()
            .await
            .expect("Failed to execute proposals");
    }

    /// Executes all the transfers that have been created but are not yet submitted to the blockchain.
    ///
    /// This function will process a maximum of `MAX_BATCH_SIZE` transfers at once.
    async fn execute_created_transfers(&self) -> Result<(), ApiError> {
        let current_time = time();
        let mut transfers = self.transfer_repository.find_by_status(
            TransferStatus::Created.to_string(),
            None,
            Some(current_time),
        );

        // truncate the list to avoid processing too many transfers at once
        transfers.truncate(Self::MAX_BATCH_SIZE);

        // update the status of the proposals to avoid processing them again
        for transfer in transfers.iter_mut() {
            transfer.status = TransferStatus::Processing { started_at: time() };
            transfer.last_modification_timestamp = time();
            self.transfer_repository
                .insert(transfer.to_key(), transfer.to_owned());
        }

        // batch the transfers to be executed
        let requests = transfers
            .clone()
            .into_iter()
            .map(|transfer| self.submit_transfer(transfer));

        // wait for all the transfers to be executed
        let results = future::join_all(requests).await;
        let transfers = transfers.clone();

        // update the status of the transfers
        results
            .iter()
            .enumerate()
            .for_each(|(pos, result)| match result {
                Ok(transfer) => {
                    let mut transfer = transfer.clone();
                    transfer.status = TransferStatus::Completed {
                        completed_at: time(),
                        hash: None,
                        signature: None,
                    };
                    transfer.last_modification_timestamp = time();
                    self.transfer_repository
                        .insert(transfer.to_key(), transfer.to_owned());
                }
                Err(e) => {
                    let mut transfer = transfers[pos].clone();
                    transfer.status = TransferStatus::Failed {
                        reason: e.to_json_string(),
                    };
                    transfer.last_modification_timestamp = time();
                    self.transfer_repository
                        .insert(transfer.to_key(), transfer.to_owned());
                }
            });

        Ok(())
    }

    /// Executes a single transfer.
    ///
    /// This function will handle the submission of the transfer to the blockchain.
    async fn submit_transfer(&self, mut transfer: Transfer) -> Result<Transfer, ApiError> {
        let account = self
            .account_repository
            .get(&Account::key(transfer.from_account))
            .ok_or(AccountError::AccountNotFound {
                id: Uuid::from_bytes(transfer.from_account)
                    .hyphenated()
                    .to_string(),
            })?;

        let blockchain_api = BlockchainApiFactory::build(&account.blockchain, &account.standard)?;
        match blockchain_api.submit_transaction(&account, &transfer).await {
            Ok(_) => {
                transfer.status = TransferStatus::Completed {
                    completed_at: time(),
                    hash: None,
                    signature: None,
                };
            }
            Err(error) => {
                transfer.status = TransferStatus::Failed {
                    reason: error.to_json_string(),
                };
            }
        };

        Ok(transfer)
    }
}
