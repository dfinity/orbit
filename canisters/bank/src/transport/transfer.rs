use super::{AccountIdDTO, TimestampRfc3339};
use candid::{CandidType, Deserialize};

pub type TransferIdDTO = String;
pub type NetworkIdDTO = String;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum TransferExecutionScheduleDTO {
    Immediate,
    Scheduled { execution_time: TimestampRfc3339 },
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct TransferMetadataDTO {
    pub key: String,
    pub value: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct NetworkDTO {
    pub id: NetworkIdDTO,
    pub name: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct TransferInput {
    pub to: String,
    pub fee: Option<candid::Nat>,
    pub expiration_dt: Option<TimestampRfc3339>,
    pub execution_plan: Option<TransferExecutionScheduleDTO>,
    pub metadata: Option<Vec<TransferMetadataDTO>>,
    pub network: Option<NetworkDTO>,
    pub amount: candid::Nat,
    pub from_account_id: AccountIdDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum TransferStatusDTO {
    Cancelled {
        reason: Option<String>,
    },
    Processing {
        started_at: TimestampRfc3339,
    },
    Submitted,
    Pending,
    Completed {
        signature: Option<String>,
        hash: Option<String>,
        completed_at: TimestampRfc3339,
    },
    Approved,
    Rejected {
        reason: String,
    },
    Failed {
        reason: String,
    },
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct TransferDTO {
    pub id: TransferIdDTO,
    pub from_account_id: AccountIdDTO,
    pub to: String,
    pub fee: candid::Nat,
    pub amount: candid::Nat,
    pub status: TransferStatusDTO,
    pub expiration_dt: TimestampRfc3339,
    pub execution_plan: TransferExecutionScheduleDTO,
    pub network: NetworkDTO,
    pub metadata: Vec<TransferMetadataDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct TransferResponse {
    pub transfer: TransferDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetTransferInput {
    pub transfer_id: TransferIdDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetTransferResponse {
    pub transfer: TransferDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetTransfersInput {
    pub transfer_ids: Vec<TransferIdDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetTransfersResponse {
    pub transfers: Vec<TransferDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListAccountTransfersInput {
    pub status: Option<String>,
    pub to_dt: Option<TimestampRfc3339>,
    pub from_dt: Option<TimestampRfc3339>,
    pub account_id: AccountIdDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct TransferListItemDTO {
    pub transfer_id: TransferIdDTO,
    pub status: TransferStatusDTO,
    pub to: String,
    pub amount: candid::Nat,
    pub created_at: TimestampRfc3339,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListAccountTransfersResponse {
    pub transfers: Vec<TransferListItemDTO>,
}
