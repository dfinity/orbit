use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for mapper errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum RegistryError {
    /// The registry entry failed validation.
    #[error(r#"The registry entry failed validation due to {info}."#)]
    ValidationError { info: String },
    /// Updates to the registry entry must keep the same type.
    #[error("Updates to the registry entry must use a value of {kind}.")]
    UpdateKindMismatch {
        /// The current type of the registry entry.
        kind: String,
    },
    /// The registry entry was not found.
    #[error("The registry entry with id {id} was not found.")]
    NotFound { id: String },
    /// Package with name not found.
    #[error("Package with name {name} not found.")]
    PackageNotFound { name: String },
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
            RegistryError::UpdateKindMismatch { kind } => {
                details.insert("kind".to_string(), kind.to_string());
                Some(details)
            }
            RegistryError::PackageNotFound { name } => {
                details.insert("name".to_string(), name.to_string());
                Some(details)
            }
        }
    }
}
