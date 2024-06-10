use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for external canister errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum ExternalCanisterError {
    /// The external canister operation failed.
    #[error(r#"The external canister operation failed due to {reason}"#)]
    Failed { reason: String },
}

impl DetailableError for ExternalCanisterError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        let ExternalCanisterError::Failed { reason } = &self;
        details.insert("reason".to_string(), reason.to_string());

        Some(details)
    }
}
