use crate::utils::to_snake_case;
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
    fn details(&self) -> Option<HashMap<String, String>> {
        None
    }
}

impl<E: std::error::Error + DetailableError> From<E> for ApiError {
    fn from(err: E) -> Self {
        let code = extract_error_enum_variant_name(&err);
        let message = Some(err.to_string());

        ApiError::new(code, message, err.details())
    }
}

pub fn extract_error_enum_variant_name<E: std::error::Error>(err: &E) -> String {
    let full_code = to_snake_case(format!("{:?}", err)).to_uppercase();
    full_code
        .split(|c| c == '{' || c == '(')
        .next()
        .unwrap_or(&full_code)
        .to_string()
        .trim_matches('_')
        .to_string()
}

/// Common result type for service calls, which can either be successful or contain errors.
pub type ApiResult<Data = ()> = Result<Data, ApiError>;

pub type ServiceResult<CompletedOperation = (), FailedOperation = ApiError> =
    Result<CompletedOperation, FailedOperation>;
