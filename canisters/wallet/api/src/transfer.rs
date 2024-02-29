use super::{AccountDTO, TimestampRfc3339};
use crate::{MetadataDTO, UuidDTO};
use candid::{CandidType, Deserialize};

pub type NetworkIdDTO = String;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct NetworkDTO {
    pub id: NetworkIdDTO,
    pub name: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct TransferOperationInput {
    pub from_account_id: UuidDTO,
    pub to: String,
    pub amount: candid::Nat,
    pub fee: Option<candid::Nat>,
    pub metadata: Vec<MetadataDTO>,
    pub network: Option<NetworkDTO>,
    pub description: Option<String>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct TransferOperationDTO {
    pub from_account: Option<AccountDTO>,
    pub network: NetworkDTO,
    pub input: TransferOperationInput,
    pub transfer_id: Option<UuidDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum TransferStatusDTO {
    Created,
    Processing {
        started_at: TimestampRfc3339,
    },
    Completed {
        signature: Option<String>,
        hash: Option<String>,
        completed_at: TimestampRfc3339,
    },
    Failed {
        reason: String,
    },
}

#[derive(CandidType, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum TransferStatusTypeDTO {
    Created,
    Processing,
    Completed,
    Failed,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct TransferDTO {
    pub id: UuidDTO,
    pub from_account_id: UuidDTO,
    pub to: String,
    pub fee: candid::Nat,
    pub amount: candid::Nat,
    pub status: TransferStatusDTO,
    pub network: NetworkDTO,
    pub metadata: Vec<MetadataDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct TransferResponse {
    pub transfer: TransferDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetTransfersInput {
    pub transfer_ids: Vec<UuidDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetTransfersResponse {
    pub transfers: Vec<TransferDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListAccountTransfersInput {
    pub status: Option<TransferStatusTypeDTO>,
    pub to_dt: Option<TimestampRfc3339>,
    pub from_dt: Option<TimestampRfc3339>,
    pub account_id: UuidDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct TransferListItemDTO {
    pub transfer_id: UuidDTO,
    pub status: TransferStatusDTO,
    pub to: String,
    pub amount: candid::Nat,
    pub created_at: TimestampRfc3339,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListAccountTransfersResponse {
    pub transfers: Vec<TransferListItemDTO>,
}
