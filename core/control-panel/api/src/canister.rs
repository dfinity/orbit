use candid::{CandidType, Deserialize};

pub type TimestampRfc3339 = String;

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CanisterInit {
    #[serde(with = "serde_bytes")]
    pub upgrader_wasm_module: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub station_wasm_module: Vec<u8>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CanisterUpgrade {
    #[serde(deserialize_with = "orbit_essentials::deserialize::deserialize_option_blob")]
    pub upgrader_wasm_module: Option<Vec<u8>>,
    #[serde(deserialize_with = "orbit_essentials::deserialize::deserialize_option_blob")]
    pub station_wasm_module: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum CanisterInstall {
    Init(CanisterInit),
    Upgrade(CanisterUpgrade),
}
