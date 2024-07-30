use crate::{
    AllowDTO, PaginationInput, RequestPolicyRuleDTO, Sha256HashDTO, TimestampRfc3339, UuidDTO,
    ValidationMethodResourceTargetDTO,
};
use candid::{CandidType, Deserialize, Principal};

pub type ExternalCanisterPermissionsInput = ExternalCanisterPermissionsDTO;
pub type ExternalCanisterCallRequestPolicyRuleInput = ExternalCanisterCallRequestPolicyRule;

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct CreateExternalCanisterOperationInput {
    existing_canister_id: Option<Principal>,
    initial_cycles: Option<u64>,
    name: String,
    description: Option<String>,
    labels: Option<Vec<String>>,
    permissions: ExternalCanisterPermissionsInput,
    request_policies: Vec<ExternalCanisterCallRequestPolicyRuleInput>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct CreateExternalCanisterOperationDTO {
    pub canister_id: Option<Principal>,
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
    pub rule: RequestPolicyRuleDTO,
    pub validation_method: ValidationMethodResourceTargetDTO,
    pub execution_method: String,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterPermissionsDTO {
    pub read: AllowDTO,
    pub change: AllowDTO,
    pub calls: Vec<ExternalCanisterCallPermissionDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterDTO {
    pub id: UuidDTO,
    pub canister_id: Principal,
    pub name: String,
    pub description: Option<String>,
    pub labels: Vec<String>,
    pub permissions: ExternalCanisterPermissionsDTO,
    pub request_policies: Vec<ExternalCanisterCallRequestPolicyRule>,
    pub created_at: TimestampRfc3339,
    pub modified_at: Option<TimestampRfc3339>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum GetExternalCanisterInput {
    Id(UuidDTO),
    CanisterId(Principal),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterCallerMethodPrivilegesDTO {
    pub validation_method: ValidationMethodResourceTargetDTO,
    pub execution_method: String,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCanisterCallerPrivilegesDTO {
    pub id: UuidDTO,
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
    pub names: Option<Vec<String>>,
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
pub struct GetExternalCanisterFiltersResponse {
    pub names: Vec<String>,
    pub labels: Vec<String>,
}
