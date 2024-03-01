use crate::{BasicUserDTO, MetadataDTO, PaginationInput, UserGroupDTO, UuidDTO};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProposalSpecifierDTO {
    AddAccount,
    AddUser,
    EditAccount(AccountSpecifierDTO),
    EditUser(UserSpecifierDTO),
    AddAddressBookEntry,
    EditAddressBookEntry(CommonSpecifierDTO),
    RemoveAddressBookEntry(CommonSpecifierDTO),
    Transfer(TransferSpecifierDTO),
    ChangeCanister,
    AddAccessPolicy,
    EditAccessPolicy(CommonSpecifierDTO),
    RemoveAccessPolicy(CommonSpecifierDTO),
    AddProposalPolicy,
    EditProposalPolicy(CommonSpecifierDTO),
    RemoveProposalPolicy(CommonSpecifierDTO),
    AddUserGroup,
    EditUserGroup(CommonSpecifierDTO),
    RemoveUserGroup(CommonSpecifierDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum UserSpecifierDTO {
    Any,
    Group(Vec<UuidDTO>),
    Id(Vec<UuidDTO>),
    Owner,
    Proposer,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ApprovalThresholdDTO {
    pub voters: UserSpecifierDTO,
    pub threshold: u16,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct MinimumVotesDTO {
    pub voters: UserSpecifierDTO,
    pub minimum: u16,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum CriteriaDTO {
    AutoAdopted,
    ApprovalThreshold(ApprovalThresholdDTO),
    MinimumVotes(MinimumVotesDTO),
    HasAddressBookMetadata(MetadataDTO),
    Or(Vec<CriteriaDTO>),
    And(Vec<CriteriaDTO>),
    Not(Box<CriteriaDTO>),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum CommonSpecifierDTO {
    Any,
    Id(Vec<UuidDTO>),
    Group(Vec<UuidDTO>),
}

pub type AccessControlUserSpecifierDTO = CommonSpecifierDTO;
pub type AccountSpecifierDTO = CommonSpecifierDTO;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum CommonActionSpecifierDTO {
    List,
    Create,
    Read(CommonSpecifierDTO),
    Update(CommonSpecifierDTO),
    Delete(CommonSpecifierDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ResourceTypeDTO {
    Account,
    User,
    UserGroup,
    AddressBook,
    AccessPolicy,
    ProposalPolicy,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ChangeCanisterActionSpecifierDTO {
    Create,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProposalActionSpecifierDTO {
    List,
    Read(CommonSpecifierDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum CanisterSettingsActionSpecifierDTO {
    Read,
    ReadConfig,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct TransferSpecifierDTO {
    pub account: CommonSpecifierDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum TransferActionSpecifierDTO {
    Create(TransferSpecifierDTO),
    Read(TransferSpecifierDTO),
    Delete(TransferSpecifierDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ResourceSpecifierDTO {
    Transfer(TransferActionSpecifierDTO),
    ChangeCanister(ChangeCanisterActionSpecifierDTO),
    CanisterSettings(CanisterSettingsActionSpecifierDTO),
    Proposal(ProposalActionSpecifierDTO),
    Account(CommonActionSpecifierDTO),
    User(CommonActionSpecifierDTO),
    UserGroup(CommonActionSpecifierDTO),
    AddressBook(CommonActionSpecifierDTO),
    AccessPolicy(CommonActionSpecifierDTO),
    ProposalPolicy(CommonActionSpecifierDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AccessPolicyCallerPrivilegesDTO {
    pub id: UuidDTO,
    pub can_edit: bool,
    pub can_delete: bool,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AccessPolicyDTO {
    pub id: UuidDTO,
    pub user: AccessControlUserSpecifierDTO,
    pub resource: ResourceSpecifierDTO,
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
pub struct ProposalPolicyCallerPrivilegesDTO {
    pub id: UuidDTO,
    pub can_edit: bool,
    pub can_delete: bool,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ProposalPolicyDTO {
    pub id: UuidDTO,
    pub specifier: ProposalSpecifierDTO,
    pub criteria: CriteriaDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetProposalPolicyInput {
    pub id: UuidDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetProposalPolicyResponse {
    pub policy: ProposalPolicyDTO,
    pub privileges: ProposalPolicyCallerPrivilegesDTO,
}

pub type ListProposalPoliciesInput = PaginationInput;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListProposalPoliciesResponse {
    pub policies: Vec<ProposalPolicyDTO>,
    pub next_offset: Option<u64>,
    pub total: u64,
    pub privileges: Vec<ProposalPolicyCallerPrivilegesDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AddAccessPolicyOperationInput {
    pub user: AccessControlUserSpecifierDTO,
    pub resource: ResourceSpecifierDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AddAccessPolicyOperationDTO {
    pub policy_id: Option<UuidDTO>,
    pub input: AddAccessPolicyOperationInput,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditAccessPolicyOperationInput {
    pub policy_id: UuidDTO,
    pub user: Option<AccessControlUserSpecifierDTO>,
    pub resource: Option<ResourceSpecifierDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditAccessPolicyOperationDTO {
    pub input: EditAccessPolicyOperationInput,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct RemoveAccessPolicyOperationInput {
    pub policy_id: UuidDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct RemoveAccessPolicyOperationDTO {
    pub input: RemoveAccessPolicyOperationInput,
}
