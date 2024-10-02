use candid::Principal;
use orbit_essentials::api::DetailableError;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    RecordValidationError(RecordValidationError),
    ExternalCanisterValidationError(ExternalCanisterValidationError),
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::RecordValidationError(err) => write!(f, "{}", err),
            ValidationError::ExternalCanisterValidationError(err) => write!(f, "{}", err),
        }
    }
}

impl DetailableError for ValidationError {
    fn details(&self) -> Option<std::collections::HashMap<String, String>> {
        match self {
            ValidationError::RecordValidationError(err) => err.details(),
            ValidationError::ExternalCanisterValidationError(err) => err.details(),
        }
    }
}

impl From<RecordValidationError> for ValidationError {
    fn from(err: RecordValidationError) -> ValidationError {
        ValidationError::RecordValidationError(err)
    }
}

impl From<ExternalCanisterValidationError> for ValidationError {
    fn from(err: ExternalCanisterValidationError) -> ValidationError {
        ValidationError::ExternalCanisterValidationError(err)
    }
}

#[derive(Debug, Error)]
pub enum RecordValidationError {
    #[error(r#"The {model_name} {id} does not exist."#)]
    NotFound { model_name: String, id: String },
}

impl DetailableError for RecordValidationError {
    fn details(&self) -> Option<std::collections::HashMap<String, String>> {
        let mut details = std::collections::HashMap::new();

        match self {
            RecordValidationError::NotFound { model_name, id } => {
                details.insert("model_name".to_string(), model_name.to_string());
                details.insert("id".to_string(), id.to_string());
                Some(details)
            }
        }
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ExternalCanisterValidationError {
    #[error(r#"The principal {principal} is an invalid external canister."#)]
    InvalidExternalCanister { principal: Principal },
    #[error(r#"The external canister has failed validation with reason `{info}`."#)]
    ValidationError { info: String },
}

impl DetailableError for ExternalCanisterValidationError {
    fn details(&self) -> Option<std::collections::HashMap<String, String>> {
        let mut details = std::collections::HashMap::new();

        match self {
            ExternalCanisterValidationError::InvalidExternalCanister { principal } => {
                details.insert("principal".to_string(), principal.to_string());
                Some(details)
            }
            ExternalCanisterValidationError::ValidationError { info } => {
                details.insert("info".to_string(), info.to_string());
                Some(details)
            }
        }
    }
}
