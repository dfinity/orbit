use crate::{
    core::ic_cdk::api::time,
    models::{ProposalStatus, ProposalStatusCode},
    repositories::ProposalRepository,
};
use async_trait::async_trait;
use orbit_essentials::repository::Repository;

use super::ScheduledJob;

#[derive(Debug, Default)]
pub struct Job {
    proposal_repository: ProposalRepository,
}

#[async_trait]
impl ScheduledJob for Job {
    const INTERVAL_SECS: u64 = 60;
    const ALLOW_CONCURRENT_EXECUTION: bool = false;

    async fn run() {
        Self::default().cancel_proposals().await;
    }
}

/// This job is responsible for canceling the proposals that have expired while not adopted/rejected.
impl Job {
    /// Cancel the proposals that have expired while still pending.
    async fn cancel_proposals(&self) {
        let current_time = time();
        let mut proposals = self.proposal_repository.find_by_expiration_dt_and_status(
            None,
            Some(current_time),
            ProposalStatusCode::Created.to_string(),
        );

        for proposal in proposals.iter_mut() {
            proposal.status = ProposalStatus::Cancelled {
                reason: Some("The proposal has expired".to_string()),
            };
            proposal.last_modification_timestamp = time();
            self.proposal_repository
                .insert(proposal.to_key(), proposal.to_owned());
        }
    }
}
