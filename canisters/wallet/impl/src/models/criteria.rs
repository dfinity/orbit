use std::sync::Arc;

use anyhow::{anyhow, Error};
use async_trait::async_trait;
use candid::{CandidType, Deserialize};
use futures::{stream, StreamExt, TryStreamExt};
use ic_canister_core::{repository::Repository, types::UUID};
use ic_canister_macros::stable_object;

use crate::{errors::MatchError, repositories::USER_REPOSITORY};

use super::{
    specifier::{Match, UserSpecifier},
    EvaluateError, EvaluationStatus, Proposal, ProposalVoteStatus,
};

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct Ratio(f64);

impl Eq for Ratio {}

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
#[derive(CandidType, Deserialize, Clone, Debug)]
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
pub trait EvaluateCriteria: Sync + Send {
    async fn evaluate(&self, p: &Proposal, c: &Criteria)
        -> Result<EvaluationStatus, EvaluateError>;
}

pub struct CriteriaEvaluator {
    pub user_matcher: Arc<dyn Match<(Proposal, UUID, UserSpecifier)>>,
}

#[async_trait]
impl EvaluateCriteria for CriteriaEvaluator {
    async fn evaluate(
        &self,
        p: &Proposal,
        c: &Criteria,
    ) -> Result<EvaluationStatus, EvaluateError> {
        match c {
            // Approval Threshold
            Criteria::ApprovalThreshold(u, r) => {
                // Existing Votes
                let vs: Vec<ProposalVoteStatus> =
                    stream::iter(p.votes.iter().map(Ok::<_, MatchError>))
                        .try_filter_map(|v| async {
                            Ok(
                                match self
                                    .user_matcher
                                    .is_match((p.to_owned(), v.user_id.to_owned(), u.to_owned()))
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
                                .is_match((p.to_owned(), uu.id.to_owned(), u.to_owned()))
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
                                    .is_match((p.to_owned(), v.user_id.to_owned(), u.to_owned()))
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
                                .is_match((p.to_owned(), uu.id.to_owned(), u.to_owned()))
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
                                .is_match((p.to_owned(), v.user_id.to_owned(), s.to_owned()))
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
                let evs: Vec<EvaluationStatus> =
                    stream::iter(cs.iter().map(Ok::<_, EvaluateError>))
                        .try_filter_map(|c| async { Ok(Some(self.evaluate(p, c).await?)) })
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
                let evs: Vec<EvaluationStatus> =
                    stream::iter(cs.iter().map(Ok::<_, EvaluateError>))
                        .try_filter_map(|c| async { Ok(Some(self.evaluate(p, c).await?)) })
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
            Criteria::Not(c) => Ok(match self.evaluate(p, c).await? {
                EvaluationStatus::Adopted => EvaluationStatus::Rejected,
                EvaluationStatus::Pending => EvaluationStatus::Pending,
                EvaluationStatus::Rejected => EvaluationStatus::Adopted,
            }),
        }
    }
}
