use super::{scheduler::Scheduler, JobType, ScheduledJob};
use crate::{
    core::ic_cdk::{api::print, next_time},
    errors::TransferError,
    factories::blockchains::{
        BlockchainApiFactory, BlockchainTransactionSubmitted,
        TRANSACTION_SUBMITTED_DETAILS_TRANSACTION_HASH_KEY,
    },
    models::{
        Account, Request, RequestOperation, RequestStatus, Transfer, TransferId, TransferStatus,
    },
    repositories::{AccountRepository, RequestRepository, TransferRepository},
    services::RequestService,
};
use async_trait::async_trait;
use futures::future;

use orbit_essentials::repository::Repository;
use std::collections::HashMap;

use uuid::Uuid;

#[derive(Debug, Default)]
pub struct Job {
    transfer_repository: TransferRepository,
    account_repository: AccountRepository,
    request_repository: RequestRepository,
    request_service: RequestService,
}

#[async_trait]
impl ScheduledJob for Job {
    const JOB_TYPE: JobType = JobType::ExecuteCreatedTransfers;
    async fn run() -> bool {
        Self::default().execute_created_transfers().await
    }
}

/// This job is responsible for executing the transfers that have been created and
/// are ready to be submitted to the blockchain.
impl Job {
    pub const MAX_BATCH_SIZE: usize = 20;

    /// Executes all the transfers that have been created but are not yet submitted to the blockchain.
    ///
    /// This function will process a maximum of `MAX_BATCH_SIZE` transfers at once.
    async fn execute_created_transfers(&self) -> bool {
        let current_time = next_time();
        let mut transfers = self.transfer_repository.find_by_status(
            TransferStatus::Created.to_string(),
            None,
            Some(current_time),
        );

        let processing_all_transfers = transfers.len() <= Self::MAX_BATCH_SIZE;

        // truncate the list to avoid processing too many transfers at once
        transfers.truncate(Self::MAX_BATCH_SIZE);

        // update the status of the requests to avoid processing them again
        for transfer in transfers.iter_mut() {
            let transfer_processing_time = next_time();
            transfer.status = TransferStatus::Processing {
                started_at: transfer_processing_time,
            };
            transfer.last_modification_timestamp = transfer_processing_time;
            self.transfer_repository
                .insert(transfer.to_key(), transfer.to_owned());
        }

        // load requests associated with the transfers
        let mut requests: HashMap<TransferId, Request> = HashMap::new();
        for transfer in transfers.iter() {
            match self
                .request_repository
                .get(&Request::key(transfer.request_id))
            {
                Some(request) => {
                    requests.insert(transfer.id, request);
                }
                None => {
                    // if the request is not found, mark the transfer as failed
                    print(format!(
                        "Error: request not found for transfer {}",
                        Uuid::from_bytes(transfer.id).hyphenated()
                    ));

                    let mut transfer = transfer.clone();
                    transfer.status = TransferStatus::Failed {
                        reason: "Request not found".to_string(),
                    };
                    transfer.last_modification_timestamp = next_time();
                    self.transfer_repository
                        .insert(transfer.to_key(), transfer.to_owned());
                }
            }
        }

        // batch the transfers to be executed
        let calls = transfers
            .clone()
            .into_iter()
            .filter(|transfer| requests.contains_key(&transfer.id))
            .map(|transfer| self.execute_transfer(transfer));

        // wait for all the transfers to be executed
        let results = future::join_all(calls).await;
        let transfers = transfers.clone();

        for (pos, result) in results.iter().enumerate() {
            match result {
                Ok((transfer, details)) => {
                    let mut transfer = transfer.clone();
                    let transfer_completed_time = next_time();
                    let maybe_transaction_hash = details
                        .details
                        .iter()
                        .find(|(key, _)| key == TRANSACTION_SUBMITTED_DETAILS_TRANSACTION_HASH_KEY)
                        .map(|(_, value)| value.to_owned());

                    transfer.status = TransferStatus::Completed {
                        completed_at: transfer_completed_time,
                        hash: maybe_transaction_hash,
                        signature: None,
                    };
                    transfer.last_modification_timestamp = transfer_completed_time;
                    self.transfer_repository
                        .insert(transfer.to_key(), transfer.to_owned());

                    if let Some(request) = requests.get(&transfer.id) {
                        let mut request = request.clone();

                        if let RequestOperation::Transfer(transfer_operation) =
                            &mut request.operation
                        {
                            transfer_operation.transfer_id = Some(transfer.id);
                            transfer_operation.fee = Some(transfer.fee);
                        }

                        request.status = RequestStatus::Completed {
                            completed_at: transfer_completed_time,
                        };
                        request.last_modification_timestamp = transfer_completed_time;
                        self.request_repository
                            .insert(request.to_key(), request.to_owned());
                    } else {
                        print(format!(
                            "Error: request not found for transfer {}",
                            Uuid::from_bytes(transfer.id).hyphenated()
                        ));
                    }
                }
                Err(e) => {
                    let mut transfer = transfers[pos].clone();
                    transfer.status = TransferStatus::Failed {
                        reason: e.to_string(),
                    };
                    let transfer_failed_time = next_time();
                    transfer.last_modification_timestamp = transfer_failed_time;
                    self.transfer_repository
                        .insert(transfer.to_key(), transfer.to_owned());

                    if let Some(request) = requests.get(&transfer.id) {
                        let request = request.clone();
                        self.request_service
                            .fail_request(request, e.to_string(), transfer_failed_time)
                            .await;
                    } else {
                        print(format!(
                            "Error: request not found for transfer {}",
                            Uuid::from_bytes(transfer.id).hyphenated()
                        ));
                    }
                }
            }
        }

        processing_all_transfers
    }

    /// Executes a single transfer.
    ///
    /// This function will handle the submission of the transfer to the blockchain.
    async fn execute_transfer(
        &self,
        transfer: Transfer,
    ) -> Result<(Transfer, BlockchainTransactionSubmitted), TransferError> {
        let account = self
            .account_repository
            .get(&Account::key(transfer.from_account))
            .ok_or(TransferError::ValidationError {
                info: format!(
                    "Transfer account not found for id {}",
                    Uuid::from_bytes(transfer.from_account).hyphenated()
                ),
            })?;

        let blockchain_api = BlockchainApiFactory::build(&account.blockchain, &account.standard)
            .map_err(|e| TransferError::ExecutionError {
                reason: format!("Failed to build blockchain api: {}", e),
            })?;

        match blockchain_api.submit_transaction(&account, &transfer).await {
            Ok(details) => Ok((transfer, details)),

            Err(error) => Err(TransferError::ExecutionError {
                reason: error.to_json_string(),
            })?,
        }
    }
}

pub fn schedule_process_transfers(at_ns: u64) {
    Scheduler::schedule::<Job>(at_ns);
}
