use super::{OperationIdDTO, TimestampRfc3339, WalletIdDTO};
use candid::{CandidType, Deserialize};

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
    pub from_wallet_id: WalletIdDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum TransferStatusDTO {
    Cancelled {
        reason: Option<String>,
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
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct TransferDTO {
    pub id: OperationIdDTO,
    pub to: String,
    pub fee: candid::Nat,
    pub status: TransferStatusDTO,
    pub expiration_dt: TimestampRfc3339,
    pub execution_plan: TransferExecutionScheduleDTO,
    pub metadata: Vec<TransferMetadataDTO>,
    pub network: NetworkDTO,
    pub amount: candid::Nat,
    pub from_wallet_id: WalletIdDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct TransferResponse {
    pub transfer: TransferDTO,
}
