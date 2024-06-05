use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for managed canister errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum ManagedCanisterError {
    /// The managed canister operation failed.
    #[error(r#"The managed canister operation failed due to {reason}"#)]
    Failed { reason: String },
}

impl DetailableError for ManagedCanisterError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        let ManagedCanisterError::Failed { reason } = &self;
        details.insert("reason".to_string(), reason.to_string());

        Some(details)
    }
}
