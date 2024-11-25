use candid::{CandidType, Deserialize};
use ic_cdk::api::management_canister::main as mgmt;
use orbit_essentials::utils::timestamp_to_rfc3339;
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

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct Snapshot {
    pub snapshot_id: String,
    pub taken_at_timestamp: TimestampRfc3339,
    pub total_size: u64,
}

impl From<mgmt::Snapshot> for Snapshot {
    fn from(snapshot: mgmt::Snapshot) -> Snapshot {
        Snapshot {
            snapshot_id: hex::encode(&snapshot.id),
            taken_at_timestamp: timestamp_to_rfc3339(&snapshot.taken_at_timestamp),
            total_size: snapshot.total_size,
        }
    }
}
