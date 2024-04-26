use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for policy errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum PolicyError {
    /// The policy has failed validation.
    #[error(r#"The policy has failed validation."#)]
    ValidationError { info: String },
}

impl DetailableError for PolicyError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            PolicyError::ValidationError { info } => {
                details.insert("info".to_string(), info.to_string());
                Some(details)
            }
        }
    }
}
