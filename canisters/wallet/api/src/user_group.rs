use crate::{PaginationInput, UuidDTO};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UserGroupDTO {
    pub id: UuidDTO,
    pub name: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AddUserGroupOperationInput {
    pub name: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AddUserGroupOperationDTO {
    pub user_group: Option<UserGroupDTO>,
    pub input: AddUserGroupOperationInput,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct EditUserGroupOperationInput {
    pub user_group_id: UuidDTO,
    pub name: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct EditUserGroupOperationDTO {
    pub input: EditUserGroupOperationInput,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct RemoveUserGroupOperationInput {
    pub user_group_id: UuidDTO,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct RemoveUserGroupOperationDTO {
    pub input: RemoveUserGroupOperationInput,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetUserGroupInput {
    pub user_group_id: UuidDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetUserGroupResponse {
    pub user_group: UserGroupDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListUserGroupsInput {
    pub search_term: Option<String>,
    pub paginate: Option<PaginationInput>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListUserGroupsResponse {
    pub user_groups: Vec<UserGroupDTO>,
    pub next_offset: Option<u64>,
    pub total: u64,
}
