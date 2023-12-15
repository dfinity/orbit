use super::{TimestampRfc3339, UserDTO};
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug, Default)]
pub struct WalletSettingsDTO {
    pub owners: Vec<UserDTO>,
    pub last_upgrade_timestamp: TimestampRfc3339,
}

#[derive(CandidType, Deserialize, Clone, Debug, Default)]
pub struct WalletSettingsResponse {
    pub settings: WalletSettingsDTO,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct WalletInit {
    pub owners: Option<Vec<Principal>>,
    pub upgrader_wasm_module: Vec<u8>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct WalletUpgrade {
    pub owners: Option<Vec<Principal>>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum WalletInstall {
    Init(WalletInit),
    Upgrade(WalletUpgrade),
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Uninitialized,
}
