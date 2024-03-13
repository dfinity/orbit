use ic_canister_core::api::DetailableError;
use thiserror::Error;

/// Container for waiting list errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum AllowListError {
    /// The user has failed validation.
    #[error(r#"The user has failed validation."#)]
    ValidationError { info: String },
}

impl DetailableError for AllowListError {
    fn details(&self) -> Option<std::collections::HashMap<String, String>> {
        let mut details = std::collections::HashMap::new();
        match self {
            AllowListError::ValidationError { info } => {
                details.insert("info".to_string(), info.to_string());
                Some(details)
            }
        }
    }
}
