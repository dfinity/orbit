use super::ScheduledJob;
use crate::{
    core::ic_cdk::api::time,
    errors::ProposalExecuteError,
    factories::proposals::{ProposalExecuteStage, ProposalFactory},
    models::{Proposal, ProposalStatus},
    repositories::ProposalRepository,
};
use async_trait::async_trait;
use futures::future;
use ic_canister_core::repository::Repository;

#[derive(Debug, Default)]
pub struct Job {
    proposal_repository: ProposalRepository,
}

#[async_trait]
impl ScheduledJob for Job {
    const INTERVAL_SECS: u64 = 5;
    const ALLOW_CONCURRENT_EXECUTION: bool = true;

    async fn run() {
        Self::default().execute_scheduled_proposals().await;
    }
}

/// This job is responsible for processing the proposals that have been adopted and
/// are ready to be executed.
impl Job {
    pub const MAX_BATCH_SIZE: usize = 20;

    /// Processes all the proposals that have been adopted but are not yet executed.
    ///
    /// This function will process a maximum of `MAX_BATCH_SIZE` proposals at once.
    async fn execute_scheduled_proposals(&self) {
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
    }

    /// Executes a single proposal.
    ///
    /// This function will handle the proposal execution for the given operation type.
    async fn execute_proposal(
        &self,
        mut proposal: Proposal,
    ) -> Result<Proposal, ProposalExecuteError> {
        let processor = ProposalFactory::create_processor(&proposal);

        let execute_state = processor.execute().await?;

        drop(processor);

        proposal.status = match execute_state {
            ProposalExecuteStage::Completed => ProposalStatus::Completed {
                completed_at: time(),
            },
            ProposalExecuteStage::Processing => ProposalStatus::Processing { started_at: time() },
        };

        proposal.last_modification_timestamp = time();

        Ok(proposal)
    }
}
