use super::{
    specifier::{Match, UserSpecifier},
    EvaluateError, EvaluationStatus, Proposal, ProposalVoteStatus, UserId,
};
use crate::{
    core::utils::calculate_minimum_threshold, errors::MatchError, repositories::USER_REPOSITORY,
};
use anyhow::{anyhow, Error};
use async_trait::async_trait;
use candid::{CandidType, Deserialize};
use futures::{stream, StreamExt, TryStreamExt};
use ic_canister_core::{repository::Repository, types::UUID};
use ic_canister_macros::stable_object;
use std::hash::Hash;
use std::sync::Arc;

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Percentage(pub u16);

impl TryFrom<u16> for Percentage {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value > 100 {
            return Err(anyhow!(
                "invalid percentage value, must be between >= 0 and <= 100"
            ));
        }

        Ok(Percentage(value))
    }
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Criteria {
    // Auto
    AutoAdopted,
    // Votes
    ApprovalThreshold(UserSpecifier, Percentage),
    MinimumVotes(UserSpecifier, u16),
    // Logical
    Or(Vec<Criteria>),
    And(Vec<Criteria>),
    Not(Box<Criteria>),
}

impl From<ProposalVoteStatus> for EvaluationStatus {
    fn from(value: ProposalVoteStatus) -> Self {
        match value {
            ProposalVoteStatus::Accepted => EvaluationStatus::Adopted,
            ProposalVoteStatus::Rejected => EvaluationStatus::Rejected,
            // TODO: Abstain
        }
    }
}

#[async_trait]
pub trait EvaluateCriteria<
    Status = EvaluationStatus,
    Context = (Arc<Proposal>, Arc<Criteria>),
    Error = EvaluateError,
>: Sync + Send
{
    async fn evaluate(&self, ctx: Context) -> Result<Status, Error>;
}

#[derive(Clone)]
pub struct CriteriaEvaluator {
    pub user_matcher: Arc<dyn Match<(Proposal, UUID, UserSpecifier)>>,
}

struct ProposalVoteSummary {
    total_possible_votes: usize,
    adopted_votes: usize,
    rejected_votes: usize,
}

impl ProposalVoteSummary {
    /// Evaluates the proposal vote summary and returns the evaluation status based on the
    /// minimum votes required.
    ///
    /// If the proposal does not yet have enough votes to meet the minimum votes required but has
    /// enough uncasted votes that could be casted to meet the minimum votes required, then the evaluation
    /// is kept in the `Pending` state.
    fn evaluate(&self, min_votes: &usize) -> EvaluationStatus {
        let uncasted_votes = self
            .total_possible_votes
            .saturating_sub(self.adopted_votes)
            .saturating_sub(self.rejected_votes);

        if self.adopted_votes >= *min_votes {
            return EvaluationStatus::Adopted;
        }

        if self.adopted_votes.saturating_add(uncasted_votes) < *min_votes {
            return EvaluationStatus::Rejected;
        }

        EvaluationStatus::Pending
    }
}

impl CriteriaEvaluator {
    async fn evaluate_criterias(
        &self,
        proposal: &Arc<Proposal>,
        criterias: &[Criteria],
    ) -> Result<Vec<EvaluationStatus>, EvaluateError> {
        stream::iter(criterias.iter())
            .then(|criteria| async move {
                self.evaluate((proposal.to_owned(), Arc::new(criteria.to_owned())))
                    .await
            })
            .try_collect::<Vec<EvaluationStatus>>()
            .await
    }

    async fn find_matching_users<UserMatchReturn>(
        &self,
        proposal: &Arc<Proposal>,
        users: &[(UserId, UserMatchReturn)],
        user_specifier: &UserSpecifier,
    ) -> Result<Vec<UserMatchReturn>, MatchError>
    where
        UserMatchReturn: Clone,
    {
        stream::iter(users.iter())
            .then(|(user_id, match_return)| {
                let match_return = match_return.clone();
                async move {
                    match {
                        self.user_matcher
                            .is_match((
                                proposal.as_ref().to_owned(),
                                user_id.to_owned(),
                                user_specifier.to_owned(),
                            ))
                            .await
                    } {
                        Ok(true) => Ok(Some(match_return)),
                        Ok(false) => Ok(None),
                        Err(e) => Err(e),
                    }
                }
            })
            .try_filter_map(|result| async move { Ok(result) })
            .try_collect()
            .await
    }

    async fn calculate_votes(
        &self,
        proposal: &Arc<Proposal>,
        user_specifier: &UserSpecifier,
    ) -> Result<ProposalVoteSummary, MatchError> {
        let casted_votes = self
            .find_matching_users::<ProposalVoteStatus>(
                proposal,
                proposal
                    .votes
                    .iter()
                    .map(|vote| (vote.user_id.to_owned(), vote.status.to_owned()))
                    .collect::<Vec<(UserId, ProposalVoteStatus)>>()
                    .as_slice(),
                user_specifier,
            )
            .await?;

        let total_possible_votes = self
            .find_matching_users::<()>(
                proposal,
                USER_REPOSITORY
                    .list()
                    .iter()
                    .map(|user| (user.id.to_owned(), ()))
                    .collect::<Vec<(UserId, ())>>()
                    .as_slice(),
                user_specifier,
            )
            .await?
            .len();

        Ok(ProposalVoteSummary {
            total_possible_votes,
            adopted_votes: casted_votes
                .iter()
                .filter(|&v| matches!(v, ProposalVoteStatus::Accepted))
                .count(),
            rejected_votes: casted_votes
                .iter()
                .filter(|&v| matches!(v, ProposalVoteStatus::Rejected))
                .count(),
        })
    }
}

#[async_trait]
impl EvaluateCriteria for CriteriaEvaluator {
    async fn evaluate(
        &self,
        (proposal, c): (Arc<Proposal>, Arc<Criteria>),
    ) -> Result<EvaluationStatus, EvaluateError> {
        match c.as_ref() {
            Criteria::AutoAdopted => Ok(EvaluationStatus::Adopted),
            Criteria::ApprovalThreshold(user_specifier, percentage) => {
                let votes = self.calculate_votes(&proposal, user_specifier).await?;
                let min_votes =
                    calculate_minimum_threshold(percentage, &votes.total_possible_votes);

                Ok(votes.evaluate(&min_votes))
            }
            Criteria::MinimumVotes(user_specifier, min_votes) => {
                let votes = self.calculate_votes(&proposal, user_specifier).await?;
                let min_votes = *min_votes as usize;

                Ok(votes.evaluate(&min_votes))
            }
            Criteria::And(criterias) => {
                let evaluation_statuses = self.evaluate_criterias(&proposal, criterias).await?;

                if evaluation_statuses
                    .iter()
                    .any(|s| matches!(s, EvaluationStatus::Rejected))
                {
                    return Ok(EvaluationStatus::Rejected);
                }

                if evaluation_statuses
                    .iter()
                    .all(|s| matches!(s, EvaluationStatus::Adopted))
                {
                    return Ok(EvaluationStatus::Adopted);
                }

                Ok(EvaluationStatus::Pending)
            }
            Criteria::Or(criterias) => {
                let evaluation_statuses = self.evaluate_criterias(&proposal, criterias).await?;

                if evaluation_statuses
                    .iter()
                    .any(|s| matches!(s, EvaluationStatus::Adopted))
                {
                    return Ok(EvaluationStatus::Adopted);
                }

                if evaluation_statuses
                    .iter()
                    .all(|s| matches!(s, EvaluationStatus::Rejected))
                {
                    return Ok(EvaluationStatus::Rejected);
                }

                Ok(EvaluationStatus::Pending)
            }
            Criteria::Not(criteria) => Ok(
                match self
                    .evaluate((proposal, Arc::new(criteria.as_ref().to_owned())))
                    .await?
                {
                    EvaluationStatus::Pending => EvaluationStatus::Pending,
                    EvaluationStatus::Adopted => EvaluationStatus::Rejected,
                    EvaluationStatus::Rejected => EvaluationStatus::Adopted,
                },
            ),
        }
    }
}
