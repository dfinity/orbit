use crate::{
    AllowDTO, CanisterInstallMode, ChangeMetadataDTO, CycleObtainStrategyInput, MetadataDTO,
    PaginationInput, RequestPolicyRuleDTO, Sha256HashDTO, SortDirection, TimestampRfc3339, UuidDTO,
    ValidationMethodResourceTargetDTO,
};
use candid::{CandidType, Deserialize, Nat, Principal};
use orbit_essentials::cmc::SubnetSelection;
use orbit_essentials::types::WasmModuleExtraChunks;

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct CanisterSnapshotsInput {
    pub canister_id: Principal,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct CanisterStatusInput {
    pub canister_id: Principal,
}

// Taken from https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-create_canister
#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone, Default)]
pub enum LogVisibility {
    #[serde(rename = "public")]
    Public,
    #[default]
    #[serde(rename = "controllers")]
    Controllers,
    #[serde(rename = "allowed_viewers")]
    AllowedViewers(Vec<Principal>),
}
#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone, Default)]
pub struct DefiniteCanisterSettingsInput {
    /// Controllers of the canister.
    pub controllers: Option<Vec<Principal>>,
    /// Compute allocation.
    pub compute_allocation: Option<Nat>,
    /// Memory allocation.
    pub memory_allocation: Option<Nat>,
    /// Freezing threshold.
    pub freezing_threshold: Option<Nat>,
    /// Reserved cycles limit.
    pub reserved_cycles_limit: Option<Nat>,
    /// Log visibility.
    pub log_visibility: Option<LogVisibility>,
    /// Wasm memory limit.
    pub wasm_memory_limit: Option<Nat>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct CreateExternalCanisterOperationKindCreateNewDTO {
    pub initial_cycles: Option<u64>,
    pub subnet_selection: Option<SubnetSelection>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct CreateExternalCanisterOperationKindAddExistingDTO {
    pub canister_id: Principal,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum CreateExternalCanisterOperationKindDTO {
    CreateNew(CreateExternalCanisterOperationKindCreateNewDTO),
    AddExisting(CreateExternalCanisterOperationKindAddExistingDTO),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct CreateExternalCanisterOperationInput {
    pub kind: CreateExternalCanisterOperationKindDTO,
    pub name: String,
    pub description: Option<String>,
    pub labels: Option<Vec<String>>,
    pub metadata: Option<Vec<MetadataDTO>>,
    pub permissions: ExternalCanisterPermissionsCreateInput,
    pub request_policies: ExternalCanisterRequestPoliciesCreateInput,
}

pub type ExternalCanisterPermissionsCreateInput = ExternalCanisterPermissionsDTO;

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterPermissionsUpdateInput {
    pub read: Option<AllowDTO>,
    pub change: Option<AllowDTO>,
    pub calls: Option<ExternalCanisterChangeCallPermissionsInput>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct CanisterExecutionAndValidationMethodPairDTO {
    pub validation_method: ValidationMethodResourceTargetDTO,
    pub execution_method: String,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterCallPermissionMethodPairInput {
    pub method_configuration: CanisterExecutionAndValidationMethodPairDTO,
    pub allow: Option<AllowDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterCallPermissionExecMethodEntryInput {
    pub allow: AllowDTO,
    pub validation_method: ValidationMethodResourceTargetDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterCallPermissionsExecMethodInput {
    pub execution_method: String,
    pub permissions: Vec<ExternalCanisterCallPermissionExecMethodEntryInput>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum ExternalCanisterChangeCallPermissionsInput {
    ReplaceAllBy(Vec<ExternalCanisterCallPermissionDTO>),
    OverrideSpecifiedByExecutionMethods(Vec<ExternalCanisterCallPermissionsExecMethodInput>),
    OverrideSpecifiedByExecutionValidationMethodPairs(
        Vec<ExternalCanisterCallPermissionMethodPairInput>,
    ),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct CreateExternalCanisterOperationDTO {
    pub canister_id: Option<Principal>,
    pub input: CreateExternalCanisterOperationInput,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ChangeExternalCanisterOperationInput {
    pub canister_id: Principal,
    pub mode: CanisterInstallMode,
    #[serde(with = "serde_bytes")]
    pub module: Vec<u8>,
    pub module_extra_chunks: Option<WasmModuleExtraChunks>,
    #[serde(deserialize_with = "orbit_essentials::deserialize::deserialize_option_blob")]
    pub arg: Option<Vec<u8>>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ChangeExternalCanisterOperationDTO {
    pub canister_id: Principal,
    pub mode: CanisterInstallMode,
    pub module_checksum: Sha256HashDTO,
    pub arg_checksum: Option<Sha256HashDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ConfigureExternalCanisterOperationInput {
    pub canister_id: Principal,
    pub kind: ConfigureExternalCanisterOperationKindDTO,
}

pub type ConfigureExternalCanisterOperationDTO = ConfigureExternalCanisterOperationInput;

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ConfigureExternalCanisterSettingsInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub labels: Option<Vec<String>>,
    pub change_metadata: Option<ChangeMetadataDTO>,
    pub state: Option<ExternalCanisterStateDTO>,
    pub permissions: Option<ExternalCanisterPermissionsUpdateInput>,
    pub request_policies: Option<ExternalCanisterRequestPoliciesUpdateInput>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum ConfigureExternalCanisterOperationKindDTO {
    Settings(ConfigureExternalCanisterSettingsInput),
    SoftDelete,
    Delete,
    NativeSettings(DefiniteCanisterSettingsInput),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct CanisterMethodDTO {
    pub canister_id: Principal,
    pub method_name: String,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct CallExternalCanisterOperationInput {
    pub validation_method: Option<CanisterMethodDTO>,
    pub execution_method: CanisterMethodDTO,
    #[serde(deserialize_with = "orbit_essentials::deserialize::deserialize_option_blob")]
    pub arg: Option<Vec<u8>>,
    pub execution_method_cycles: Option<u64>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct CallExternalCanisterOperationDTO {
    pub validation_method: Option<CanisterMethodDTO>,
    pub execution_method: CanisterMethodDTO,
    pub arg_checksum: Option<Sha256HashDTO>,
    pub arg_rendering: Option<String>,
    pub execution_method_cycles: Option<u64>,
    pub execution_method_reply: Option<Vec<u8>>,
    #[serde(deserialize_with = "orbit_essentials::deserialize::deserialize_option_blob")]
    pub arg: Option<Vec<u8>>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterCallPermissionDTO {
    pub allow: AllowDTO,
    pub validation_method: ValidationMethodResourceTargetDTO,
    pub execution_method: String,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterCallRequestPolicyRuleDTO {
    pub policy_id: UuidDTO,
    pub rule: RequestPolicyRuleDTO,
    pub validation_method: ValidationMethodResourceTargetDTO,
    pub execution_method: String,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterCallRequestPolicyRuleInput {
    pub policy_id: Option<UuidDTO>,
    pub rule: RequestPolicyRuleDTO,
    pub validation_method: ValidationMethodResourceTargetDTO,
    pub execution_method: String,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterChangeRequestPolicyRuleDTO {
    pub policy_id: UuidDTO,
    pub rule: RequestPolicyRuleDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterChangeRequestPolicyRuleInput {
    pub policy_id: Option<UuidDTO>,
    pub rule: RequestPolicyRuleDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterPermissionsDTO {
    pub read: AllowDTO,
    pub change: AllowDTO,
    pub calls: Vec<ExternalCanisterCallPermissionDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterRequestPoliciesDTO {
    pub change: Vec<ExternalCanisterChangeRequestPolicyRuleDTO>,
    pub calls: Vec<ExternalCanisterCallRequestPolicyRuleDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterRequestPoliciesCreateInput {
    pub change: Vec<ExternalCanisterChangeRequestPolicyRuleInput>,
    pub calls: Vec<ExternalCanisterCallRequestPolicyRuleInput>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterRequestPoliciesUpdateInput {
    pub change: Option<Vec<ExternalCanisterChangeRequestPolicyRuleInput>>,
    pub calls: Option<ExternalCanisterChangeCallRequestPoliciesInput>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterCallRequestPoliciesMethodPairInput {
    pub method_configuration: CanisterExecutionAndValidationMethodPairDTO,
    pub policies: Vec<ExternalCanisterChangeRequestPolicyRuleInput>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterCallRequestPolicyRuleValidationInput {
    pub policy_id: Option<UuidDTO>,
    pub rule: RequestPolicyRuleDTO,
    pub validation_method: ValidationMethodResourceTargetDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterCallRequestPoliciesExecMethodInput {
    pub execution_method: String,
    pub policies: Vec<ExternalCanisterCallRequestPolicyRuleValidationInput>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum ExternalCanisterChangeCallRequestPoliciesInput {
    ReplaceAllBy(Vec<ExternalCanisterCallRequestPolicyRuleInput>),
    RemoveByPolicyIds(Vec<UuidDTO>),
    OverrideSpecifiedByExecutionMethods(Vec<ExternalCanisterCallRequestPoliciesExecMethodInput>),
    OverrideSpecifiedByExecutionValidationMethodPairs(
        Vec<ExternalCanisterCallRequestPoliciesMethodPairInput>,
    ),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterDTO {
    pub id: UuidDTO,
    pub canister_id: Principal,
    pub name: String,
    pub description: Option<String>,
    pub labels: Vec<String>,
    pub metadata: Vec<MetadataDTO>,
    pub state: ExternalCanisterStateDTO,
    pub permissions: ExternalCanisterPermissionsDTO,
    pub request_policies: ExternalCanisterRequestPoliciesDTO,
    pub created_at: TimestampRfc3339,
    pub modified_at: Option<TimestampRfc3339>,
    pub monitoring: Option<MonitorExternalCanisterStartInput>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum ExternalCanisterStateDTO {
    Active,
    Archived,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct GetExternalCanisterInput {
    pub canister_id: Principal,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterCallerMethodPrivilegesDTO {
    pub validation_method: ValidationMethodResourceTargetDTO,
    pub execution_method: String,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterCallerPrivilegesDTO {
    pub id: UuidDTO,
    pub canister_id: Principal,
    pub can_change: bool,
    pub can_fund: bool,
    pub can_call: Vec<ExternalCanisterCallerMethodPrivilegesDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct GetExternalCanisterResponse {
    pub canister: ExternalCanisterDTO,
    pub privileges: ExternalCanisterCallerPrivilegesDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum ListExternalCanistersSortInput {
    Name(SortDirection),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ListExternalCanistersInput {
    pub canister_ids: Option<Vec<Principal>>,
    pub labels: Option<Vec<String>>,
    pub states: Option<Vec<ExternalCanisterStateDTO>>,
    pub paginate: Option<PaginationInput>,
    pub sort_by: Option<ListExternalCanistersSortInput>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ListExternalCanistersResponse {
    pub canisters: Vec<ExternalCanisterDTO>,
    pub next_offset: Option<u64>,
    pub total: u64,
    pub privileges: Vec<ExternalCanisterCallerPrivilegesDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct GetExternalCanisterFiltersInputWithName {
    pub prefix: Option<String>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct GetExternalCanisterFiltersInput {
    pub with_name: Option<GetExternalCanisterFiltersInputWithName>,
    pub with_labels: Option<bool>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct GetExternalCanisterFiltersResponseNameEntry {
    pub name: String,
    pub canister_id: Principal,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct GetExternalCanisterFiltersResponse {
    pub names: Option<Vec<GetExternalCanisterFiltersResponseNameEntry>>,
    pub labels: Option<Vec<String>>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct FundExternalCanisterSendCyclesInput {
    pub cycles: u64,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum FundExternalCanisterOperationKindDTO {
    Send(FundExternalCanisterSendCyclesInput),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct FundExternalCanisterOperationInput {
    pub canister_id: Principal,
    pub kind: FundExternalCanisterOperationKindDTO,
}

pub type FundExternalCanisterOperationDTO = FundExternalCanisterOperationInput;

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct MonitoringExternalCanisterEstimatedRuntimeInput {
    pub fund_runtime_secs: u64,
    pub fallback_min_cycles: u128,
    pub min_runtime_secs: u64,
    pub fallback_fund_cycles: u128,
    pub max_runtime_cycles_fund: u128,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct MonitoringExternalCanisterCyclesThresholdInput {
    pub fund_cycles: u128,
    pub min_cycles: u128,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum MonitorExternalCanisterStrategyDTO {
    Always(u128),
    BelowThreshold(MonitoringExternalCanisterCyclesThresholdInput),
    BelowEstimatedRuntime(MonitoringExternalCanisterEstimatedRuntimeInput),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct MonitorExternalCanisterStartInput {
    pub funding_strategy: MonitorExternalCanisterStrategyDTO,
    pub cycle_obtain_strategy: Option<CycleObtainStrategyInput>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum MonitorExternalCanisterOperationKindDTO {
    Start(MonitorExternalCanisterStartInput),
    Stop,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct MonitorExternalCanisterOperationInput {
    pub canister_id: Principal,
    pub kind: MonitorExternalCanisterOperationKindDTO,
}

pub type MonitorExternalCanisterOperationDTO = MonitorExternalCanisterOperationInput;

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct SnapshotExternalCanisterOperationInput {
    pub canister_id: Principal,
    pub replace_snapshot: Option<String>,
    pub force: bool,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct SnapshotExternalCanisterOperationDTO {
    pub snapshot_id: Option<String>,
    pub input: SnapshotExternalCanisterOperationInput,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct RestoreExternalCanisterOperationInput {
    pub canister_id: Principal,
    pub snapshot_id: String,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct RestoreExternalCanisterOperationDTO {
    pub input: RestoreExternalCanisterOperationInput,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum PruneExternalCanisterResourceDTO {
    #[serde(rename = "snapshot")]
    Snapshot(String),
    #[serde(rename = "chunk_store")]
    ChunkStore,
    #[serde(rename = "state")]
    State,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct PruneExternalCanisterOperationInput {
    pub canister_id: Principal,
    pub prune: PruneExternalCanisterResourceDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct PruneExternalCanisterOperationDTO {
    pub input: PruneExternalCanisterOperationInput,
}
