use super::evaluation::Evaluate;
use crate::{
    core::ic_cdk::api::print,
    errors::EvaluateError,
    models::{
        criteria::{Criteria, EvaluateCriteria},
        specifier::{Match, ProposalSpecifier, UserSpecifier},
        EvaluationStatus, Proposal, ProposalStatus, UserId,
    },
    repositories::policy::PROPOSAL_POLICY_REPOSITORY,
};
use anyhow::Context;
use async_trait::async_trait;
use futures::{stream, StreamExt};
use ic_canister_core::repository::Repository;
use std::sync::Arc;

pub struct ProposalEvaluator {
    pub proposal_matcher: Arc<dyn Match<(Proposal, ProposalSpecifier)>>,
    pub criteria_evaluator: Arc<dyn EvaluateCriteria>,
    pub proposal: Proposal,
}

impl ProposalEvaluator {
    pub fn new(
        proposal_matcher: Arc<dyn Match<(Proposal, ProposalSpecifier)>>,
        criteria_evaluator: Arc<dyn EvaluateCriteria>,
        proposal: Proposal,
    ) -> Self {
        Self {
            proposal_matcher,
            criteria_evaluator,
            proposal,
        }
    }
}

#[async_trait]
impl Evaluate<EvaluationStatus> for ProposalEvaluator {
    async fn evaluate(&self) -> Result<EvaluationStatus, EvaluateError> {
        let mut matching_policies = Vec::new();
        for policy in PROPOSAL_POLICY_REPOSITORY.list() {
            if self
                .proposal_matcher
                .is_match((self.proposal.to_owned(), policy.specifier.to_owned()))
                .await
                .context("failed to match proposal")?
            {
                matching_policies.push(policy.to_owned());
            }
        }

        if matching_policies.is_empty() {
            print(format!(
                "Proposal {:?} does not match any policy",
                self.proposal.operation
            ));
            // Since proposals handle security critical operations, we want to reject them by default if
            // they don't match any policy. Users need to explicitly add the necessary policies to evaluate them.
            return Ok(EvaluationStatus::Rejected);
        }

        let proposal = Arc::new(self.proposal.to_owned());

        for policy in matching_policies {
            print(format!(
                "Proposal {:?} is going to be evaluated by policy {:?}",
                self.proposal.operation, policy.specifier
            ));
            // Evaluate the criteria
            let evaluation_status = self
                .criteria_evaluator
                .evaluate((proposal.to_owned(), Arc::new(policy.criteria)))
                .await
                .context("failed to evaluate criteria")?;

            if let EvaluationStatus::Adopted | EvaluationStatus::Rejected = evaluation_status {
                return Ok(evaluation_status);
            }
        }

        // Since there are matching policies, but none of them adopted or rejected the proposal, we keep it in the
        // pending status until one of the policies evaluates it as adopted or rejected.
        Ok(EvaluationStatus::Pending)
    }
}

/// Evaluates if the user has the right to vote on the proposal.
///
/// The user has the right to vote if:
///
/// - The proposal is not adopted or rejected
/// - There are matching policies for the proposal and the user is a part of the group that is allowed to vote
/// - The user has not already voted on the proposal
pub struct ProposalVoteRightsEvaluator<'p> {
    pub vote_rights_evaluator: Arc<VoteRightsEvaluate>,
    pub voter_id: UserId,
    pub proposal: &'p Proposal,
}

pub type VoteRightsEvaluate =
    dyn EvaluateCriteria<bool, (Arc<Proposal>, Arc<UserId>, Arc<Criteria>), EvaluateError>;

impl<'p> ProposalVoteRightsEvaluator<'p> {
    pub fn new(
        vote_rights_evaluator: Arc<VoteRightsEvaluate>,
        voter_id: UserId,
        proposal: &'p Proposal,
    ) -> Self {
        Self {
            vote_rights_evaluator,
            voter_id,
            proposal,
        }
    }
}

#[async_trait]
impl Evaluate<bool> for ProposalVoteRightsEvaluator<'_> {
    async fn evaluate(&self) -> Result<bool, EvaluateError> {
        if self.proposal.voters().contains(&self.voter_id)
            || self.proposal.status != ProposalStatus::Created
        {
            return Ok(false);
        }

        for policy in PROPOSAL_POLICY_REPOSITORY.list() {
            if self
                .vote_rights_evaluator
                .evaluate((
                    Arc::new(self.proposal.to_owned()),
                    Arc::new(self.voter_id),
                    Arc::new(policy.criteria.to_owned()),
                ))
                .await?
            {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

pub struct ProposalVoteRightsCriteriaEvaluator {
    pub voter_matcher: Arc<dyn Match<(Proposal, UserId, UserSpecifier)>>,
}

#[async_trait]
impl EvaluateCriteria<bool, (Arc<Proposal>, Arc<UserId>, Arc<Criteria>), EvaluateError>
    for ProposalVoteRightsCriteriaEvaluator
{
    async fn evaluate(
        &self,
        (proposal, voter_id, criteria): (Arc<Proposal>, Arc<UserId>, Arc<Criteria>),
    ) -> Result<bool, EvaluateError> {
        match criteria.as_ref() {
            Criteria::ApprovalThreshold(voter_specifier, _)
            | Criteria::MinimumVotes(voter_specifier, _) => {
                let can_vote = self
                    .voter_matcher
                    .is_match((
                        proposal.as_ref().to_owned(),
                        voter_id.as_ref().to_owned(),
                        voter_specifier.to_owned(),
                    ))
                    .await
                    .context("failed to match proposal voters")?;

                Ok(can_vote)
            }
            Criteria::And(criterias) | Criteria::Or(criterias) => {
                let proposal = &proposal;
                let voter_id = &voter_id;
                let vote_evaluations = stream::iter(criterias)
                    .filter_map(|criteria| async move {
                        match self
                            .evaluate((
                                proposal.to_owned(),
                                voter_id.to_owned(),
                                Arc::new(criteria.to_owned()),
                            ))
                            .await
                        {
                            Ok(can_vote) => Some(can_vote),
                            Err(e) => {
                                print(format!(
                                    "Failed evaluation of criteria vote rights: {:?}",
                                    e
                                ));

                                None
                            }
                        }
                    })
                    .collect::<Vec<bool>>()
                    .await;

                Ok(vote_evaluations.iter().any(|v| *v))
            }
            Criteria::Not(criteria) => {
                let can_vote = self
                    .evaluate((
                        proposal.to_owned(),
                        voter_id.to_owned(),
                        Arc::new(criteria.as_ref().to_owned()),
                    ))
                    .await?;

                Ok(can_vote)
            }
            _ => Ok(false),
        }
    }
}
