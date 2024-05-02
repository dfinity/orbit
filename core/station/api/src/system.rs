use super::TimestampRfc3339;
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct SystemInfoDTO {
    pub name: String,
    pub version: String,
    pub upgrader_id: Principal,
    pub cycles: u64,
    pub last_upgrade_timestamp: TimestampRfc3339,
    pub raw_rand_successful: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct SystemInfoResponse {
    pub system: SystemInfoDTO,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AdminInitInput {
    pub name: String,
    pub identity: Principal,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct SystemInit {
    pub name: String,
    pub admins: Vec<AdminInitInput>,
    #[serde(with = "serde_bytes")]
    pub upgrader_wasm_module: Vec<u8>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct SystemUpgrade {
    pub name: Option<String>,
}

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
