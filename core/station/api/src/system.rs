use super::TimestampRfc3339;
use crate::{
    AccountSeedDTO, AllowDTO, DisasterRecoveryCommitteeDTO, MetadataDTO, RequestPolicyRuleDTO,
    RequestSpecifierDTO, ResourceDTO, Sha256HashDTO, UserStatusDTO, UuidDTO,
};
use candid::{CandidType, Deserialize, Principal};
use orbit_essentials::types::WasmModuleExtraChunks;

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct SystemInfoDTO {
    pub name: String,
    pub version: String,
    pub upgrader_id: Principal,
    pub cycles: u64,
    pub upgrader_cycles: Option<u64>,
    pub last_upgrade_timestamp: TimestampRfc3339,
    pub raw_rand_successful: bool,
    pub disaster_recovery: Option<DisasterRecoveryDTO>,
    pub cycle_obtain_strategy: CycleObtainStrategyDTO,
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
pub enum CycleObtainStrategyDTO {
    Disabled,
    MintFromNativeToken {
        account_id: UuidDTO,
        account_name: Option<String>,
    },
    WithdrawFromCyclesLedger {
        account_id: UuidDTO,
        account_name: Option<String>,
    },
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum CycleObtainStrategyInput {
    Disabled,
    MintFromNativeToken { account_id: UuidDTO },
    WithdrawFromCyclesLedger { account_id: UuidDTO },
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ManageSystemInfoOperationInput {
    pub name: Option<String>,
    pub cycle_obtain_strategy: Option<CycleObtainStrategyInput>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct SystemInfoResponse {
    pub system: SystemInfoDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct UserInitInput {
    pub id: Option<UuidDTO>,
    pub name: String,
    pub identities: Vec<UserIdentityInput>,
    pub groups: Option<Vec<UuidDTO>>,
    pub status: Option<UserStatusDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct UserIdentityInput {
    pub identity: Principal,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct InitUserGroupInput {
    pub id: Option<UuidDTO>,
    pub name: String,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct InitPermissionInput {
    pub resource: ResourceDTO,
    pub allow: AllowDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct InitRequestPolicyInput {
    pub id: Option<UuidDTO>,
    pub specifier: RequestSpecifierDTO,
    pub rule: RequestPolicyRuleDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct InitNamedRuleInput {
    pub id: Option<UuidDTO>,
    pub name: String,
    pub description: Option<String>,
    pub rule: RequestPolicyRuleDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct DeploySystemUpgraderInput {
    #[serde(with = "serde_bytes")]
    pub wasm_module: Vec<u8>,
    pub initial_cycles: Option<u128>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub enum SystemUpgraderInput {
    Id(Principal),
    Deploy(DeploySystemUpgraderInput),
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct InitAccountInput {
    pub id: Option<UuidDTO>,
    pub name: String,
    pub seed: AccountSeedDTO,
    pub assets: Vec<UuidDTO>,
    pub metadata: Vec<MetadataDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct InitAccountPermissionsInput {
    pub read_permission: AllowDTO,
    pub configs_permission: AllowDTO,
    pub transfer_permission: AllowDTO,
    pub configs_request_policy: Option<RequestPolicyRuleDTO>,
    pub transfer_request_policy: Option<RequestPolicyRuleDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct InitAccountWithPermissionsInput {
    pub account_init: InitAccountInput,
    pub permissions: InitAccountPermissionsInput,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct InitAssetInput {
    pub id: Option<UuidDTO>,
    pub name: String,
    pub blockchain: String,
    pub standards: Vec<String>,
    pub metadata: Vec<MetadataDTO>,
    pub symbol: String,
    pub decimals: u32,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub enum InitialEntries {
    WithDefaultPolicies {
        assets: Vec<InitAssetInput>,
        accounts: Vec<InitAccountInput>,
    },
    Complete {
        permissions: Vec<InitPermissionInput>,
        assets: Vec<InitAssetInput>,
        request_policies: Vec<InitRequestPolicyInput>,
        user_groups: Vec<InitUserGroupInput>,
        accounts: Vec<InitAccountWithPermissionsInput>,
        named_rules: Vec<InitNamedRuleInput>,
    },
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct SystemInit {
    /// The station name.
    pub name: String,
    /// The upgrader configuration.
    pub upgrader: SystemUpgraderInput,
    /// Optional fallback controller for the station and upgrader canisters.
    pub fallback_controller: Option<Principal>,
    /// The initial users.
    pub users: Vec<UserInitInput>,
    /// The initial quorum.
    pub quorum: Option<u16>,
    /// The initial database entries.
    pub entries: Option<InitialEntries>,
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

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum SystemUpgradeTargetDTO {
    UpgradeStation,
    UpgradeUpgrader,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct SystemUpgradeOperationInput {
    pub target: SystemUpgradeTargetDTO,
    #[serde(with = "serde_bytes")]
    pub module: Vec<u8>,
    pub module_extra_chunks: Option<WasmModuleExtraChunks>,
    #[serde(deserialize_with = "orbit_essentials::deserialize::deserialize_option_blob")]
    pub arg: Option<Vec<u8>>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct SystemUpgradeOperationDTO {
    pub target: SystemUpgradeTargetDTO,
    pub module_checksum: Sha256HashDTO,
    pub arg_checksum: Option<Sha256HashDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct NotifyFailedStationUpgradeInput {
    pub reason: String,
}
