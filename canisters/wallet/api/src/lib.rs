//! Transport layer that defines the data transfer objects (DTOs) that are used to communicate
//! with the clients of the wallet canister.

mod wallet_details;
use candid::{CandidType, Deserialize};
pub use wallet_details::*;

mod address_book;
pub use address_book::*;

mod common;
pub use common::*;

mod management;
pub use management::*;

mod metadata;
pub use metadata::*;

mod notification;
pub use notification::*;

mod account;
pub use account::*;

mod transfer;
pub use transfer::*;

mod proposal;
pub use proposal::*;

mod user;
pub use user::*;

mod user_group;
pub use user_group::*;

mod change_canister;
pub use change_canister::*;

mod proposal_policy;
pub use proposal_policy::*;

mod access_policy;
pub use access_policy::*;

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
