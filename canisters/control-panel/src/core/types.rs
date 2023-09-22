use super::extract_error_enum_variant_name;
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

pub trait DetailableError {
    fn details(&self) -> Option<HashMap<String, String>>;
}

impl<E: std::error::Error + DetailableError> From<E> for ApiError {
    fn from(err: E) -> Self {
        let code = extract_error_enum_variant_name(&err);
        let message = Some(err.to_string());

        ApiError::new(code, message, err.details())
    }
}

/// Common result type for service calls, which can either be successful or contain errors.
// #[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub type ApiResult<Data = ()> = Result<Data, ApiError>;

/// A UUID that identifies objects within the system.
pub type UUID = [u8; 16];

/// A timestamp in nano seconds since epoch.
pub type Timestamp = u64;

pub type ServiceResult<T = (), E = ApiError> = Result<T, E>;
