use super::TimestampRfc3339;
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct SystemInfoDTO {
    pub name: String,
    pub version: String,
    pub upgrader_id: Principal,
    pub cycles: u64,
    pub last_upgrade_timestamp: TimestampRfc3339,
    pub raw_rand_successful: bool,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ManageSystemInfoOperationDTO {
    pub input: ManageSystemInfoOperationInput,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ManageSystemInfoOperationInput {
    pub name: Option<String>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct SystemInfoResponse {
    pub system: SystemInfoDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct AdminInitInput {
    pub name: String,
    pub identity: Principal,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct SystemInit {
    /// The station name.
    pub name: String,
    /// The initial admins.
    pub admins: Vec<AdminInitInput>,
    /// The number of admin approvals required in initial policies.
    pub quorum: u16,
    /// The upgrader canister wasm module.
    #[serde(with = "serde_bytes")]
    pub upgrader_wasm_module: Vec<u8>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct SystemUpgrade {
    pub name: Option<String>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub enum SystemInstall {
    Init(SystemInit),
    Upgrade(SystemUpgrade),
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Uninitialized,
}
