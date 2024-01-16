use crate::models::{Metadata, Transfer};
use ic_canister_core::utils::timestamp_to_rfc3339;
use uuid::Uuid;
use wallet_api::{NetworkDTO, TransferDTO, TransferListItemDTO};

#[derive(Default, Clone, Debug)]
pub struct TransferMapper {}

impl TransferMapper {
    pub fn to_dto(transfer: Transfer) -> TransferDTO {
        TransferDTO {
            id: Uuid::from_slice(&transfer.id)
                .unwrap()
                .hyphenated()
                .to_string(),
            amount: transfer.amount,
            fee: transfer.fee,
            metadata: Metadata::into_vec_dto(transfer.metadata),
            network: NetworkDTO {
                id: transfer.blockchain_network.to_owned(),
                name: transfer.blockchain_network.to_owned(),
            },
            from_account_id: Uuid::from_slice(&transfer.from_account)
                .unwrap()
                .hyphenated()
                .to_string(),
            to: transfer.to_address,
            status: transfer.status.into(),
        }
    }

    pub fn to_list_item_dto(transfer: Transfer) -> TransferListItemDTO {
        TransferListItemDTO {
            transfer_id: Uuid::from_slice(&transfer.id)
                .unwrap()
                .hyphenated()
                .to_string(),
            amount: transfer.amount,
            to: transfer.to_address,
            created_at: timestamp_to_rfc3339(&transfer.created_timestamp),
            status: transfer.status.into(),
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
