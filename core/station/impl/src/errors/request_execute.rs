use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for request execution errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum RequestExecuteError {
    /// Request execution failed due to {reason}.
    #[error(r#"Request execution failed due to `{reason}`."#)]
    Failed { reason: String },
    /// Request can't be executed because it was not approved.
    #[error(r#"Request can't be executed because it was not approved."#)]
    NotApproved,
}

impl DetailableError for RequestExecuteError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            RequestExecuteError::Failed { reason } => {
                details.insert("reason".to_string(), reason.to_string());
                Some(details)
            }
            _ => None,
        }
    }
}
