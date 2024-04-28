use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for system request errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum RequestError {
    /// The requested system request was not found.
    #[error(r#"The requested system request was not found."#)]
    NotFound { request_id: String },
    /// You don't have access to the requested resource.
    #[error(r#"You don't have access to the requested resource."#)]
    Forbidden { request_id: String },
    /// Requests that have already been completed cannot be modified.
    #[error(r#"This request was already completed, it cannot be modified."#)]
    NotAllowedModification { request_id: String },
    /// The reason for the request approval is too long.
    #[error(r#"The reason for the request status is too long."#)]
    ApprovalReasonTooLong { max_len: u8 },
    /// The request has failed validation.
    #[error(r#"The request has failed validation."#)]
    ValidationError { info: String },
    /// You can't add your approval decision to the request.
    #[error(r#"You can't add your approval decision to the request."#)]
    ApprovalNotAllowed,
    /// Request execution failed due to {reason}.
    #[error(r#"Request execution failed due to `{reason}`."#)]
    ExecutionError { reason: String },
    /// Request can't be executed because it was not adopted.
    #[error(r#"Request can't be executed because it was not adopted."#)]
    ExecutionFailedNotAdopted,
    #[error(r#"You don't have permission to create the requested request."#)]
    Unauthorized,
    /// Request policy not found for id `{id}`.
    #[error(r#"Request policy not found for id `{id}`"#)]
    PolicyNotFound { id: String },
}

impl DetailableError for RequestError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            RequestError::NotFound { request_id } => {
                details.insert("request_id".to_string(), request_id.to_string());
                Some(details)
            }
            RequestError::Forbidden { request_id } => {
                details.insert("request_id".to_string(), request_id.to_string());
                Some(details)
            }
            RequestError::NotAllowedModification { request_id } => {
                details.insert("request_id".to_string(), request_id.to_string());
                Some(details)
            }
            RequestError::ApprovalReasonTooLong { max_len } => {
                details.insert("max_len".to_string(), max_len.to_string());
                Some(details)
            }
            RequestError::ValidationError { info } => {
                details.insert("info".to_string(), info.to_string());
                Some(details)
            }
            RequestError::ExecutionError { reason } => {
                details.insert("reason".to_string(), reason.to_string());
                Some(details)
            }
            RequestError::PolicyNotFound { id } => {
                details.insert("id".to_string(), id.to_string());
                Some(details)
            }
            _ => None,
        }
    }
}
