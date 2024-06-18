use candid::{CandidType, Deserialize, Principal};

#[derive(Clone, Debug, CandidType, serde::Serialize, Deserialize, PartialEq)]
pub struct UpgradeParams {
    #[serde(with = "serde_bytes")]
    pub module: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub arg: Vec<u8>,
}

#[derive(Clone, Debug, CandidType, serde::Serialize, Deserialize)]
pub struct InitArg {
    pub target_canister: Principal,
}

#[derive(Clone, Debug, CandidType, serde::Serialize, Deserialize)]
pub enum TriggerUpgradeError {
    NotController,
    Unauthorized,
    UnexpectedError(String),
}

#[derive(Clone, Debug, CandidType, serde::Serialize, Deserialize)]
pub enum TriggerUpgradeResponse {
    Ok,
    Err(TriggerUpgradeError),
}
