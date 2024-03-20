use super::{
    specifier::{Match, ProposalHasMetadata, ProposalHasVoterInUserSpecifier, UserSpecifier},
    EvaluateError, EvaluationStatus, MetadataItem, Proposal, ProposalVoteStatus, UserId,
    UserStatus,
};
use crate::{
    core::utils::calculate_minimum_threshold,
    errors::MatchError,
    repositories::{UserWhereClause, USER_REPOSITORY},
};
use anyhow::{anyhow, Error};
use ic_canister_macros::storable;
use std::sync::Arc;
use std::{cmp, hash::Hash};

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Criteria {
    // Auto
    AutoAdopted,
    // Votes
    ApprovalThreshold(UserSpecifier, Percentage),
    MinimumVotes(UserSpecifier, u16),
    HasAddressBookMetadata(MetadataItem),
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

pub trait EvaluateCriteria<
    Status = EvaluationStatus,
    Context = (Arc<Proposal>, Arc<Criteria>),
    Error = EvaluateError,
>: Sync + Send
{
    fn evaluate(&self, ctx: Context) -> Result<Status, Error>;
}

#[derive(Clone)]
pub struct CriteriaEvaluator {
    pub user_matcher: Arc<dyn Match<ProposalHasVoterInUserSpecifier>>,
    pub address_book_metadata_matcher: Arc<dyn Match<ProposalHasMetadata>>,
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
        let min_votes = *cmp::min(min_votes, &self.total_possible_votes);
        let uncasted_votes = self
            .total_possible_votes
            .saturating_sub(self.adopted_votes)
            .saturating_sub(self.rejected_votes);

        if self.adopted_votes >= min_votes {
            return EvaluationStatus::Adopted;
        }

        if self.adopted_votes.saturating_add(uncasted_votes) < min_votes {
            return EvaluationStatus::Rejected;
        }

        EvaluationStatus::Pending
    }
}

impl CriteriaEvaluator {
    fn evaluate_criterias(
        &self,
        proposal: &Arc<Proposal>,
        criterias: &[Criteria],
    ) -> Result<Vec<EvaluationStatus>, EvaluateError> {
        criterias
            .iter()
            .map(|criteria| self.evaluate((proposal.to_owned(), Arc::new(criteria.to_owned()))))
            .collect()
    }

    fn find_matching_users<UserMatchReturn>(
        &self,
        proposal: &Arc<Proposal>,
        users: &[(UserId, UserMatchReturn)],
        user_specifier: &UserSpecifier,
    ) -> Result<Vec<UserMatchReturn>, MatchError>
    where
        UserMatchReturn: Clone,
    {
        let mut result = vec![];

        for (user_id, match_return) in users {
            if self.user_matcher.is_match((
                proposal.as_ref().to_owned(),
                user_id.to_owned(),
                user_specifier.to_owned(),
            ))? {
                result.push(match_return.clone());
            }
        }

        Ok(result)
    }

    fn calculate_votes(
        &self,
        proposal: &Arc<Proposal>,
        user_specifier: &UserSpecifier,
    ) -> Result<ProposalVoteSummary, MatchError> {
        let casted_votes = self.find_matching_users::<ProposalVoteStatus>(
            proposal,
            proposal
                .votes
                .iter()
                .map(|vote| (vote.user_id.to_owned(), vote.status.to_owned()))
                .collect::<Vec<(UserId, ProposalVoteStatus)>>()
                .as_slice(),
            user_specifier,
        )?;

        let mut total_possible_votes = self
            .find_matching_users::<()>(
                proposal,
                USER_REPOSITORY
                    .find_where(UserWhereClause {
                        statuses: Some(vec![UserStatus::Active]),
                        search_term: None,
                    })
                    .iter()
                    .map(|user| (user.id.to_owned(), ()))
                    .collect::<Vec<(UserId, ())>>()
                    .as_slice(),
                user_specifier,
            )?
            .len();

        // This is to ensure that the if users become inactive or the criteria is misconfigured
        // the total_possible_votes is not less than the casted votes.
        total_possible_votes = cmp::max(casted_votes.len(), total_possible_votes);

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

impl EvaluateCriteria for CriteriaEvaluator {
    fn evaluate(
        &self,
        (proposal, c): (Arc<Proposal>, Arc<Criteria>),
    ) -> Result<EvaluationStatus, EvaluateError> {
        match c.as_ref() {
            Criteria::AutoAdopted => Ok(EvaluationStatus::Adopted),
            Criteria::ApprovalThreshold(user_specifier, percentage) => {
                let votes = self.calculate_votes(&proposal, user_specifier)?;
                let min_votes =
                    calculate_minimum_threshold(percentage, &votes.total_possible_votes);

                Ok(votes.evaluate(&min_votes))
            }
            Criteria::MinimumVotes(user_specifier, min_votes) => {
                let votes = self.calculate_votes(&proposal, user_specifier)?;
                let min_votes = *min_votes as usize;

                Ok(votes.evaluate(&min_votes))
            }
            Criteria::HasAddressBookMetadata(metadata) => {
                let is_match = self
                    .address_book_metadata_matcher
                    .is_match((proposal.as_ref().to_owned(), metadata.clone()))?;
                if is_match {
                    Ok(EvaluationStatus::Adopted)
                } else {
                    Ok(EvaluationStatus::Rejected)
                }
            }
            Criteria::And(criterias) => {
                let evaluation_statuses = self.evaluate_criterias(&proposal, criterias)?;

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
                let evaluation_statuses = self.evaluate_criterias(&proposal, criterias)?;

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
                match self.evaluate((proposal, Arc::new(criteria.as_ref().to_owned())))? {
                    EvaluationStatus::Pending => EvaluationStatus::Pending,
                    EvaluationStatus::Adopted => EvaluationStatus::Rejected,
                    EvaluationStatus::Rejected => EvaluationStatus::Adopted,
                },
            ),
        }
    }
}
