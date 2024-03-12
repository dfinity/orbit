use crate::{BasicUserDTO, PaginationInput, UserGroupDTO, UuidDTO};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AccessPolicyCallerPrivilegesDTO {
    pub can_edit: bool,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AccessPolicyDTO {
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
pub enum AllowLevelDTO {
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
pub enum ResourceTypeIdDTO {
    Any,
    Resource(ResourceTypeDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ResourceTypeDTO {
    AccessPolicy(AccessPolicyResourceActionTypeDTO),
    Account(AccountResourceActionTypeDTO),
    AddressBook(ResourceActionTypeDTO),
    ChangeCanister(ChangeCanisterResourceActionTypeDTO),
    Proposal(ProposalResourceActionTypeDTO),
    ProposalPolicy(ResourceActionTypeDTO),
    Settings(SettingsResourceActionTypeDTO),
    User(UserResourceActionTypeDTO),
    UserGroup(ResourceActionTypeDTO),
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
pub enum ResourceActionTypeDTO {
    List,
    Create,
    Read,
    Update,
    Delete,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum AccessPolicyResourceActionDTO {
    List,
    Read(ResourceTypeIdDTO),
    Edit(ResourceTypeIdDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum AccessPolicyResourceActionTypeDTO {
    List,
    Read,
    Edit,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum UserResourceActionDTO {
    List,
    Create,
    Read(ResourceIdDTO),
    Update(ResourceIdDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum UserResourceActionTypeDTO {
    List,
    Create,
    Read,
    Update,
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
pub enum AccountResourceActionTypeDTO {
    List,
    Create,
    Transfer,
    Read,
    Update,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum SettingsResourceActionDTO {
    Read,
    ReadConfig,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum SettingsResourceActionTypeDTO {
    Read,
    ReadConfig,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ChangeCanisterResourceActionDTO {
    Create,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ChangeCanisterResourceActionTypeDTO {
    Create,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProposalResourceActionDTO {
    List,
    Read(ResourceIdDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProposalResourceActionTypeDTO {
    List,
    Read,
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
    pub resource: ResourceDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetAccessPolicyResponse {
    pub policies: Vec<AccessPolicyDTO>,
    pub privileges: Vec<AccessPolicyCallerPrivilegesDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditAccessPolicyOperationDTO {
    pub input: EditAccessPolicyOperationInput,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ResourceAccessDTO {
    Deny(AllowLevelDTO),
    Allow(AllowDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditAccessPolicyOperationInput {
    pub resource: ResourceDTO,
    pub access: ResourceAccessDTO,
}
