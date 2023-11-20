use crate::{
    core::ic_cdk::{api::time, spawn},
    errors::ProposalExecuteError,
    factories::proposals::ProposalFactory,
    models::{Proposal, ProposalStatus},
    repositories::ProposalRepository,
};
use futures::future;
use ic_canister_core::repository::Repository;
use std::time::Duration;

#[derive(Debug, Default)]
pub struct Job {
    proposal_repository: ProposalRepository,
}

/// This job is responsible for processing the proposals that have been adopted and
/// are ready to be executed.
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
            .execute_scheduled_proposals()
            .await
            .expect("Failed to execute proposals");
    }

    /// Processes all the proposals that have been adopted but are not yet executed.
    ///
    /// This function will process a maximum of `MAX_BATCH_SIZE` proposals at once.
    async fn execute_scheduled_proposals(&self) -> Result<(), ProposalExecuteError> {
        let current_time = time();
        let mut proposals = self
            .proposal_repository
            .find_scheduled(None, Some(current_time));

        // truncate the list to avoid processing too many proposals at once
        proposals.truncate(Self::MAX_BATCH_SIZE);

        // update the status of the proposals to avoid processing them again
        for proposal in proposals.iter_mut() {
            proposal.status = ProposalStatus::Processing { started_at: time() };
            proposal.last_modification_timestamp = time();
            self.proposal_repository
                .insert(proposal.to_key(), proposal.to_owned());
        }

        // batch the proposals to be executed
        let requests = proposals
            .clone()
            .into_iter()
            .map(|proposal| self.execute_proposal(proposal));

        // wait for all the proposals to be executed
        let results = future::join_all(requests).await;
        let proposals = proposals.clone();

        // update the status of the proposals
        results
            .iter()
            .enumerate()
            .for_each(|(pos, result)| match result {
                Ok(proposal) => {
                    let mut proposal = proposal.clone();
                    proposal.status = ProposalStatus::Completed {
                        completed_at: time(),
                    };
                    proposal.last_modification_timestamp = time();
                    self.proposal_repository
                        .insert(proposal.to_key(), proposal.to_owned());
                }
                Err(e) => {
                    let mut proposal = proposals[pos].clone();
                    proposal.status = ProposalStatus::Failed {
                        reason: Some(e.to_string()),
                    };
                    proposal.last_modification_timestamp = time();
                    self.proposal_repository
                        .insert(proposal.to_key(), proposal.to_owned());
                }
            });

        Ok(())
    }

    /// Executes a single proposal.
    ///
    /// This function will handle the proposal execution for the given operation type.
    async fn execute_proposal(&self, proposal: Proposal) -> Result<Proposal, ProposalExecuteError> {
        let processor = ProposalFactory::create_processor(&proposal);

        processor.execute().await?;

        drop(processor);

        Ok(proposal)
    }
}
