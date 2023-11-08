use std::borrow::Cow;

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::{storable::Bound, Storable};

#[derive(Clone, CandidType, Deserialize)]
pub struct UpgradeParams {
    pub module: Vec<u8>,
    pub checksum: Vec<u8>,
}

impl Storable for UpgradeParams {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct InitArg {
    #[serde(rename = "targetCanister")]
    pub target_canister: Principal,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum QueueUpgradeError {
    ChecksumMismatch,
    NotController,
    Unauthorized,
    UnexpectedError(String),
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum QueueUpgradeResponse {
    Ok,
    Err(QueueUpgradeError),
}
