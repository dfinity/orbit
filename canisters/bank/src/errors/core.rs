use ic_canister_core::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for core errors used across the canister.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum CoreError {
    /// The provided format is not compatible with a UUID.
    #[error(r#"The provided format is not compatible with a UUID."#)]
    MalformedUuid {
        /// The malformed UUID.
        malformed_uuid: String,
    },
}

impl DetailableError for CoreError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        let CoreError::MalformedUuid { malformed_uuid } = self;
        details.insert("malformed_uuid".to_string(), malformed_uuid.to_string());

        return Some(details);
    }
}
