use crate::{BasicUserDTO, PaginationInput, ResourceDTO, UserGroupDTO, UuidDTO};
use candid::{CandidType, Deserialize};

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct PermissionCallerPrivilegesDTO {
    pub resource: ResourceDTO,
    pub can_edit: bool,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct PermissionDTO {
    pub allow: AllowDTO,
    pub resource: ResourceDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct AllowDTO {
    pub auth_scope: AuthScopeDTO,
    pub users: Vec<UuidDTO>,
    pub user_groups: Vec<UuidDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum AuthScopeDTO {
    Public = 1,
    Authenticated = 2,
    Restricted = 3,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ListPermissionsInput {
    pub resources: Option<Vec<ResourceDTO>>,
    pub paginate: Option<PaginationInput>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ListPermissionsResponse {
    pub permissions: Vec<PermissionDTO>,
    pub user_groups: Vec<UserGroupDTO>,
    pub users: Vec<BasicUserDTO>,
    pub next_offset: Option<u64>,
    pub total: u64,
    pub privileges: Vec<PermissionCallerPrivilegesDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct GetPermissionInput {
    pub resource: ResourceDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct GetPermissionResponse {
    pub permission: PermissionDTO,
    pub privileges: PermissionCallerPrivilegesDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct EditPermissionOperationDTO {
    pub input: EditPermissionOperationInput,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct EditPermissionOperationInput {
    pub resource: ResourceDTO,
    pub auth_scope: Option<AuthScopeDTO>,
    pub users: Option<Vec<UuidDTO>>,
    pub user_groups: Option<Vec<UuidDTO>>,
}
