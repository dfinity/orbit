use super::{
    EditAccountOperationInput, TimestampRfc3339, TransferOperationDTO, TransferOperationInput,
};
use crate::{
    AddAccessPolicyOperationDTO, AddAccessPolicyOperationInput, AddAccountOperationDTO,
    AddAccountOperationInput, AddUserGroupOperationDTO, AddUserGroupOperationInput,
    AddUserOperationDTO, AddUserOperationInput, ChangeCanisterOperationDTO,
    ChangeCanisterOperationInput, CriteriaDTO, EditAccessPolicyOperationDTO,
    EditAccessPolicyOperationInput, EditAccountOperationDTO, EditUserGroupOperationDTO,
    EditUserGroupOperationInput, EditUserOperationDTO, EditUserOperationInput, ProposalPolicyDTO,
    ProposalSpecifierDTO, RemoveAccessPolicyOperationDTO, RemoveAccessPolicyOperationInput,
    RemoveUserGroupOperationDTO, RemoveUserGroupOperationInput, UuidDTO,
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

#[derive(CandidType, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ProposalStatusCodeDTO {
    Created,
    Adopted,
    Rejected,
    Cancelled,
    Scheduled,
    Processing,
    Completed,
    Failed,
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
    AddUser(Box<AddUserOperationDTO>),
    EditUser(Box<EditUserOperationDTO>),
    AddUserGroup(Box<AddUserGroupOperationDTO>),
    EditUserGroup(Box<EditUserGroupOperationDTO>),
    RemoveUserGroup(Box<RemoveUserGroupOperationDTO>),
    ChangeCanister(Box<ChangeCanisterOperationDTO>),
    AddAccessPolicy(Box<AddAccessPolicyOperationDTO>),
    EditAccessPolicy(Box<EditAccessPolicyOperationDTO>),
    RemoveAccessPolicy(Box<RemoveAccessPolicyOperationDTO>),
    AddProposalPolicy(Box<AddProposalPolicyOperationDTO>),
    EditProposalPolicy(Box<EditProposalPolicyOperationDTO>),
    RemoveProposalPolicy(Box<RemoveProposalPolicyOperationDTO>),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProposalOperationInput {
    Transfer(TransferOperationInput),
    AddAccount(AddAccountOperationInput),
    EditAccount(EditAccountOperationInput),
    AddUser(AddUserOperationInput),
    EditUser(EditUserOperationInput),
    AddUserGroup(AddUserGroupOperationInput),
    EditUserGroup(EditUserGroupOperationInput),
    RemoveUserGroup(RemoveUserGroupOperationInput),
    ChangeCanister(ChangeCanisterOperationInput),
    AddAccessPolicy(AddAccessPolicyOperationInput),
    EditAccessPolicy(EditAccessPolicyOperationInput),
    RemoveAccessPolicy(RemoveAccessPolicyOperationInput),
    AddProposalPolicy(AddProposalPolicyOperationInput),
    EditProposalPolicy(EditProposalPolicyOperationInput),
    RemoveProposalPolicy(RemoveProposalPolicyOperationInput),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProposalOperationTypeDTO {
    Transfer,
    AddAccount,
    EditAccount,
    AddUser,
    EditUser,
    AddUserGroup,
    EditUserGroup,
    RemoveUserGroup,
    ChangeCanister,
    AddAccessPolicy,
    EditAccessPolicy,
    RemoveAccessPolicy,
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
    pub metadata: Vec<(String, String)>,
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
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetProposalInput {
    pub proposal_id: UuidDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetProposalResponse {
    pub proposal: ProposalDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListProposalsInput {
    pub status: Option<Vec<ProposalStatusCodeDTO>>,
    pub operation_type: Option<ProposalOperationTypeDTO>,
    pub from_dt: Option<TimestampRfc3339>,
    pub to_dt: Option<TimestampRfc3339>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListProposalsResponse {
    pub proposals: Vec<ProposalDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListAccountProposalsInput {
    pub account_id: UuidDTO,
    pub status: Option<Vec<ProposalStatusCodeDTO>>,
    pub operation_type: Option<ProposalOperationTypeDTO>,
    pub from_dt: Option<TimestampRfc3339>,
    pub to_dt: Option<TimestampRfc3339>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListAccountProposalsResponse {
    pub proposals: Vec<ProposalDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct CreateProposalResponse {
    pub proposal: ProposalDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AddProposalPolicyOperationInput {
    pub specifier: ProposalSpecifierDTO,
    pub criteria: CriteriaDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AddProposalPolicyOperationDTO {
    pub policy: Option<ProposalPolicyDTO>,
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
