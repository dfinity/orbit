use crate::{
    core::ic_cdk::{api::time, spawn},
    errors::WalletError,
    factories::blockchains::BlockchainApiFactory,
    models::{Transfer, TransferStatus, Wallet},
    repositories::{TransferRepository, WalletRepository},
};
use futures::future;
use ic_canister_core::{api::ApiError, repository::Repository};
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct Job {
    transfer_repository: TransferRepository,
    wallet_repository: WalletRepository,
}

/// This job is responsible for processing the transfers that have been approved and
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
            .process_approved_transfers()
            .await
            .expect("Failed to process transfers");
    }

    /// Processes a single transfer.
    ///
    /// This function will submit the transfer to the blockchain and update its status accordingly.
    async fn process_transfer(&self, mut transfer: Transfer) -> Result<Transfer, ApiError> {
        let wallet = self
            .wallet_repository
            .get(&Wallet::key(transfer.from_wallet))
            .ok_or(WalletError::WalletNotFound {
                id: Uuid::from_bytes(transfer.from_wallet)
                    .hyphenated()
                    .to_string(),
            })?;

        let blockchain_api = BlockchainApiFactory::build(&wallet.blockchain, &wallet.standard)?;
        match blockchain_api.submit_transaction(&wallet, &transfer).await {
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

        transfer.last_modification_timestamp = time();
        self.transfer_repository
            .insert(transfer.to_key(), transfer.to_owned());

        Ok(transfer)
    }

    /// Processes all the transfers that have been approved and are ready to be submitted to the blockchain.
    ///
    /// This function will process a maximum of `MAX_BATCH_SIZE` transfers at once.
    async fn process_approved_transfers(&self) -> Result<(), ApiError> {
        let current_time = time();
        let mut transfers = self.transfer_repository.find_by_execution_dt_and_status(
            None,
            Some(current_time),
            TransferStatus::Approved.to_string(),
        );

        // truncate the list to avoid processing too many transfers at once
        transfers.truncate(Self::MAX_BATCH_SIZE);

        // update the status of the transfers to avoid processing them again
        for transfer in transfers.iter_mut() {
            transfer.status = TransferStatus::Processing { started_at: time() };
            transfer.last_modification_timestamp = time();
            self.transfer_repository
                .insert(transfer.to_key(), transfer.to_owned());
        }

        // process the transfers
        let requests = transfers
            .clone()
            .into_iter()
            .map(|transfer| self.process_transfer(transfer));

        // wait for all the transfers to be processed
        let results = future::join_all(requests).await;
        let transfers = transfers.clone();

        // update the status of the transfers
        results.iter().enumerate().for_each(|(pos, result)| {
            if let Err(e) = result {
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
}
