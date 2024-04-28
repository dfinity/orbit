use super::ScheduledJob;
use crate::{
    core::ic_cdk::api::{print, time},
    errors::TransferError,
    factories::blockchains::{
        BlockchainApiFactory, BlockchainTransactionSubmitted,
        TRANSACTION_SUBMITTED_DETAILS_TRANSACTION_HASH_KEY,
    },
    models::{
        Account, Request, RequestOperation, RequestStatus, Transfer, TransferId, TransferStatus,
    },
    repositories::{AccountRepository, RequestRepository, TransferRepository},
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
}

#[async_trait]
impl ScheduledJob for Job {
    const INTERVAL_SECS: u64 = 5;

    async fn run() {
        Self::default().execute_created_transfers().await;
    }
}

/// This job is responsible for executing the transfers that have been created and
/// are ready to be submitted to the blockchain.
impl Job {
    pub const MAX_BATCH_SIZE: usize = 20;

    /// Executes all the transfers that have been created but are not yet submitted to the blockchain.
    ///
    /// This function will process a maximum of `MAX_BATCH_SIZE` transfers at once.
    async fn execute_created_transfers(&self) {
        let current_time = time();
        let mut transfers = self.transfer_repository.find_by_status(
            TransferStatus::Created.to_string(),
            None,
            Some(current_time),
        );

        // truncate the list to avoid processing too many transfers at once
        transfers.truncate(Self::MAX_BATCH_SIZE);

        // update the status of the requests to avoid processing them again
        for transfer in transfers.iter_mut() {
            transfer.status = TransferStatus::Processing { started_at: time() };
            transfer.last_modification_timestamp = time();
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
                    transfer.last_modification_timestamp = time();
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

        // update the status of the transfers
        results
            .iter()
            .enumerate()
            .for_each(|(pos, result)| match result {
                Ok((transfer, details)) => {
                    let mut transfer = transfer.clone();

                    let maybe_transaction_hash = details
                        .details
                        .iter()
                        .find(|(key, _)| key == TRANSACTION_SUBMITTED_DETAILS_TRANSACTION_HASH_KEY)
                        .map(|(_, value)| value.to_owned());

                    transfer.status = TransferStatus::Completed {
                        completed_at: time(),
                        hash: maybe_transaction_hash,
                        signature: None,
                    };
                    transfer.last_modification_timestamp = time();
                    self.transfer_repository
                        .insert(transfer.to_key(), transfer.to_owned());

                    if let Some(request) = requests.get(&transfer.id) {
                        let mut request = request.clone();

                        if let RequestOperation::Transfer(transfer_operation) =
                            &mut request.operation
                        {
                            transfer_operation.transfer_id = Some(transfer.id);
                        }

                        request.status = RequestStatus::Completed {
                            completed_at: time(),
                        };
                        request.last_modification_timestamp = time();
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
                    transfer.last_modification_timestamp = time();
                    self.transfer_repository
                        .insert(transfer.to_key(), transfer.to_owned());

                    if let Some(request) = requests.get(&transfer.id) {
                        let mut request = request.clone();
                        request.status = RequestStatus::Failed {
                            reason: Some(e.to_string()),
                        };
                        request.last_modification_timestamp = time();
                        self.request_repository
                            .insert(request.to_key(), request.to_owned());
                    } else {
                        print(format!(
                            "Error: request not found for transfer {}",
                            Uuid::from_bytes(transfer.id).hyphenated()
                        ));
                    }
                }
            });
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
