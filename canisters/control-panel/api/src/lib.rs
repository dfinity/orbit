//! Transport layer that defines the data transfer objects (DTOs) that are used to communicate
//! with the clients of the control panel.

/// User DTOs.
mod user;
use candid::{CandidType, Deserialize};
pub use user::*;

/// User Wallet DTOs.
mod user_wallet;
pub use user_wallet::*;

/// Manage user DTOs.
mod manage_user;
pub use manage_user::*;

/// Canister hooks DTOs.
mod canister;
pub use canister::*;

// Http Interface (for metrics)

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HeaderField(pub String, pub String);

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<HeaderField>,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<HeaderField>,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
}
