use crate::{
    core::ic_cdk::api::time,
    models::{ProposalExecutionPlan, ProposalStatus, ProposalStatusType},
    repositories::ProposalRepository,
};
use async_trait::async_trait;
use ic_canister_core::repository::Repository;

use super::ScheduledJob;

#[derive(Debug, Default)]
pub struct Job {
    proposal_repository: ProposalRepository,
}

#[async_trait]
impl ScheduledJob for Job {
    const INTERVAL_SECS: u64 = 5;
    const ALLOW_CONCURRENT_EXECUTION: bool = false;

    async fn run() {
        Self::default().process_adopted_proposals().await;
    }
}

/// This job is responsible for processing the proposals that have been adopted and
/// are ready to be scheduled.
impl Job {
    pub const MAX_BATCH_SIZE: usize = 100;

    /// Processes all the proposals that have been adopted but are not yet scheduled to be executed.
    ///
    /// This function will process a maximum of `MAX_BATCH_SIZE` proposals at once.
    async fn process_adopted_proposals(&self) {
        let current_time = time();
        let mut proposals = self.proposal_repository.find_by_status(
            ProposalStatusType::Adopted.to_string(),
            None,
            Some(current_time),
        );

        // truncate the list to avoid processing too many proposals at once.
        proposals.truncate(Self::MAX_BATCH_SIZE);

        // schedule the proposals to be executed.
        for proposal in proposals.iter_mut() {
            let scheduled_at = match &proposal.execution_plan {
                ProposalExecutionPlan::Immediate => time(),
                ProposalExecutionPlan::Scheduled { execution_time } => *execution_time,
            };

            proposal.status = ProposalStatus::Scheduled { scheduled_at };
            proposal.last_modification_timestamp = time();
            self.proposal_repository
                .insert(proposal.to_key(), proposal.to_owned());
        }
    }
}
