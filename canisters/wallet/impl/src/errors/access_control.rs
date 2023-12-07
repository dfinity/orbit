use ic_canister_core::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AccessControlError {
    #[error(r#"Unauthorized to access `{resource}` with permission to `{access_modifier}`"#)]
    Unauthorized {
        /// The requested resource.
        resource: String,
        /// The requested access modifier.
        access_modifier: String,
    },
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl DetailableError for AccessControlError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            AccessControlError::Unauthorized {
                resource,
                access_modifier,
            } => {
                details.insert("resource".to_string(), resource.to_string());
                details.insert("access_modifier".to_string(), access_modifier.to_string());
                Some(details)
            }
            _ => None,
        }
    }
}
