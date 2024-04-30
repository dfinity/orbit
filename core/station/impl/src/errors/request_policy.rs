use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for request policy errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum RequestPolicyError {
    /// The request policy has failed validation.
    #[error(r#"The request policy has failed validation."#)]
    ValidationError { info: String },
}

impl DetailableError for RequestPolicyError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            RequestPolicyError::ValidationError { info } => {
                details.insert("info".to_string(), info.to_string());
                Some(details)
            }
        }
    }
}
