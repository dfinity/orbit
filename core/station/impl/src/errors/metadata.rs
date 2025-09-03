use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

use super::ValidationError;

/// Container for metadata errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum MetadataError {
    /// The metadata has failed validation.
    #[error(r#"The metadata have failed validation."#)]
    ValidationError { info: String },
}

impl From<ValidationError> for MetadataError {
    fn from(err: ValidationError) -> Self {
        MetadataError::ValidationError {
            info: err.to_string(),
        }
    }
}

impl DetailableError for MetadataError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            MetadataError::ValidationError { info } => {
                details.insert("info".to_string(), info.to_string());
                Some(details)
            }
        }
    }
}
