use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CanisterInit {
    pub upgrader_wasm_module: Vec<u8>,
    pub wallet_wasm_module: Vec<u8>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CanisterUpgrade {
    pub upgrader_wasm_module: Option<Vec<u8>>,
    pub wallet_wasm_module: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum CanisterInstall {
    Init(CanisterInit),
    Upgrade(CanisterUpgrade),
}
