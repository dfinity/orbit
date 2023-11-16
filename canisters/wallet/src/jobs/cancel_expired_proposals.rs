use crate::{
    core::ic_cdk::api::time, errors::ProposalError, models::ProposalStatus,
    repositories::ProposalRepository,
};
use ic_canister_core::{cdk::spawn, repository::Repository};
use std::time::Duration;

#[derive(Debug, Default)]
pub struct Job {
    proposal_repository: ProposalRepository,
}

/// This job is responsible for canceling the proposals that have expired while not adopted/rejected.
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
            .cancel_proposals()
            .await
            .expect("Failed to cancel expirated proposals");
    }

    /// Cancel the proposals that have expired while still pending.
    async fn cancel_proposals(&self) -> Result<(), ProposalError> {
        let current_time = time();
        let mut proposals = self.proposal_repository.find_by_expiration_dt_and_status(
            Some(current_time),
            None,
            ProposalStatus::Created.to_string(),
        );

        for proposal in proposals.iter_mut() {
            proposal.status = ProposalStatus::Cancelled {
                reason: Some("The proposal has expired".to_string()),
            };
            proposal.last_modification_timestamp = time();
            self.proposal_repository
                .insert(proposal.to_key(), proposal.to_owned());
        }

        Ok(())
    }
}
