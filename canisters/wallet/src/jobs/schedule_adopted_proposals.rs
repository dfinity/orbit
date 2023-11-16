use crate::{
    core::ic_cdk::{api::time, spawn},
    models::{ProposalExecutionPlan, ProposalStatus},
    repositories::ProposalRepository,
};
use ic_canister_core::{api::ApiError, repository::Repository};
use std::time::Duration;

#[derive(Debug, Default)]
pub struct Job {
    proposal_repository: ProposalRepository,
}

/// This job is responsible for processing the proposals that have been adopted and
/// are ready to be scheduled.
impl Job {
    pub const INTERVAL_SECS: u64 = 5;
    pub const MAX_BATCH_SIZE: usize = 100;

    pub fn register() {
        let interval = Duration::from_secs(Self::INTERVAL_SECS);
        ic_cdk_timers::set_timer_interval(interval, || {
            spawn(Self::run());
        });
    }

    pub async fn run() {
        Self::default()
            .process_adopted_proposals()
            .await
            .expect("Failed to schedule adopted proposals");
    }

    /// Processes all the proposals that have been adopted but are not yet scheduled to be executed.
    ///
    /// This function will process a maximum of `MAX_BATCH_SIZE` proposals at once.
    async fn process_adopted_proposals(&self) -> Result<(), ApiError> {
        let current_time = time();
        let mut proposals = self.proposal_repository.find_by_status(
            ProposalStatus::Adopted.to_string(),
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

        Ok(())
    }
}
