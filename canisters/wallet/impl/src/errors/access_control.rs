use ic_canister_core::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AccessControlError {
    #[error(r#"Unauthorized to access to resource `{resource}`"#)]
    Unauthorized {
        /// The requested resource.
        resource: String,
    },
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl DetailableError for AccessControlError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            AccessControlError::Unauthorized { resource } => {
                details.insert("resource".to_string(), resource.to_string());
                Some(details)
            }
            _ => None,
        }
    }
}
