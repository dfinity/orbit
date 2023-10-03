use crate::{
    factories::blockchains::BlockchainApiFactory,
    models::{
        indexes::transfer_execution_time_index::TransferExecutionTimeIndexCriteria, TransferStatus,
        Wallet,
    },
    repositories::{
        indexes::transfer_execution_time_index::TransferExecutionTimeIndexRepository,
        TransferRepository, WalletRepository,
    },
};
use ic_canister_core::{
    api::ApiError,
    cdk::spawn,
    repository::{IndexRepository, Repository},
};
use ic_cdk::api::time;
use std::time::Duration;

#[derive(Debug, Default)]
pub struct ProcessTransfersJob {
    transfer_execution_time_index: TransferExecutionTimeIndexRepository,
    transfer_repository: TransferRepository,
    wallet_repository: WalletRepository,
}

impl ProcessTransfersJob {
    pub const INTERVAL_SECS: u64 = 5;
    pub const MAX_BATCH_SIZE: usize = 20;

    pub fn register() {
        let interval = Duration::from_secs(Self::INTERVAL_SECS);
        ic_cdk_timers::set_timer_interval(interval, || {
            spawn(ProcessTransfersJob::run());
        });
    }

    pub async fn run() {
        ProcessTransfersJob::default()
            .process_approved_transfers()
            .await
            .expect("Failed to process transfers");
    }

    pub async fn process_approved_transfers(&self) -> Result<(), ApiError> {
        let current_time = time();
        let mut transfers = self.transfer_execution_time_index.find_by_criteria(
            TransferExecutionTimeIndexCriteria {
                to_dt: current_time,
                status: Some(TransferStatus::Approved.to_string()),
            },
        );
        // truncate the list to avoid processing too many transfers at once
        transfers.truncate(Self::MAX_BATCH_SIZE);
        // update the status of the transfers to avoid processing them again
        for transfer in transfers.iter_mut() {
            transfer.status = TransferStatus::Processing { started_at: time() };
            transfer.last_modification_timestamp = time();
            self.transfer_repository
                .insert(transfer.as_key(), transfer.to_owned());
        }
        // process the transfers
        for transfer in transfers.iter_mut() {
            let wallet = self
                .wallet_repository
                .get(&Wallet::key(transfer.from_wallet))
                .expect("Wallet not found");

            let blockchain_api = BlockchainApiFactory::build(&wallet.blockchain, &wallet.standard)?;

            match blockchain_api.submit_transaction(&wallet, transfer).await {
                Ok(_) => {
                    transfer.status = TransferStatus::Completed {
                        completed_at: time(),
                        hash: None,
                        signature: None,
                    };
                }
                Err(error) => {
                    transfer.status = TransferStatus::Rejected {
                        reason: format!("Failed to submit transaction, due to: {}", error),
                    };
                }
            };

            transfer.last_modification_timestamp = time();
            self.transfer_repository
                .insert(transfer.as_key(), transfer.to_owned());
        }

        Ok(())
    }
}
