use orbit_essentials::api::DetailableError;
use thiserror::Error;

/// Container for canister errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum CanisterError {
    /// You don't have permission to make the call.
    #[error(r#"You don't have permission to make the call."#)]
    Forbidden {
        /// The called function name.
        method: String,
    },
}

impl DetailableError for CanisterError {
    fn details(&self) -> Option<std::collections::HashMap<String, String>> {
        let mut details = std::collections::HashMap::new();
        match self {
            CanisterError::Forbidden { method } => {
                details.insert("method".to_string(), method.to_string());
                Some(details)
            }
        }
    }
}
