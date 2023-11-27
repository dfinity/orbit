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
    pub access_roles: Vec<UserRoleDTO>,
    pub last_modification_timestamp: TimestampRfc3339,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct RegisterUserInput {
    pub identities: Vec<Principal>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct RegisterUserResponse {
    pub user: UserDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditUserInput {
    pub user_id: UserIdDTO,
    pub identities: Option<Vec<Principal>>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditUserResponse {
    pub user: UserDTO,
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
    pub groups: Option<Vec<String>>,
    pub status: Option<UserStatusDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditUserOperationDTO {
    pub input: EditUserOperationInput,
}
