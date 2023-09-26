use candid::{CandidType, Deserialize};
use std::collections::HashMap;

/// Generic service error type used for service calls.
#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ApiError {
    /// The error code uppercased and underscored (e.g. `INVALID_ARGUMENT`).
    code: String,
    /// The error message that describes the error.
    message: Option<String>,
    /// The error details if any.
    details: Option<HashMap<String, String>>,
}

impl ApiError {
    /// Creates a new service error.
    pub fn new(
        code: String,
        message: Option<String>,
        details: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            code,
            message,
            details,
        }
    }
}

/// Common result type for service calls, which can either be successful or contain errors.
pub type ApiResult<Data = ()> = Result<Data, ApiError>;

pub type ServiceResult<CompletedOperation = (), FailedOperation = ApiError> =
    Result<CompletedOperation, FailedOperation>;
