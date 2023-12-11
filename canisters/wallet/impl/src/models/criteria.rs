use super::{
    specifier::{Match, UserSpecifier},
    EvaluateError, EvaluationStatus, Proposal, ProposalVoteStatus,
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
    VetoPower(UserSpecifier),

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

#[async_trait]
impl EvaluateCriteria for CriteriaEvaluator {
    async fn evaluate(
        &self,
        (p, c): (Arc<Proposal>, Arc<Criteria>),
    ) -> Result<EvaluationStatus, EvaluateError> {
        match c.as_ref() {
            // Approval Threshold
            Criteria::ApprovalThreshold(u, r) => {
                // Existing Votes
                let vs: Vec<ProposalVoteStatus> =
                    stream::iter(p.votes.iter().map(Ok::<_, MatchError>))
                        .try_filter_map(|v| async {
                            Ok(
                                match self
                                    .user_matcher
                                    .is_match((
                                        p.as_ref().to_owned(),
                                        v.user_id.to_owned(),
                                        u.to_owned(),
                                    ))
                                    .await?
                                {
                                    false => None,
                                    true => Some(v.status.clone()),
                                },
                            )
                        })
                        .try_collect()
                        .await?;

                // Overall Voter Count
                let vs_total = stream::iter(USER_REPOSITORY.list().iter().map(Ok::<_, MatchError>))
                    .try_filter_map(|uu| async {
                        Ok(
                            match self
                                .user_matcher
                                .is_match((p.as_ref().to_owned(), uu.id.to_owned(), u.to_owned()))
                                .await?
                            {
                                false => None,
                                true => Some(()),
                            },
                        )
                    })
                    .count()
                    .await;

                // Evaluate Status
                let v_adopt = vs
                    .iter()
                    .filter(|&v| matches!(v, ProposalVoteStatus::Accepted))
                    .count();
                let v_adopt = v_adopt as f64;

                let v_unvoted = vs_total - vs.len();
                let v_unvoted = v_unvoted as f64;

                if (v_adopt / vs_total as f64) >= r.0 {
                    return Ok(EvaluationStatus::Adopted);
                }

                if ((v_adopt + v_unvoted) / vs_total as f64) < r.0 {
                    return Ok(EvaluationStatus::Rejected);
                }

                Ok(EvaluationStatus::Pending)
            }

            // Minimum Votes
            Criteria::MinimumVotes(u, c) => {
                // Existing Votes
                let vs: Vec<ProposalVoteStatus> =
                    stream::iter(p.votes.iter().map(Ok::<_, MatchError>))
                        .try_filter_map(|v| async {
                            Ok(
                                match self
                                    .user_matcher
                                    .is_match((
                                        p.as_ref().to_owned(),
                                        v.user_id.to_owned(),
                                        u.to_owned(),
                                    ))
                                    .await?
                                {
                                    false => None,
                                    true => Some(v.status.clone()),
                                },
                            )
                        })
                        .try_collect()
                        .await?;

                // Overall Voter Count
                let vs_total = stream::iter(USER_REPOSITORY.list().iter().map(Ok::<_, MatchError>))
                    .try_filter_map(|uu| async {
                        Ok(
                            match self
                                .user_matcher
                                .is_match((p.as_ref().to_owned(), uu.id.to_owned(), u.to_owned()))
                                .await?
                            {
                                false => None,
                                true => Some(()),
                            },
                        )
                    })
                    .count()
                    .await;

                // Evaluate Status
                let v_adopt = vs
                    .iter()
                    .filter(|&v| matches!(v, ProposalVoteStatus::Accepted))
                    .count();

                let v_unvoted = vs_total - vs.len();

                if v_adopt >= *c as usize {
                    return Ok(EvaluationStatus::Adopted);
                }

                if (v_adopt + v_unvoted) < *c as usize {
                    return Ok(EvaluationStatus::Rejected);
                }

                Ok(EvaluationStatus::Pending)
            }

            // Auto
            Criteria::Auto(status) => Ok(status.clone()),

            // Veto
            Criteria::VetoPower(s) => {
                for v in &p.votes {
                    match v.status {
                        ProposalVoteStatus::Accepted | ProposalVoteStatus::Rejected => {
                            if self
                                .user_matcher
                                .is_match((
                                    p.as_ref().to_owned(),
                                    v.user_id.to_owned(),
                                    s.to_owned(),
                                ))
                                .await?
                            {
                                return Ok(v.status.clone().into());
                            }
                        }
                    }
                }

                Ok(EvaluationStatus::Pending)
            }

            // IsAddressKYC
            Criteria::IsAddressKYC => todo!(),

            // And
            Criteria::And(cs) => {
                let proposal = &p;
                let evs: Vec<EvaluationStatus> =
                    stream::iter(cs.iter().map(Ok::<_, EvaluateError>))
                        .try_filter_map(|c| async {
                            Ok(Some(
                                self.evaluate((proposal.to_owned(), Arc::new(c.to_owned())))
                                    .await?,
                            ))
                        })
                        .try_collect()
                        .await?;

                if evs.iter().any(|s| matches!(s, EvaluationStatus::Rejected)) {
                    return Ok(EvaluationStatus::Rejected);
                }

                if evs.iter().all(|s| matches!(s, EvaluationStatus::Adopted)) {
                    return Ok(EvaluationStatus::Adopted);
                }

                Ok(EvaluationStatus::Pending)
            }

            // Or
            Criteria::Or(cs) => {
                let proposal = &p;
                let evs: Vec<EvaluationStatus> =
                    stream::iter(cs.iter().map(Ok::<_, EvaluateError>))
                        .try_filter_map(|c| async {
                            Ok(Some(
                                self.evaluate((proposal.to_owned(), Arc::new(c.to_owned())))
                                    .await?,
                            ))
                        })
                        .try_collect()
                        .await?;

                if evs.iter().any(|s| matches!(s, EvaluationStatus::Rejected)) {
                    return Ok(EvaluationStatus::Rejected);
                }

                if evs.iter().any(|s| matches!(s, EvaluationStatus::Adopted)) {
                    return Ok(EvaluationStatus::Adopted);
                }

                Ok(EvaluationStatus::Pending)
            }

            // Not
            Criteria::Not(c) => Ok(
                match self.evaluate((p, Arc::new(c.as_ref().to_owned()))).await? {
                    EvaluationStatus::Adopted => EvaluationStatus::Rejected,
                    EvaluationStatus::Pending => EvaluationStatus::Pending,
                    EvaluationStatus::Rejected => EvaluationStatus::Adopted,
                },
            ),
        }
    }
}
