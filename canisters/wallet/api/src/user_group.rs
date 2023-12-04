use crate::UuidDTO;
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
