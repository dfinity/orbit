use crate::{BasicUserDTO, PaginationInput, ResourceDTO, UserGroupDTO, UuidDTO};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AccessPolicyCallerPrivilegesDTO {
    pub resource: ResourceDTO,
    pub can_edit: bool,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AccessPolicyDTO {
    pub allow: AllowDTO,
    pub resource: ResourceDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AllowDTO {
    pub auth_scope: AuthScopeDTO,
    pub users: Vec<UuidDTO>,
    pub user_groups: Vec<UuidDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum AuthScopeDTO {
    Public = 1,
    Authenticated = 2,
    Restricted = 3,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListAccessPoliciesInput {
    pub resources: Option<Vec<ResourceDTO>>,
    pub paginate: Option<PaginationInput>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListAccessPoliciesResponse {
    pub policies: Vec<AccessPolicyDTO>,
    pub user_groups: Vec<UserGroupDTO>,
    pub users: Vec<BasicUserDTO>,
    pub next_offset: Option<u64>,
    pub total: u64,
    pub privileges: Vec<AccessPolicyCallerPrivilegesDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetAccessPolicyInput {
    pub resource: ResourceDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetAccessPolicyResponse {
    pub policy: AccessPolicyDTO,
    pub privileges: AccessPolicyCallerPrivilegesDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditAccessPolicyOperationDTO {
    pub input: EditAccessPolicyOperationInput,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditAccessPolicyOperationInput {
    pub resource: ResourceDTO,
    pub auth_scope: Option<AuthScopeDTO>,
    pub users: Option<Vec<UuidDTO>>,
    pub user_groups: Option<Vec<UuidDTO>>,
}
