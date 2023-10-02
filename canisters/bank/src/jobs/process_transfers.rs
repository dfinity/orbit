use crate::{
    blockchains::BlockchainApiFactory,
    models::{Transfer, TransferStatus, Wallet},
    repositories::{TransferQueueRepository, TransferRepository, WalletRepository},
};
use ic_canister_core::{api::ApiError, cdk::spawn, repository::Repository};
use ic_cdk::api::time;
use std::time::Duration;

#[derive(Debug, Default)]
pub struct ProcessTransfersJob {
    transfer_queue: TransferQueueRepository,
    transfer_repository: TransferRepository,
    wallet_repository: WalletRepository,
}

impl ProcessTransfersJob {
    pub const INTERVAL_SECS: u64 = 5;

    pub fn register() {
        let interval = Duration::from_secs(Self::INTERVAL_SECS);
        ic_cdk_timers::set_timer_interval(interval, || {
            spawn(ProcessTransfersJob::run());
        });
    }

    pub async fn run() {
        ProcessTransfersJob::default()
            .process_transfers()
            .await
            .expect("Failed to process transfers");
    }

    pub async fn process_transfers(&self) -> Result<(), ApiError> {
        let current_time = time();
        let queue_items = self
            .transfer_queue
            .find_all_until_execution_dt(&current_time);

        for queue_item in queue_items {
            match queue_item.transfer_status.as_str() {
                "pending" => {
                    let transfer = self
                        .transfer_repository
                        .get(&Transfer::key(queue_item.transfer_id));

                    match transfer {
                        Some(mut transfer) => {
                            let wallet = self
                                .wallet_repository
                                .get(&Wallet::key(transfer.from_wallet))
                                .expect("Wallet not found");

                            let blockchain_api =
                                BlockchainApiFactory::build(&wallet.blockchain, &wallet.standard)?;

                            let result =
                                blockchain_api.submit_transaction(&wallet, &transfer).await;

                            if result.is_ok() {
                                transfer.status = TransferStatus::Completed {
                                    completed_at: time(),
                                    hash: None,
                                    signature: None,
                                };
                                transfer.last_modification_timestamp = time();
                            } else {
                                transfer.status = TransferStatus::Rejected {
                                    reason: "Failed to submit transaction".to_string(),
                                };
                                transfer.last_modification_timestamp = time();
                            }

                            self.transfer_repository
                                .insert(transfer.as_key(), transfer.to_owned());
                            self.transfer_queue.remove(&queue_item.as_key());
                        }
                        None => {
                            self.transfer_queue.remove(&queue_item.as_key());
                        }
                    }
                }
                "submitted" => {
                    todo!()
                }
                _ => {}
            }
        }

        Ok(())
    }
}
