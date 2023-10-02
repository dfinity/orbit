use super::{BankAccountIdDTO, TimestampRfc3339};
use candid::{CandidType, Deserialize};

pub type OperationIdDTO = String;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum OperationStatusDTO {
    Rejected,
    Completed,
    Pending,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct OperationDTO {
    pub id: OperationIdDTO,
    pub status: OperationStatusDTO,
    pub code: String,
    pub read: bool,
    pub created_at: TimestampRfc3339,
    pub feedback_reason: Option<String>,
    pub account: BankAccountIdDTO,
    pub feedback_time_at: Option<TimestampRfc3339>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct OperationListItemDTO {
    pub id: OperationIdDTO,
    pub status: OperationStatusDTO,
    pub code: String,
    pub created_at: TimestampRfc3339,
    pub account: BankAccountIdDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditOperationInput {
    pub read: Option<bool>,
    pub approve: Option<bool>,
    pub operation_id: OperationIdDTO,
    pub reason: Option<String>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EditOperationResponse {
    pub operation: OperationDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetOperationInput {
    pub operation_id: OperationIdDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetOperationResponse {
    pub operation: OperationDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListOperationsInput {
    pub status: Option<OperationStatusDTO>,
    pub code: Option<String>,
    pub read: Option<bool>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListOperationsResponse {
    pub operations: Vec<OperationListItemDTO>,
}
