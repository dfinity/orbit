use crate::{DisasterRecoveryCommitteeDTO, MetadataDTO, UuidDTO};

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
    pub disaster_recovery: Option<DisasterRecoveryDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct DisasterRecoveryDTO {
    pub committee: DisasterRecoveryCommitteeDTO,
    pub user_group_name: Option<String>,
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
pub enum SystemUpgraderInput {
    Id(Principal),
    WasmModule(#[serde(with = "serde_bytes")] Vec<u8>),
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct InitAccountInput {
    pub id: Option<UuidDTO>,
    pub name: String,
    pub blockchain: String,
    pub standard: String,
    pub metadata: Vec<MetadataDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct SystemInit {
    /// The station name.
    pub name: String,
    /// The initial admins.
    pub admins: Vec<AdminInitInput>,
    /// The quorum of admin approvals required in initial policies.
    pub quorum: Option<u16>,
    /// The upgrader configuration.
    pub upgrader: SystemUpgraderInput,
    /// Optional fallback controller for the station and upgrader canisters.
    pub fallback_controller: Option<Principal>,
    /// Optionally set the initial accounts.
    pub accounts: Option<Vec<InitAccountInput>>,
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
