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
    #[error(r#"There are too many admins defined, max allowed is {max}."#)]
    TooManyAdminsSpecified { max: usize },
    #[error(r#"System upgrade failed."#)]
    UpgradeFailed { reason: String },
    #[error(r#"No station upgrade request is processing."#)]
    NoStationUpgradeProcessing,
}

impl DetailableError for SystemError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            SystemError::InitFailed { reason } => {
                details.insert("reason".to_string(), reason.to_string());

                Some(details)
            }
            SystemError::TooManyAdminsSpecified { max } => {
                details.insert("max".to_string(), max.to_string());

                Some(details)
            }
            SystemError::UpgradeFailed { reason } => {
                details.insert("reason".to_string(), reason.to_string());

                Some(details)
            }
            _ => Some(details),
        }
    }
}
