use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for mapper errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum ArtifactError {
    /// The registry entry failed validation.
    #[error(r#"The registry entry failed validation due to {info}."#)]
    ValidationError { info: String },
    /// The artifact was not found.
    #[error("The artifact with id {id} was not found.")]
    NotFound { id: String },
}

impl DetailableError for ArtifactError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            ArtifactError::ValidationError { info } => {
                details.insert("info".to_string(), info.to_string());
                Some(details)
            }
            ArtifactError::NotFound { id } => {
                details.insert("id".to_string(), id.to_string());
                Some(details)
            }
        }
    }
}
