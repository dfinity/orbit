use super::{AccountIdDTO, TimestampRfc3339, TransferDTO, WalletDTO, WalletIdDTO};
use candid::{CandidType, Deserialize};

pub type OperationIdDTO = String;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum OperationStatusDTO {
    Rejected,
    Adopted,
    Pending,
    NotRequired,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct OperationContextDTO {
    pub transfer: Option<TransferDTO>,
    pub wallet: Option<WalletDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct OperationDecisionDTO {
    pub account_id: AccountIdDTO,
    pub read: bool,
    pub status: OperationStatusDTO,
    pub status_reason: Option<String>,
    pub decided_at: Option<TimestampRfc3339>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct OperationDTO {
    pub id: OperationIdDTO,
    pub status: OperationStatusDTO,
    pub code: String,
    pub created_at: TimestampRfc3339,
    pub metadata: Vec<(String, String)>,
    pub originator_account_id: Option<AccountIdDTO>,
    pub decisions: Vec<OperationDecisionDTO>,
    pub context: OperationContextDTO,
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
    pub from_dt: Option<TimestampRfc3339>,
    pub to_dt: Option<TimestampRfc3339>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListOperationsResponse {
    pub operations: Vec<OperationDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListWalletOperationsInput {
    pub wallet_id: WalletIdDTO,
    pub status: Option<OperationStatusDTO>,
    pub code: Option<String>,
    pub read: Option<bool>,
    pub from_dt: Option<TimestampRfc3339>,
    pub to_dt: Option<TimestampRfc3339>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListWalletOperationsResponse {
    pub operations: Vec<OperationDTO>,
}
