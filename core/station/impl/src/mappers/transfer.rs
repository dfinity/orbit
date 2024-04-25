use crate::models::Transfer;
use ic_canister_core::utils::timestamp_to_rfc3339;
use station_api::{NetworkDTO, TransferDTO, TransferListItemDTO};
use uuid::Uuid;

#[derive(Default, Clone, Debug)]
pub struct TransferMapper {}

impl TransferMapper {
    pub fn to_dto(transfer: Transfer) -> TransferDTO {
        TransferDTO {
            id: Uuid::from_bytes(transfer.id).hyphenated().to_string(),
            proposal_id: Uuid::from_bytes(transfer.proposal_id)
                .hyphenated()
                .to_string(),
            amount: transfer.amount,
            fee: transfer.fee,
            metadata: transfer.metadata.into_vec_dto(),
            network: NetworkDTO {
                id: transfer.blockchain_network.to_owned(),
                name: transfer.blockchain_network.to_owned(),
            },
            from_account_id: Uuid::from_bytes(transfer.from_account)
                .hyphenated()
                .to_string(),
            to: transfer.to_address,
            status: transfer.status.into(),
        }
    }

    pub fn to_list_item_dto(transfer: Transfer) -> TransferListItemDTO {
        TransferListItemDTO {
            transfer_id: Uuid::from_bytes(transfer.id).hyphenated().to_string(),
            amount: transfer.amount,
            to: transfer.to_address,
            created_at: timestamp_to_rfc3339(&transfer.created_timestamp),
            status: transfer.status.into(),
            proposal_id: Uuid::from_bytes(transfer.proposal_id)
                .hyphenated()
                .to_string(),
        }
    }
}

impl Transfer {
    pub fn to_dto(&self) -> TransferDTO {
        TransferMapper::to_dto(self.clone())
    }

    pub fn to_list_item_dto(&self) -> TransferListItemDTO {
        TransferMapper::to_list_item_dto(self.clone())
    }
}
