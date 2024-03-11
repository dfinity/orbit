use crate::{BasicUserDTO, PaginationInput, UserGroupDTO, UuidDTO};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AccessPolicyCallerPrivilegesDTO {
    pub policy_id: UuidDTO,
    pub can_edit: bool,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AccessPolicyDTO {
    pub id: UuidDTO,
    pub allow: AllowDTO,
    pub resource: ResourceDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum AllowDTO {
    Any,
    Authenticated,
    Users(Vec<UuidDTO>),
    UserGroups(Vec<UuidDTO>),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum AllowKeyDTO {
    Any = 1,
    Authenticated = 2,
    Users = 3,
    UserGroups = 4,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ResourceDTO {
    AccessPolicy(AccessPolicyResourceActionDTO),
    Account(AccountResourceActionDTO),
    AddressBook(ResourceActionDTO),
    ChangeCanister(ChangeCanisterResourceActionDTO),
    Proposal(ProposalResourceActionDTO),
    ProposalPolicy(ResourceActionDTO),
    Settings(SettingsResourceActionDTO),
    User(UserResourceActionDTO),
    UserGroup(ResourceActionDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ResourceIdDTO {
    Any,
    Id(UuidDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ResourceActionDTO {
    List,
    Create,
    Read(ResourceIdDTO),
    Update(ResourceIdDTO),
    Delete(ResourceIdDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum AccessPolicyResourceActionDTO {
    List,
    Read(ResourceIdDTO),
    Edit(ResourceIdDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum UserResourceActionDTO {
    List,
    Create,
    Read(ResourceIdDTO),
    Update(ResourceIdDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum AccountResourceActionDTO {
    List,
    Create,
    Transfer(ResourceIdDTO),
    Read(ResourceIdDTO),
    Update(ResourceIdDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum SettingsResourceActionDTO {
    Read,
    ReadConfig,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ChangeCanisterResourceActionDTO {
    Create,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProposalResourceActionDTO {
    List,
    Read(ResourceIdDTO),
}

pub type ListAccessPoliciesInput = PaginationInput;

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
    pub id: UuidDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetAccessPolicyResponse {
    pub policy: AccessPolicyDTO,
    pub privileges: AccessPolicyCallerPrivilegesDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditAccessPolicyOperationDTO {
    pub policy_id: Option<UuidDTO>,
    pub input: EditAccessPolicyOperationInput,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ResourceAccessDTO {
    Deny(AllowKeyDTO),
    Allow(AllowDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditAccessPolicyOperationInput {
    pub resource: ResourceDTO,
    pub access: ResourceAccessDTO,
}
