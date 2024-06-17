use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for mapper errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum RegistryError {
    /// The registry entry failed validation.
    #[error(r#"The registry entry failed validation due to {info}."#)]
    ValidationError { info: String },
    /// The registry entry is a duplicate.
    #[error("The registry entry is a duplicate of {id}.")]
    Duplicate { id: String },
    /// The registry entry was not found.
    #[error("The registry entry with id {id} was not found.")]
    NotFound { id: String },
}

impl DetailableError for RegistryError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            RegistryError::ValidationError { info } => {
                details.insert("info".to_string(), info.to_string());
                Some(details)
            }
            RegistryError::NotFound { id } => {
                details.insert("id".to_string(), id.to_string());
                Some(details)
            }
            RegistryError::Duplicate { id } => {
                details.insert("id".to_string(), id.to_string());
                Some(details)
            }
        }
    }
}
