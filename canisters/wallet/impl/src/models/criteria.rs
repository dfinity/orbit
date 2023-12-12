use super::{
    specifier::{Match, UserSpecifier},
    EvaluateError, EvaluationStatus, Proposal, ProposalVoteStatus, UserId,
};
use crate::{errors::MatchError, repositories::USER_REPOSITORY};
use anyhow::{anyhow, Error};
use async_trait::async_trait;
use candid::{CandidType, Deserialize};
use futures::{stream, StreamExt, TryStreamExt};
use ic_canister_core::{repository::Repository, types::UUID};
use ic_canister_macros::stable_object;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Ratio(pub f64);

impl Ratio {
    fn is_nan(&self) -> bool {
        self.0.is_nan()
    }
}

impl Eq for Ratio {}

impl PartialEq for Ratio {
    fn eq(&self, other: &Self) -> bool {
        if self.is_nan() || other.is_nan() {
            false
        } else {
            self.0 == other.0
        }
    }
}

impl Hash for Ratio {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Handle special cases such as NaN and zeros
        if self.0.is_nan() {
            // Hash a constant value for NaN
            "NaN".hash(state);
        } else if self.0 == 0.0 {
            // Hash zero consistently, regardless of sign
            0.0f64.to_bits().hash(state);
        } else {
            // For normal cases, hash the bit representation
            self.0.to_bits().hash(state);
        }
    }
}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ratio {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.0.is_nan(), other.0.is_nan()) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            _ => {
                if self.0 == other.0 {
                    // Handle the case of positive and negative zero
                    if self.0.is_sign_positive() == other.0.is_sign_positive() {
                        Ordering::Equal
                    } else if self.0.is_sign_positive() {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                } else {
                    self.0.partial_cmp(&other.0).unwrap()
                }
            }
        }
    }
}

impl TryFrom<f64> for Ratio {
    type Error = Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value <= 0. || value > 1. {
            return Err(anyhow!("invalid ratio value, must be between > 0 and <= 1"));
        }

        Ok(Ratio(value))
    }
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Criteria {
    // Auto
    Auto(EvaluationStatus),
    // Votes
    ApprovalThreshold(UserSpecifier, Ratio),
    MinimumVotes(UserSpecifier, u16),
    // Metadata
    IsAddressKYC,
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
}

#[async_trait]
impl EvaluateCriteria for CriteriaEvaluator {
    async fn evaluate(
        &self,
        (proposal, c): (Arc<Proposal>, Arc<Criteria>),
    ) -> Result<EvaluationStatus, EvaluateError> {
        match c.as_ref() {
            Criteria::Auto(status) => Ok(status.clone()),
            // TODO: Add evaluation of KYC criteria once address book is implemented
            Criteria::IsAddressKYC => todo!(),
            Criteria::ApprovalThreshold(user_specifier, ratio) => {
                let casted_votes = self
                    .find_matching_users::<ProposalVoteStatus>(
                        &proposal,
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
                        &proposal,
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

                // Evaluate Status
                let v_adopt = casted_votes
                    .iter()
                    .filter(|&v| matches!(v, ProposalVoteStatus::Accepted))
                    .count();
                let v_adopt = v_adopt as f64;

                let v_unvoted = total_possible_votes.saturating_sub(casted_votes.len());
                let v_unvoted = v_unvoted as f64;

                if (v_adopt / total_possible_votes as f64) >= ratio.0 {
                    return Ok(EvaluationStatus::Adopted);
                }

                if ((v_adopt + v_unvoted) / total_possible_votes as f64) < ratio.0 {
                    return Ok(EvaluationStatus::Rejected);
                }

                Ok(EvaluationStatus::Pending)
            }
            Criteria::MinimumVotes(user_specifier, min_votes) => {
                let casted_votes = self
                    .find_matching_users::<ProposalVoteStatus>(
                        &proposal,
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
                        &proposal,
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

                // Evaluate Status
                let v_adopt = casted_votes
                    .iter()
                    .filter(|&v| matches!(v, ProposalVoteStatus::Accepted))
                    .count();

                let v_unvoted = total_possible_votes.saturating_sub(casted_votes.len());

                if v_adopt >= *min_votes as usize {
                    return Ok(EvaluationStatus::Adopted);
                }

                if (v_adopt + v_unvoted) < *min_votes as usize {
                    return Ok(EvaluationStatus::Rejected);
                }

                Ok(EvaluationStatus::Pending)
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
                    .any(|s| matches!(s, EvaluationStatus::Rejected))
                {
                    return Ok(EvaluationStatus::Rejected);
                }

                if evaluation_statuses
                    .iter()
                    .any(|s| matches!(s, EvaluationStatus::Adopted))
                {
                    return Ok(EvaluationStatus::Adopted);
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
