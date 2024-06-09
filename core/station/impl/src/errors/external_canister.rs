use crate::errors::ExternalCanisterValidationError;
use candid::Principal;
use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for external canister errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum ExternalCanisterError {
    /// The external canister operation failed in validation.
    #[error(r#"The principal {principal} is an invalid external canister."#)]
    InvalidExternalCanister { principal: Principal },
    /// The external canister operation failed in execution.
    #[error(r#"The external canister operation failed due to {reason}"#)]
    Failed { reason: String },
}

impl DetailableError for ExternalCanisterError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();

        match self {
            ExternalCanisterError::InvalidExternalCanister { principal } => {
                details.insert("principal".to_string(), principal.to_string());
            }
            ExternalCanisterError::Failed { reason } => {
                details.insert("reason".to_string(), reason.to_string());
            }
        }

        Some(details)
    }
}

impl From<ExternalCanisterValidationError> for ExternalCanisterError {
    fn from(err: ExternalCanisterValidationError) -> ExternalCanisterError {
        match err {
            ExternalCanisterValidationError::InvalidExternalCanister { principal } => {
                ExternalCanisterError::InvalidExternalCanister { principal }
            }
        }
    }
}
