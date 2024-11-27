use crate::{
    resource::ResourceDTO, CallExternalCanisterResourceTargetDTO, ExternalCanisterIdDTO,
    MetadataDTO, PaginationInput, ResourceIdsDTO, UuidDTO,
};
use candid::{CandidType, Deserialize};

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum RequestSpecifierDTO {
    AddAccount,
    AddUser,
    EditAccount(ResourceIdsDTO),
    EditUser(ResourceIdsDTO),
    AddAddressBookEntry,
    EditAddressBookEntry(ResourceIdsDTO),
    RemoveAddressBookEntry(ResourceIdsDTO),
    Transfer(ResourceIdsDTO),
    SystemUpgrade,
    SetDisasterRecovery,
    ChangeExternalCanister(ExternalCanisterIdDTO),
    FundExternalCanister(ExternalCanisterIdDTO),
    CreateExternalCanister,
    CallExternalCanister(CallExternalCanisterResourceTargetDTO),
    EditPermission(ResourceSpecifierDTO),
    AddRequestPolicy,
    EditRequestPolicy(ResourceIdsDTO),
    RemoveRequestPolicy(ResourceIdsDTO),
    AddUserGroup,
    EditUserGroup(ResourceIdsDTO),
    RemoveUserGroup(ResourceIdsDTO),
    ManageSystemInfo,
    AddAsset,
    EditAsset(ResourceIdsDTO),
    RemoveAsset(ResourceIdsDTO),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum UserSpecifierDTO {
    Any,
    Group(Vec<UuidDTO>),
    Id(Vec<UuidDTO>),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum ResourceSpecifierDTO {
    Any,
    Resource(ResourceDTO),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct QuorumPercentageDTO {
    pub approvers: UserSpecifierDTO,
    pub min_approved: u16,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct QuorumDTO {
    pub approvers: UserSpecifierDTO,
    pub min_approved: u16,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum RequestPolicyRuleInput {
    Remove,
    Set(RequestPolicyRuleDTO),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum EvaluationStatusDTO {
    Approved,
    Rejected,
    Pending,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum RequestPolicyRuleDTO {
    AutoApproved,
    QuorumPercentage(QuorumPercentageDTO),
    Quorum(QuorumDTO),
    AllowListedByMetadata(MetadataDTO),
    AllowListed,
    AnyOf(Vec<RequestPolicyRuleDTO>),
    AllOf(Vec<RequestPolicyRuleDTO>),
    Not(Box<RequestPolicyRuleDTO>),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum EvaluatedRequestPolicyRuleDTO {
    AutoApproved,
    QuorumPercentage {
        total_possible_approvers: usize,
        min_approved: usize,
        approvers: Vec<UuidDTO>,
    },
    Quorum {
        total_possible_approvers: usize,
        min_approved: usize,
        approvers: Vec<UuidDTO>,
    },
    AllowListedByMetadata {
        metadata: MetadataDTO,
    },
    AllowListed,
    AnyOf(Vec<RequestPolicyRuleResultDTO>),
    AllOf(Vec<RequestPolicyRuleResultDTO>),
    Not(Box<RequestPolicyRuleResultDTO>),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct RequestPolicyRuleResultDTO {
    pub status: EvaluationStatusDTO,
    pub evaluated_rule: EvaluatedRequestPolicyRuleDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub enum EvaluationSummaryReasonDTO {
    ApprovalQuorum,
    AllowList,
    AllowListMetadata,
    AutoApproved,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct RequestEvaluationResultDTO {
    pub request_id: UuidDTO,
    pub status: EvaluationStatusDTO,
    pub policy_results: Vec<RequestPolicyRuleResultDTO>,
    pub result_reasons: Option<Vec<EvaluationSummaryReasonDTO>>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct RequestPolicyCallerPrivilegesDTO {
    pub id: UuidDTO,
    pub can_edit: bool,
    pub can_delete: bool,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct RequestPolicyDTO {
    pub id: UuidDTO,
    pub specifier: RequestSpecifierDTO,
    pub rule: RequestPolicyRuleDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct GetRequestPolicyInput {
    pub id: UuidDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct GetRequestPolicyResponse {
    pub policy: RequestPolicyDTO,
    pub privileges: RequestPolicyCallerPrivilegesDTO,
}

pub type ListRequestPoliciesInput = PaginationInput;

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ListRequestPoliciesResponse {
    pub policies: Vec<RequestPolicyDTO>,
    pub next_offset: Option<u64>,
    pub total: u64,
    pub privileges: Vec<RequestPolicyCallerPrivilegesDTO>,
}
