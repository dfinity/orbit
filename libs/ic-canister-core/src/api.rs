use crate::utils::to_snake_case;
use candid::{CandidType, Deserialize};
use serde_json::json;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

/// Generic service error type used for service calls.
#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ApiError {
    /// The error code uppercased and underscored (e.g. `INVALID_ARGUMENT`).
    pub code: String,
    /// The error message that describes the error.
    pub message: Option<String>,
    /// The error details if any.
    pub details: Option<HashMap<String, String>>,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.code,
            self.message.as_ref().unwrap_or(&"no message".to_string())
        )
    }
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

    pub fn to_json_string(&self) -> String {
        let mut map = HashMap::new();
        map.insert("code".to_string(), self.code.clone());
        map.insert(
            "message".to_string(),
            self.message.clone().unwrap_or("".to_string()),
        );
        map.insert(
            "details".to_string(),
            json!(&self.details.clone().unwrap_or_default()).to_string(),
        );

        json!(map).to_string()
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
