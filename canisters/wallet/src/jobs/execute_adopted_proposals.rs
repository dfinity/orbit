use crate::{
    core::ic_cdk::{api::time, spawn},
    errors::AccountError,
    factories::blockchains::BlockchainApiFactory,
    models::{Account, Transfer, TransferStatus, Proposal, ProposalStatus},
    repositories::{AccountRepository, TransferRepository, ProposalRepository},
};
use futures::future;
use ic_canister_core::{api::ApiError, repository::Repository};
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct Job {
    proposal_repository: ProposalRepository,
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
            .execute_adopted_proposals()
            .await
            .expect("Failed to execute proposals");
    }

    /// Processes all the proposals that have been adopted but are not yet executed.
    ///
    /// This function will process a maximum of `MAX_BATCH_SIZE` proposals at once.
    async fn process_approved_transfers(&self) -> Result<(), ApiError> {
        let current_time = time();
        // let mut transfers = self.proposal_repository.find_by_execution_dt_and_status(
        //     None,
        //     Some(current_time),
        //     TransferStatus::Approved.to_string(),
        // );

        let mut proposals: Vec<Proposal> = Vec::new();

        // truncate the list to avoid processing too many proposals at once
        proposals.truncate(Self::MAX_BATCH_SIZE);

        // update the status of the proposals to avoid processing them again
        for proposal in proposals.iter_mut() {
            proposal.status = ProposalStatus::Processing { started_at: time() };
            proposal.last_modification_timestamp = time();
            self.proposal_repository
                .insert(proposal.to_key(), proposal.to_owned());
        }

        // process the transfers
        let requests = proposals
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

    /// Processes a single transfer.
    ///
    /// This function will submit the transfer to the blockchain and update its status accordingly.
    // async fn execute_proposals(&self, mut transfer: Transfer) -> Result<Transfer, ApiError> {
    //     let account = self
    //         .account_repository
    //         .get(&Account::key(transfer.from_account))
    //         .ok_or(AccountError::AccountNotFound {
    //             id: Uuid::from_bytes(transfer.from_account)
    //                 .hyphenated()
    //                 .to_string(),
    //         })?;

    //     let blockchain_api = BlockchainApiFactory::build(&account.blockchain, &account.standard)?;
    //     match blockchain_api.submit_transaction(&account, &transfer).await {
    //         Ok(_) => {
    //             transfer.status = TransferStatus::Completed {
    //                 completed_at: time(),
    //                 hash: None,
    //                 signature: None,
    //             };
    //         }
    //         Err(error) => {
    //             transfer.status = TransferStatus::Failed {
    //                 reason: error.to_json_string(),
    //             };
    //         }
    //     };

    //     transfer.last_modification_timestamp = time();
    //     self.transfer_repository
    //         .insert(transfer.to_key(), transfer.to_owned());

    //     Ok(transfer)
    // }
}
