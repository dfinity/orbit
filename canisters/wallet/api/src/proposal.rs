use super::{
    EditAccountOperationInput, TimestampRfc3339, TransferOperationDTO, TransferOperationInput,
};
use crate::{
    AddAccountOperationDTO, AddAccountOperationInput, AddAddressBookEntryOperationDTO,
    AddAddressBookEntryOperationInput, AddUserGroupOperationDTO, AddUserGroupOperationInput,
    AddUserOperationDTO, AddUserOperationInput, ChangeCanisterOperationDTO,
    ChangeCanisterOperationInput, CriteriaDTO, DisplayUserDTO, EditAccessPolicyOperationDTO,
    EditAccessPolicyOperationInput, EditAccountOperationDTO, EditAddressBookEntryOperationDTO,
    EditAddressBookEntryOperationInput, EditUserGroupOperationDTO, EditUserGroupOperationInput,
    EditUserOperationDTO, EditUserOperationInput, PaginationInput, ProposalSpecifierDTO,
    RemoveAddressBookEntryOperationDTO, RemoveAddressBookEntryOperationInput,
    RemoveUserGroupOperationDTO, RemoveUserGroupOperationInput, SortDirection, UuidDTO,
};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProposalStatusDTO {
    Created,
    Adopted,
    Rejected,
    Cancelled { reason: Option<String> },
    Scheduled { scheduled_at: TimestampRfc3339 },
    Processing { started_at: TimestampRfc3339 },
    Completed { completed_at: TimestampRfc3339 },
    Failed { reason: Option<String> },
}

#[derive(CandidType, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProposalStatusCodeDTO {
    Created = 0,
    Adopted = 1,
    Rejected = 2,
    Cancelled = 3,
    Scheduled = 4,
    Processing = 5,
    Completed = 6,
    Failed = 7,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProposalVoteStatusDTO {
    Accepted,
    Rejected,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProposalExecutionScheduleDTO {
    Immediate,
    Scheduled { execution_time: TimestampRfc3339 },
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProposalOperationDTO {
    Transfer(Box<TransferOperationDTO>),
    AddAccount(Box<AddAccountOperationDTO>),
    EditAccount(Box<EditAccountOperationDTO>),
    AddAddressBookEntry(Box<AddAddressBookEntryOperationDTO>),
    EditAddressBookEntry(Box<EditAddressBookEntryOperationDTO>),
    RemoveAddressBookEntry(Box<RemoveAddressBookEntryOperationDTO>),
    AddUser(Box<AddUserOperationDTO>),
    EditUser(Box<EditUserOperationDTO>),
    AddUserGroup(Box<AddUserGroupOperationDTO>),
    EditUserGroup(Box<EditUserGroupOperationDTO>),
    RemoveUserGroup(Box<RemoveUserGroupOperationDTO>),
    ChangeCanister(Box<ChangeCanisterOperationDTO>),
    EditAccessPolicy(Box<EditAccessPolicyOperationDTO>),
    AddProposalPolicy(Box<AddProposalPolicyOperationDTO>),
    EditProposalPolicy(Box<EditProposalPolicyOperationDTO>),
    RemoveProposalPolicy(Box<RemoveProposalPolicyOperationDTO>),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProposalOperationInput {
    Transfer(TransferOperationInput),
    AddAccount(AddAccountOperationInput),
    EditAccount(EditAccountOperationInput),
    AddAddressBookEntry(AddAddressBookEntryOperationInput),
    EditAddressBookEntry(EditAddressBookEntryOperationInput),
    RemoveAddressBookEntry(RemoveAddressBookEntryOperationInput),
    AddUser(AddUserOperationInput),
    EditUser(EditUserOperationInput),
    AddUserGroup(AddUserGroupOperationInput),
    EditUserGroup(EditUserGroupOperationInput),
    RemoveUserGroup(RemoveUserGroupOperationInput),
    ChangeCanister(ChangeCanisterOperationInput),
    EditAccessPolicy(EditAccessPolicyOperationInput),
    AddProposalPolicy(AddProposalPolicyOperationInput),
    EditProposalPolicy(EditProposalPolicyOperationInput),
    RemoveProposalPolicy(RemoveProposalPolicyOperationInput),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProposalOperationTypeDTO {
    Transfer,
    AddAccount,
    EditAccount,
    AddAddressBookEntry,
    EditAddressBookEntry,
    RemoveAddressBookEntry,
    AddUser,
    EditUser,
    AddUserGroup,
    EditUserGroup,
    RemoveUserGroup,
    ChangeCanister,
    EditAccessPolicy,
    AddProposalPolicy,
    EditProposalPolicy,
    RemoveProposalPolicy,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ProposalVoteDTO {
    pub user_id: UuidDTO,
    pub status: ProposalVoteStatusDTO,
    pub status_reason: Option<String>,
    pub decided_at: TimestampRfc3339,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ProposalDTO {
    pub id: UuidDTO,
    pub title: String,
    pub summary: Option<String>,
    pub operation: ProposalOperationDTO,
    pub proposed_by: UuidDTO,
    pub votes: Vec<ProposalVoteDTO>,
    pub created_at: TimestampRfc3339,
    pub status: ProposalStatusDTO,
    pub expiration_dt: TimestampRfc3339,
    pub execution_plan: ProposalExecutionScheduleDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ProposalCallerPrivilegesDTO {
    pub id: UuidDTO,
    pub can_vote: bool,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ProposalAdditionalInfoDTO {
    pub id: UuidDTO,
    pub proposer_name: Option<String>,
    pub voters: Vec<DisplayUserDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct CreateProposalInput {
    pub operation: ProposalOperationInput,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub execution_plan: Option<ProposalExecutionScheduleDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct VoteOnProposalInput {
    pub approve: bool,
    pub proposal_id: UuidDTO,
    pub reason: Option<String>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct VoteOnProposalResponse {
    pub proposal: ProposalDTO,
    pub privileges: ProposalCallerPrivilegesDTO,
    pub additional_info: ProposalAdditionalInfoDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetProposalInput {
    pub proposal_id: UuidDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetProposalResponse {
    pub proposal: ProposalDTO,
    pub privileges: ProposalCallerPrivilegesDTO,
    pub additional_info: ProposalAdditionalInfoDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ListProposalsOperationTypeDTO {
    Transfer(Option<String>),
    AddAccount,
    EditAccount,
    AddAddressBookEntry,
    EditAddressBookEntry,
    RemoveAddressBookEntry,
    AddUser,
    EditUser,
    AddUserGroup,
    EditUserGroup,
    RemoveUserGroup,
    ChangeCanister,
    EditAccessPolicy,
    AddProposalPolicy,
    EditProposalPolicy,
    RemoveProposalPolicy,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ListProposalsSortBy {
    CreatedAt(SortDirection),
    ExpirationDt(SortDirection),
    LastModificationDt(SortDirection),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListProposalsInput {
    pub voter_ids: Option<Vec<UuidDTO>>,
    pub proposer_ids: Option<Vec<UuidDTO>>,
    pub statuses: Option<Vec<ProposalStatusCodeDTO>>,
    pub operation_types: Option<Vec<ListProposalsOperationTypeDTO>>,
    pub expiration_from_dt: Option<TimestampRfc3339>,
    pub expiration_to_dt: Option<TimestampRfc3339>,
    pub created_from_dt: Option<TimestampRfc3339>,
    pub created_to_dt: Option<TimestampRfc3339>,
    pub paginate: Option<PaginationInput>,
    pub sort_by: Option<ListProposalsSortBy>,
    pub only_votable: bool,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListProposalsResponse {
    pub proposals: Vec<ProposalDTO>,
    pub next_offset: Option<u64>,
    pub total: u64,
    pub privileges: Vec<ProposalCallerPrivilegesDTO>,
    pub additional_info: Vec<ProposalAdditionalInfoDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetNextVotableProposalInput {
    pub excluded_proposal_ids: Vec<UuidDTO>,
    pub operation_types: Option<Vec<ListProposalsOperationTypeDTO>>,
}

pub type GetNextVotableProposalResponse = Option<GetProposalResponse>;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct CreateProposalResponse {
    pub proposal: ProposalDTO,
    pub privileges: ProposalCallerPrivilegesDTO,
    pub additional_info: ProposalAdditionalInfoDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AddProposalPolicyOperationInput {
    pub specifier: ProposalSpecifierDTO,
    pub criteria: CriteriaDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AddProposalPolicyOperationDTO {
    pub policy_id: Option<UuidDTO>,
    pub input: AddProposalPolicyOperationInput,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditProposalPolicyOperationInput {
    pub policy_id: UuidDTO,
    pub specifier: Option<ProposalSpecifierDTO>,
    pub criteria: Option<CriteriaDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditProposalPolicyOperationDTO {
    pub input: EditProposalPolicyOperationInput,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct RemoveProposalPolicyOperationInput {
    pub policy_id: UuidDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct RemoveProposalPolicyOperationDTO {
    pub input: RemoveProposalPolicyOperationInput,
}
