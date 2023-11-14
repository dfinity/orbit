use super::{AccountDTO, AccountIdDTO, TimestampRfc3339, TransferDTO, UserIdDTO};
use candid::{CandidType, Deserialize};

pub type ProposalIdDTO = String;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProposalStatusDTO {
    Rejected,
    Adopted,
    Pending,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProposalVoteStatusDTO {
    Rejected,
    Adopted,
    Pending,
    NotRequired,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ProposedTransferOperationDTO {
    pub transfer: TransferDTO,
    pub account: AccountDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProposalOperationDTO {
    Transfer(ProposedTransferOperationDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProposalOperationTypeDTO {
    Transfer,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ProposalVoteDTO {
    pub user_id: UserIdDTO,
    pub status: ProposalVoteStatusDTO,
    pub status_reason: Option<String>,
    pub decided_at: Option<TimestampRfc3339>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ProposalDTO {
    pub id: ProposalIdDTO,
    pub status: ProposalStatusDTO,
    pub operation: ProposalOperationDTO,
    pub created_at: TimestampRfc3339,
    pub metadata: Vec<(String, String)>,
    pub proposed_by: Option<UserIdDTO>,
    pub votes: Vec<ProposalVoteDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct VoteOnProposalInput {
    pub approve: Option<bool>,
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
    pub status: Option<ProposalStatusDTO>,
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
    pub status: Option<ProposalStatusDTO>,
    pub operation_type: Option<ProposalOperationTypeDTO>,
    pub from_dt: Option<TimestampRfc3339>,
    pub to_dt: Option<TimestampRfc3339>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListAccountProposalsResponse {
    pub proposals: Vec<ProposalDTO>,
}
