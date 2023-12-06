use candid::{CandidType, Deserialize};
use ic_canister_core::types::UUID;
use ic_canister_macros::stable_object;

use crate::errors::MatchError;

use super::{criteria::Criteria, specifier::ProposalSpecifier};

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EvaluationStatus {
    Adopted,
    Pending,
    Rejected,
}

/// Represents a policy within the system.
///
/// Policies are used to define the rules of operating within the wallet, including approval thresholds for
/// operations and others.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Policy {
    ApprovalThreshold(ApprovalThresholdPolicy),
}

/// Represents an approval threshold policy.
///
/// This policy defines the number of approvals required for operations to be executed.
/// It can be either a fixed number or percentage, based on the number of owners.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ApprovalThresholdPolicy {
    VariableThreshold(u8),
    FixedThreshold(u8),
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ProposalPolicy {
    pub id: UUID,
    pub specifier: ProposalSpecifier,
    pub criteria: Criteria,
}

#[derive(Debug, thiserror::Error)]
pub enum EvaluateError {
    #[error("unauthorized")]
    Unauthorized,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl From<MatchError> for EvaluateError {
    fn from(value: MatchError) -> Self {
        match value {
            MatchError::UnexpectedError(err) => EvaluateError::UnexpectedError(err),
        }
    }
}
