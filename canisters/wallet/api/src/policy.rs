use crate::{MetadataDTO, PaginationInput, ResourceDTO, UuidDTO};
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
    EditAccessPolicy(ResourceSpecifierDTO),
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
pub enum ResourceSpecifierDTO {
    Any,
    Resource(ResourceDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct TransferSpecifierDTO {
    pub account: CommonSpecifierDTO,
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
