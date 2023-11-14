use crate::{
    core::ic_cdk::api::time,
    models::{ProposalStatus, ProposalVoteStatus, TransferStatus},
    repositories::{ProposalRepository, TransferRepository},
};
use ic_canister_core::{api::ApiError, cdk::spawn, repository::Repository};
use std::time::Duration;

#[derive(Debug, Default)]
pub struct Job {
    transfer_repository: TransferRepository,
    proposal_repository: ProposalRepository,
}

/// This job is responsible for canceling the transfers that have expired while still pending.
impl Job {
    pub const INTERVAL_SECS: u64 = 60;

    pub fn register() {
        let interval = Duration::from_secs(Self::INTERVAL_SECS);
        ic_cdk_timers::set_timer_interval(interval, || {
            spawn(Self::run());
        });
    }

    pub async fn run() {
        Self::default()
            .cancel_transfers()
            .await
            .expect("Failed to cancel expirated transfers");
    }

    /// Cancel the transfers that have expired while still pending.
    async fn cancel_transfers(&self) -> Result<(), ApiError> {
        let current_time = time();
        let mut transfers = self.transfer_repository.find_by_expiration_dt_and_status(
            None,
            Some(current_time),
            TransferStatus::Pending.to_string(),
        );

        for transfer in transfers.iter_mut() {
            let proposals = self.proposal_repository.find_by_transfer_id(transfer.id);
            for mut proposal in proposals.into_iter() {
                proposal.status = ProposalStatus::Rejected;
                proposal.last_modification_timestamp = time();
                proposal.votes.iter_mut().for_each(|vote| {
                    if let ProposalVoteStatus::Pending = vote.status {
                        vote.status = ProposalVoteStatus::NotRequired;
                        vote.decided_dt = Some(time());
                        vote.status_reason = Some("The transfer has expired".to_string());
                        vote.last_modification_timestamp = time();
                    }
                });

                self.proposal_repository
                    .insert(proposal.to_key(), proposal.to_owned());
            }

            transfer.status = TransferStatus::Cancelled {
                reason: Some("The transfer has expired".to_string()),
            };
            transfer.last_modification_timestamp = time();
            self.transfer_repository
                .insert(transfer.to_key(), transfer.to_owned());
        }

        Ok(())
    }
}
