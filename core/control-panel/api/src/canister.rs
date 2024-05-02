use candid::{CandidType, Deserialize};

pub type TimestampRfc3339 = String;

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UploadCanisterModulesInput {
    #[serde(with = "serde_bytes")]
    pub upgrader_wasm_module: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub station_wasm_module: Vec<u8>,
}
