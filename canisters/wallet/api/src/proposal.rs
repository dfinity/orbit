use crate::{AddAccountOperationDTO, AddAccountOperationInput, EditAccountOperationDTO};

use super::{
    AccountIdDTO, EditAccountOperationInput, TimestampRfc3339, TransferOperationDTO,
    TransferOperationInput, UserIdDTO,
};
use candid::{CandidType, Deserialize};

pub type ProposalIdDTO = String;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProposalStatusDTO {
    Created,
    Adopted,
    Rejected,
    Scheduled { scheduled_at: TimestampRfc3339 },
    Processing { started_at: TimestampRfc3339 },
    Completed { completed_at: TimestampRfc3339 },
    Failed { reason: Option<String> },
    Cancelled { reason: Option<String> },
}

#[derive(CandidType, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ProposalStatusCodeDTO {
    Created,
    Adopted,
    Rejected,
    Scheduled,
    Processing,
    Completed,
    Failed,
    Cancelled,
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
    EditAccount(Box<EditAccountOperationDTO>),
    AddAccount(Box<AddAccountOperationDTO>),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProposalOperationInput {
    Transfer(TransferOperationInput),
    EditAccount(EditAccountOperationInput),
    AddAccount(AddAccountOperationInput),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProposalOperationTypeDTO {
    Transfer,
    EditAccount,
    AddAccount,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ProposalVoteDTO {
    pub user_id: UserIdDTO,
    pub status: ProposalVoteStatusDTO,
    pub status_reason: Option<String>,
    pub decided_at: TimestampRfc3339,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ProposalDTO {
    pub id: ProposalIdDTO,
    pub title: String,
    pub summary: Option<String>,
    pub operation: ProposalOperationDTO,
    pub proposed_by: Option<UserIdDTO>,
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
    pub proposal_id: ProposalIdDTO,
    pub reason: Option<String>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct VoteOnProposalResponse {
    pub proposal: ProposalDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetProposalInput {
    pub proposal_id: ProposalIdDTO,
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
    pub account_id: AccountIdDTO,
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
