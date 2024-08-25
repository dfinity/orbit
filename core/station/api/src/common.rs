use candid::{CandidType, Deserialize};
use std::collections::HashMap;

pub type TimestampRfc3339 = String;
pub type UuidDTO = String;
pub type Sha256HashDTO = String;

/// Generic error type used for calls.
#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct ApiErrorDTO {
    /// The error code uppercased and underscored (e.g. `INVALID_ARGUMENT`).
    pub code: String,
    /// The error message that describes the error.
    pub message: Option<String>,
    /// The error details if any.
    pub details: Option<HashMap<String, String>>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct PaginationInput {
    pub offset: Option<u64>,
    pub limit: Option<u16>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum CanisterInstallMode {
    #[serde(rename = "install")]
    Install = 1,
    #[serde(rename = "reinstall")]
    Reinstall = 2,
    #[serde(rename = "upgrade")]
    Upgrade = 3,
}
