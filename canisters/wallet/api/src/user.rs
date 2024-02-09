use crate::{PaginationInput, UserGroupDTO, UuidDTO};

use super::TimestampRfc3339;
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum UserStatusDTO {
    Active,
    Inactive,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct UserDTO {
    pub id: UuidDTO,
    pub identities: Vec<Principal>,
    pub groups: Vec<UserGroupDTO>,
    pub status: UserStatusDTO,
    pub name: Option<String>,
    pub last_modification_timestamp: TimestampRfc3339,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct BasicUserDTO {
    pub id: UuidDTO,
    pub name: String,
    pub status: UserStatusDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetUserInput {
    pub user_id: UuidDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetUserResponse {
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
    pub id: UuidDTO,
    pub name: Option<String>,
    pub identities: Option<Vec<Principal>>,
    pub groups: Option<Vec<String>>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditUserOperationDTO {
    pub input: EditUserOperationInput,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListUsersInput {
    pub search_term: Option<String>,
    pub statuses: Option<Vec<UserStatusDTO>>,
    pub paginate: Option<PaginationInput>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListUsersResponse {
    pub users: Vec<UserDTO>,
    pub next_offset: Option<u64>,
    pub total: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum UserPrivilege {
    ListAccounts,
    AddAccount,
    ListUsers,
    AddUser,
    ListUserGroups,
    AddUserGroup,
    ListAccessPolicies,
    AddAccessPolicy,
    ListProposalPolicies,
    AddProposalPolicy,
    ListAddressBookEntries,
    AddAddressBookEntry,
    UpgradeCanister,
    ListProposals,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct MeResponse {
    pub me: UserDTO,
    pub privileges: Vec<UserPrivilege>,
}
