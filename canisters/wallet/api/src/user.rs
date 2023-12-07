use crate::UserGroupDTO;

use super::TimestampRfc3339;
use candid::{CandidType, Deserialize, Principal};

pub type UserIdDTO = String;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum UserRoleDTO {
    Admin = 0,
    User = 1,
    Guest = 2,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum UserStatusDTO {
    Active,
    Inactive,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct UserDTO {
    pub id: UserIdDTO,
    pub identities: Vec<Principal>,
    pub unconfirmed_identities: Vec<Principal>,
    pub groups: Vec<UserGroupDTO>,
    pub status: UserStatusDTO,
    pub name: Option<String>,
    pub last_modification_timestamp: TimestampRfc3339,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetUserInput {
    pub user_id: Option<UserIdDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetUserResponse {
    pub user: UserDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ConfirmUserIdentityInput {
    pub user_id: UserIdDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ConfirmUserIdentityResponse {
    pub user: UserDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AddUserOperationInput {
    pub name: Option<String>,
    pub identities: Vec<Principal>,
    pub unconfirmed_identities: Vec<Principal>,
    pub groups: Vec<String>,
    pub status: UserStatusDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AddUserOperationDTO {
    pub user: Option<UserDTO>,
    pub input: AddUserOperationInput,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditUserOperationInput {
    pub id: UserIdDTO,
    pub name: Option<String>,
    pub identities: Option<Vec<Principal>>,
    pub unconfirmed_identities: Option<Vec<Principal>>,
    pub groups: Option<Vec<String>>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditUserOperationDTO {
    pub input: EditUserOperationInput,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditUserStatusOperationInput {
    pub id: UserIdDTO,
    pub status: UserStatusDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditUserStatusOperationDTO {
    pub input: EditUserStatusOperationInput,
}
