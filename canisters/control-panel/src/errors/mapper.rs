use ic_canister_core::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for mapper errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum MapperError {
    /// The provided format is not compatible with a UUID.
    #[error(r#"The provided format is not compatible with a UUID."#)]
    MalformedUuid {
        /// The malformed UUID.
        malformed_uuid: String,
    },
}

impl DetailableError for MapperError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            MapperError::MalformedUuid { malformed_uuid } => {
                details.insert("malformed_uuid".to_string(), malformed_uuid.to_string());
                Some(details)
            }
        }
    }
}
