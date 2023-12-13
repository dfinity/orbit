use crate::UuidDTO;
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum UserSpecifierDTO {
    Any,
    Group(Vec<UuidDTO>),
    Id(Vec<UuidDTO>),
    Owner,
    Proposer,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum CriteriaDTO {
    AutoAdopted,
    ApprovalThreshold(UserSpecifierDTO, u16),
    MinimumVotes(UserSpecifierDTO, u16),
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
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum UpgradeActionSpecifierDTO {
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
    ReadFeatures,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum AddressSpecifierDTO {
    Any,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct TransferSpecifierDTO {
    pub account: CommonSpecifierDTO,
    pub address: AddressSpecifierDTO,
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
    Upgrade(UpgradeActionSpecifierDTO),
    CanisterSettings(CanisterSettingsActionSpecifierDTO),
    Proposal(ProposalActionSpecifierDTO),
    Common(ResourceSpecifierCommonArgsDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ResourceSpecifierCommonArgsDTO {
    pub resource_type: ResourceTypeDTO,
    pub action: CommonActionSpecifierDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AccessControlPolicyDTO {
    pub id: UuidDTO,
    pub user: AccessControlUserSpecifierDTO,
    pub resource: ResourceSpecifierDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListAccessPoliciesInput {
    pub offset: Option<u64>,
    pub limit: Option<u16>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListAccessPoliciesResponse {
    pub policies: Vec<AccessControlPolicyDTO>,
    pub next_offset: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetAccessPolicyInput {
    pub id: UuidDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetAccessPolicyResponse {
    pub policy: AccessControlPolicyDTO,
}
