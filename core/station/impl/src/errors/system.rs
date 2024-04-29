use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for canister install errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum SystemError {
    /// The initialization of the canister failed.
    #[error(r#"The initialization of the canister failed due to {reason}"#)]
    InitFailed { reason: String },
    #[error(r#"The canister needs at least one admin"#)]
    NoAdminsSpecified,
}

impl DetailableError for SystemError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            SystemError::InitFailed { reason } => {
                details.insert("reason".to_string(), reason.to_string());

                Some(details)
            }
            _ => Some(details),
        }
    }
}
