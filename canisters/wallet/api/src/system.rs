use super::TimestampRfc3339;
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct SystemInfoDTO {
    pub version: String,
    pub upgrader_id: Principal,
    pub cycles: u64,
    pub last_upgrade_timestamp: TimestampRfc3339,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct SystemInfoResponse {
    pub system: SystemInfoDTO,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct SystemInit {
    pub admins: Option<Vec<Principal>>,
    pub upgrader_wasm_module: Vec<u8>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct SystemUpgrade {}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum SystemInstall {
    Init(SystemInit),
    Upgrade(SystemUpgrade),
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Uninitialized,
}
