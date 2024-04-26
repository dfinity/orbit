use crate::{resource::ResourceDTO, MetadataDTO, PaginationInput, ResourceIdsDTO, UuidDTO};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProposalSpecifierDTO {
    AddAccount,
    AddUser,
    EditAccount(ResourceIdsDTO),
    EditUser(ResourceIdsDTO),
    AddAddressBookEntry,
    EditAddressBookEntry(ResourceIdsDTO),
    RemoveAddressBookEntry(ResourceIdsDTO),
    Transfer(ResourceIdsDTO),
    ChangeCanister,
    EditPermission(ResourceSpecifierDTO),
    AddProposalPolicy,
    EditProposalPolicy(ResourceIdsDTO),
    RemoveProposalPolicy(ResourceIdsDTO),
    AddUserGroup,
    EditUserGroup(ResourceIdsDTO),
    RemoveUserGroup(ResourceIdsDTO),
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
pub enum ApprovalCriteriaInput {
    Remove,
    Set(CriteriaDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum CriteriaDTO {
    AutoAdopted,
    ApprovalThreshold(ApprovalThresholdDTO),
    MinimumVotes(MinimumVotesDTO),
    HasAddressBookMetadata(MetadataDTO),
    HasAddressInAddressBook,
    Or(Vec<CriteriaDTO>),
    And(Vec<CriteriaDTO>),
    Not(Box<CriteriaDTO>),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum EvaluationStatusDTO {
    Adopted,
    Rejected,
    Pending,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum EvaluatedCriteriaDTO {
    AutoAdopted,
    ApprovalThreshold {
        min_required_votes: usize,
        total_possible_votes: usize,
        votes: Vec<UuidDTO>,
    },
    MinimumVotes {
        min_required_votes: usize,
        votes: Vec<UuidDTO>,
        total_possible_votes: usize,
    },
    HasAddressBookMetadata {
        metadata: MetadataDTO,
    },
    HasAddressInAddressBook,
    Or(Vec<CriteriaResultDTO>),
    And(Vec<CriteriaResultDTO>),
    Not(Box<CriteriaResultDTO>),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct CriteriaResultDTO {
    pub status: EvaluationStatusDTO,
    pub evaluated_criteria: EvaluatedCriteriaDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ProposalEvaluationResultDTO {
    pub proposal_id: UuidDTO,
    pub status: EvaluationStatusDTO,
    pub policy_results: Vec<CriteriaResultDTO>,
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
