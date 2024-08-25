use super::{request_policy_rule::RequestPolicyRule, request_specifier::RequestSpecifier};
use crate::errors::{MatchError, RequestPolicyError};
use candid::{CandidType, Deserialize};
use orbit_essentials::model::ModelKey;
use orbit_essentials::storable;
use orbit_essentials::{
    model::{ModelValidator, ModelValidatorResult},
    types::UUID,
};

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EvaluationStatus {
    Approved,
    Rejected,
    Pending,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestPolicy {
    pub id: UUID,
    pub specifier: RequestSpecifier,
    pub rule: RequestPolicyRule,
}

impl ModelKey<UUID> for RequestPolicy {
    fn key(&self) -> UUID {
        self.id
    }
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct RequestPolicyCallerPrivileges {
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

impl ModelValidator<RequestPolicyError> for RequestPolicy {
    fn validate(&self) -> ModelValidatorResult<RequestPolicyError> {
        self.specifier.validate()?;
        self.rule.validate()?;
        Ok(())
    }
}

#[cfg(test)]
pub mod request_policy_test_utils {
    use super::RequestPolicy;
    use crate::models::{
        request_policy_rule::RequestPolicyRule, request_specifier::RequestSpecifier,
    };
    use uuid::Uuid;

    pub fn mock_request_policy() -> RequestPolicy {
        RequestPolicy {
            id: *Uuid::new_v4().as_bytes(),
            specifier: RequestSpecifier::AddAccount,
            rule: RequestPolicyRule::AutoApproved,
        }
    }
}
