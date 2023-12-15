use ic_canister_core::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for wallet install errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum InstallError {
    /// The initialization of the canister failed.
    #[error(r#"The initialization of the canister failed due to {reason}"#)]
    InitFailed { reason: String },
    #[error(r#"The wallet needs at least one owner"#)]
    NoOwnersSpecified,
}

impl DetailableError for InstallError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            InstallError::InitFailed { reason } => {
                details.insert("reason".to_string(), reason.to_string());

                Some(details)
            }
            _ => Some(details),
        }
    }
}
