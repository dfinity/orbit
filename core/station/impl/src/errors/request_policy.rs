use crate::errors::{
    ExternalCanisterValidationError, RecordValidationError, SystemInfoValidationError,
    ValidationError,
};
use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

use super::FieldValidationError;

/// Container for request policy errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum RequestPolicyError {
    /// The request policy has failed validation.
    #[error(r#"The request policy has failed validation."#)]
    ValidationError { info: String },
    /// Request policy with id `{id}` already exists.
    #[error(r#"Request policy with id `{id}` already exists."#)]
    IdAlreadyExists { id: String },
}

impl DetailableError for RequestPolicyError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            RequestPolicyError::ValidationError { info } => {
                details.insert("info".to_string(), info.to_string());
                Some(details)
            }
            RequestPolicyError::IdAlreadyExists { id } => {
                details.insert("id".to_string(), id.to_string());
                Some(details)
            }
        }
    }
}

impl From<RecordValidationError> for RequestPolicyError {
    fn from(err: RecordValidationError) -> RequestPolicyError {
        match err {
            RecordValidationError::NotFound { id, model_name } => {
                RequestPolicyError::ValidationError {
                    info: format!("Invalid UUID: {} {} not found", model_name, id),
                }
            }
        }
    }
}

impl From<ExternalCanisterValidationError> for RequestPolicyError {
    fn from(err: ExternalCanisterValidationError) -> RequestPolicyError {
        match err {
            ExternalCanisterValidationError::InvalidExternalCanister { principal } => {
                RequestPolicyError::ValidationError {
                    info: format!("Invalid external canister {}", principal),
                }
            }
            ExternalCanisterValidationError::ValidationError { info } => {
                RequestPolicyError::ValidationError { info }
            }
        }
    }
}

impl From<SystemInfoValidationError> for RequestPolicyError {
    fn from(err: SystemInfoValidationError) -> RequestPolicyError {
        RequestPolicyError::ValidationError {
            info: err.to_string(),
        }
    }
}

impl From<FieldValidationError> for RequestPolicyError {
    fn from(err: FieldValidationError) -> RequestPolicyError {
        RequestPolicyError::ValidationError {
            info: err.to_string(),
        }
    }
}

impl From<ValidationError> for RequestPolicyError {
    fn from(err: ValidationError) -> RequestPolicyError {
        match err {
            ValidationError::RecordValidationError(err) => err.into(),
            ValidationError::ExternalCanisterValidationError(err) => err.into(),
            ValidationError::SystemInfoValidationError(err) => err.into(),
            ValidationError::FieldValidationError(err) => err.into(),
        }
    }
}
