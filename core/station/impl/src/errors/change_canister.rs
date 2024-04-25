use ic_canister_core::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for canister change errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum ChangeCanisterError {
    /// The canister change failed.
    #[error(r#"The canister change failed due to {reason}"#)]
    Failed { reason: String },
}

impl DetailableError for ChangeCanisterError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        let ChangeCanisterError::Failed { reason } = &self;
        details.insert("reason".to_string(), reason.to_string());

        Some(details)
    }
}
