use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for mapper errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum RegistryError {
    /// The registry entry failed validation.
    #[error(r#"The registry entry failed validation due to {info}."#)]
    ValidationError { info: String },
}

impl DetailableError for RegistryError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            RegistryError::ValidationError { info } => {
                details.insert("info".to_string(), info.to_string());
                Some(details)
            }
        }
    }
}
