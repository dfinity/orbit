use super::ScheduledJob;
use crate::{
    core::ic_cdk::api::{print, time},
    errors::TransferError,
    factories::blockchains::BlockchainApiFactory,
    models::{Account, Proposal, ProposalStatus, Transfer, TransferId, TransferStatus},
    repositories::{AccountRepository, ProposalRepository, TransferRepository},
};
use async_trait::async_trait;
use futures::future;
use ic_canister_core::repository::Repository;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct Job {
    transfer_repository: TransferRepository,
    account_repository: AccountRepository,
    proposal_repository: ProposalRepository,
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

        // update the status of the proposals to avoid processing them again
        for transfer in transfers.iter_mut() {
            transfer.status = TransferStatus::Processing { started_at: time() };
            transfer.last_modification_timestamp = time();
            self.transfer_repository
                .insert(transfer.to_key(), transfer.to_owned());
        }

        // load proposals associated with the transfers
        let mut proposals: HashMap<TransferId, Proposal> = HashMap::new();
        for transfer in transfers.iter() {
            match self
                .proposal_repository
                .get(&Proposal::key(transfer.proposal_id))
            {
                Some(proposal) => {
                    proposals.insert(transfer.id, proposal);
                }
                None => {
                    // if the proposal is not found, mark the transfer as failed
                    print(format!(
                        "Error: proposal not found for transfer {}",
                        Uuid::from_bytes(transfer.id).hyphenated()
                    ));

                    let mut transfer = transfer.clone();
                    transfer.status = TransferStatus::Failed {
                        reason: "Proposal not found".to_string(),
                    };
                    transfer.last_modification_timestamp = time();
                    self.transfer_repository
                        .insert(transfer.to_key(), transfer.to_owned());
                }
            }
        }

        // batch the transfers to be executed
        let requests = transfers
            .clone()
            .into_iter()
            .filter(|transfer| proposals.contains_key(&transfer.id))
            .map(|transfer| self.execute_transfer(transfer));

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

                    if let Some(proposal) = proposals.get(&transfer.id) {
                        let mut proposal = proposal.clone();
                        proposal.status = ProposalStatus::Completed {
                            completed_at: time(),
                        };
                        proposal.last_modification_timestamp = time();
                        self.proposal_repository
                            .insert(proposal.to_key(), proposal.to_owned());
                    } else {
                        print(format!(
                            "Error: proposal not found for transfer {}",
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

                    if let Some(proposal) = proposals.get(&transfer.id) {
                        let mut proposal = proposal.clone();
                        proposal.status = ProposalStatus::Failed {
                            reason: Some(e.to_string()),
                        };
                        proposal.last_modification_timestamp = time();
                        self.proposal_repository
                            .insert(proposal.to_key(), proposal.to_owned());
                    } else {
                        print(format!(
                            "Error: proposal not found for transfer {}",
                            Uuid::from_bytes(transfer.id).hyphenated()
                        ));
                    }
                }
            });
    }

    /// Executes a single transfer.
    ///
    /// This function will handle the submission of the transfer to the blockchain.
    async fn execute_transfer(&self, transfer: Transfer) -> Result<Transfer, TransferError> {
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

        if let Err(error) = blockchain_api.submit_transaction(&account, &transfer).await {
            Err(TransferError::ExecutionError {
                reason: error.to_json_string(),
            })?
        }

        Ok(transfer)
    }
}
