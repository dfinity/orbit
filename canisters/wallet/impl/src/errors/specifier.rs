use ic_canister_core::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

use crate::core::validation::RecordNotFoundError;

/// Container for user specifier errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum UserSpecifierError {
    /// The user specifier has failed validation.
    #[error(r#"The user specifier has failed validation."#)]
    ValidationError { info: String },
}

impl DetailableError for UserSpecifierError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            UserSpecifierError::ValidationError { info } => {
                details.insert("info".to_string(), info.to_string());
                Some(details)
            }
        }
    }
}

impl From<RecordNotFoundError> for UserSpecifierError {
    fn from(error: RecordNotFoundError) -> Self {
        UserSpecifierError::ValidationError {
            info: format!("{} record not found", error.model_name),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MatchError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
