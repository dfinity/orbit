use ic_canister_core::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for notification errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum NotificationError {
    /// The requested notification was not found.
    #[error(r#"The requested notification was not found."#)]
    NotFound { id: String },
    /// You don't have access to the requested resource.
    #[error(r#"You don't have access to the requested resource."#)]
    Forbidden { id: String },
    /// The notification has failed validation.
    #[error(r#"The notification has failed validation."#)]
    ValidationError { info: String },
}

impl DetailableError for NotificationError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            NotificationError::NotFound { id } => {
                details.insert("id".to_string(), id.to_string());
                Some(details)
            }
            NotificationError::Forbidden { id } => {
                details.insert("id".to_string(), id.to_string());
                Some(details)
            }
            NotificationError::ValidationError { info } => {
                details.insert("info".to_string(), info.to_string());
                Some(details)
            }
        }
    }
}
