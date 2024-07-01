use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for disaster recovery errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum DisasterRecoveryError {
    /// Account sync failed.
    #[error(r#"Failed to sync accounts to the upgrader due to {reason}"#)]
    AccountSyncFailed { reason: String },

    /// Committee sync failed.
    #[error(r#"Failed to sync the committee to the upgrader due to {reason}"#)]
    CommitteeSyncFailed { reason: String },
}

impl DetailableError for DisasterRecoveryError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();

        match self {
            DisasterRecoveryError::AccountSyncFailed { reason } => {
                details.insert("reason".to_string(), reason.to_string());
            }
            DisasterRecoveryError::CommitteeSyncFailed { reason } => {
                details.insert("reason".to_string(), reason.to_string());
            }
        }

        Some(details)
    }
}
