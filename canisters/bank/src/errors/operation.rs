use ic_canister_core::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for system operation errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum OperationError {
    /// The requested system operation was not found.
    #[error(r#"The requested system operation was not found."#)]
    OperationNotFound { operation_id: String },
    /// You don't have access to the requested resource.
    #[error(r#"You don't have access to the requested resource."#)]
    Forbidden { operation_id: String },
    /// Operations that have already been completed cannot be modified.
    #[error(r#"This operation was already completed, it cannot be modified."#)]
    NotAllowedModification { operation_id: String },
    /// The reason for the operation status is too long.
    #[error(r#"The reason for the operation status is too long."#)]
    TaskReasonTooLong { max_len: u8 },
    /// The operation has failed validation.
    #[error(r#"The operation has failed validation."#)]
    ValidationError { info: String },
}

impl DetailableError for OperationError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            OperationError::OperationNotFound { operation_id } => {
                details.insert("operation_id".to_string(), operation_id.to_string());
                Some(details)
            }
            OperationError::Forbidden { operation_id } => {
                details.insert("operation_id".to_string(), operation_id.to_string());
                Some(details)
            }
            OperationError::NotAllowedModification { operation_id } => {
                details.insert("operation_id".to_string(), operation_id.to_string());
                Some(details)
            }
            OperationError::TaskReasonTooLong { max_len } => {
                details.insert("max_len".to_string(), max_len.to_string());
                Some(details)
            }
            OperationError::ValidationError { info } => {
                details.insert("info".to_string(), info.to_string());
                Some(details)
            }
        }
    }
}
