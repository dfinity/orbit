use candid::{CandidType, Deserialize};
use std::collections::HashMap;

/// Generic error type used for calls.
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ApiErrorDTO {
    /// The error code uppercased and underscored (e.g. `INVALID_ARGUMENT`).
    pub code: String,
    /// The error message that describes the error.
    pub message: Option<String>,
    /// The error details if any.
    pub details: Option<HashMap<String, String>>,
}

