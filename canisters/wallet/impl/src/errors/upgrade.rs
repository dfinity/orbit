use ic_canister_core::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for upgrade errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum UpgradeError {
    /// The upgrade of the canister failed.
    #[error(r#"The upgrade of the canister failed due to {reason}"#)]
    Failed { reason: String },
    /// No pending proposal was found during the upgrade of the wallet
    #[error(r#"No pending proposal was found during the upgrade of the wallet."#)]
    MissingUpgradeProposal,
}

impl DetailableError for UpgradeError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            UpgradeError::Failed { reason } => {
                details.insert("reason".to_string(), reason.to_string());
                Some(details)
            }
            _ => Some(details),
        }
    }
}
