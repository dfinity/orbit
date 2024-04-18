use super::{criteria::Criteria, specifier::ProposalSpecifier};
use crate::errors::{MatchError, PolicyError};
use candid::{CandidType, Deserialize};
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::UUID,
};
use ic_canister_macros::storable;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EvaluationStatus {
    Adopted,
    Pending,
    Rejected,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalPolicy {
    pub id: UUID,
    pub specifier: ProposalSpecifier,
    pub criteria: Criteria,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ProposalPolicyCallerPrivileges {
    pub id: UUID,
    pub can_edit: bool,
    pub can_delete: bool,
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

impl ModelValidator<PolicyError> for ProposalPolicy {
    fn validate(&self) -> ModelValidatorResult<PolicyError> {
        // todo: can't validate self.specifier because it might reference a resource that doesn't exist yet (eg. when creating a new account)
        self.criteria.validate()?;
        Ok(())
    }
}

#[cfg(test)]
pub mod proposal_policy_test_utils {
    use super::ProposalPolicy;
    use crate::models::{criteria::Criteria, specifier::ProposalSpecifier};

    pub fn mock_proposal_policy() -> ProposalPolicy {
        ProposalPolicy {
            id: [0; 16],
            specifier: ProposalSpecifier::AddAccount,
            criteria: Criteria::AutoAdopted,
        }
    }
}
