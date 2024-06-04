use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for status call errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum StatusError {
    /// The status call failed.
    #[error(r#"The status call failed due to {reason}"#)]
    Failed { reason: String },
}

impl DetailableError for StatusError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        let StatusError::Failed { reason } = &self;
        details.insert("reason".to_string(), reason.to_string());

        Some(details)
    }
}
