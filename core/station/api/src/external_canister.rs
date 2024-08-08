use crate::{
    AllowDTO, CanisterInstallMode, PaginationInput, RequestPolicyRuleDTO, Sha256HashDTO,
    TimestampRfc3339, UuidDTO, ValidationMethodResourceTargetDTO,
};
use candid::{CandidType, Deserialize, Nat, Principal};

pub type ExternalCanisterPermissionsInput = ExternalCanisterPermissionsDTO;
pub type ExternalCanisterCallRequestPolicyRuleInput = ExternalCanisterCallRequestPolicyRule;
pub type ExternalCanisterRequestPoliciesInput = ExternalCanisterRequestPoliciesDTO;

// Taken from https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-create_canister
#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
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
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct CreateExternalCanisterOperationKindCreateNewDTO {
    pub initial_cycles: Option<u64>,
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
    pub permissions: ExternalCanisterPermissionsInput,
    pub request_policies: ExternalCanisterRequestPoliciesInput,
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
    pub state: Option<ExternalCanisterStateDTO>,
    pub permissions: Option<ExternalCanisterPermissionsInput>,
    pub request_policies: Option<ExternalCanisterRequestPoliciesInput>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum ConfigureExternalCanisterOperationKindDTO {
    Settings(ConfigureExternalCanisterSettingsInput),
    TopUp(u64),
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
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterCallPermissionDTO {
    pub allow: AllowDTO,
    pub validation_method: ValidationMethodResourceTargetDTO,
    pub execution_method: String,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterCallRequestPolicyRule {
    pub policy_id: Option<UuidDTO>,
    pub rule: RequestPolicyRuleDTO,
    pub validation_method: ValidationMethodResourceTargetDTO,
    pub execution_method: String,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterChangeRequestPolicyRule {
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
    pub change: Vec<ExternalCanisterChangeRequestPolicyRule>,
    pub calls: Vec<ExternalCanisterCallRequestPolicyRule>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterDTO {
    pub id: UuidDTO,
    pub canister_id: Principal,
    pub name: String,
    pub description: Option<String>,
    pub labels: Vec<String>,
    pub state: ExternalCanisterStateDTO,
    pub permissions: ExternalCanisterPermissionsDTO,
    pub request_policies: ExternalCanisterRequestPoliciesDTO,
    pub created_at: TimestampRfc3339,
    pub modified_at: Option<TimestampRfc3339>,
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
    pub can_call: Vec<ExternalCanisterCallerMethodPrivilegesDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct GetExternalCanisterResponse {
    pub canister: ExternalCanisterDTO,
    pub privileges: ExternalCanisterCallerPrivilegesDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ListExternalCanistersInput {
    pub canister_ids: Option<Vec<Principal>>,
    pub labels: Option<Vec<String>>,
    pub paginate: Option<PaginationInput>,
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
    prefix: Option<String>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct GetExternalCanisterFiltersInput {
    with_name: Option<GetExternalCanisterFiltersInputWithName>,
    with_labels: Option<bool>,
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
