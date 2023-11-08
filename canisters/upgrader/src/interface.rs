use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_canister_macros::stable_object;

#[stable_object]
#[derive(Clone, Debug, CandidType, Deserialize, PartialEq)]
pub struct UpgradeParams {
    pub module: Vec<u8>,
    pub checksum: Vec<u8>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct InitArg {
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
