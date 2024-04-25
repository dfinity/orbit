use crate::models::TransferStatus;
use ic_canister_core::utils::timestamp_to_rfc3339;
use wallet_api::{TransferStatusDTO, TransferStatusTypeDTO};

impl From<TransferStatus> for TransferStatusDTO {
    fn from(status: TransferStatus) -> Self {
        match status {
            TransferStatus::Processing { started_at } => TransferStatusDTO::Processing {
                started_at: timestamp_to_rfc3339(&started_at),
            },
            TransferStatus::Created => TransferStatusDTO::Created,
            TransferStatus::Completed {
                signature,
                hash,
                completed_at,
            } => TransferStatusDTO::Completed {
                signature: signature.map(|s| s.to_owned()),
                hash: hash.map(|h| h.to_owned()),
                completed_at: timestamp_to_rfc3339(&completed_at),
            },
            TransferStatus::Failed { reason } => TransferStatusDTO::Failed {
                reason: reason.to_owned(),
            },
        }
    }
}

impl From<TransferStatus> for TransferStatusTypeDTO {
    fn from(status: TransferStatus) -> Self {
        match status {
            TransferStatus::Processing { .. } => TransferStatusTypeDTO::Processing,
            TransferStatus::Created => TransferStatusTypeDTO::Created,
            TransferStatus::Completed { .. } => TransferStatusTypeDTO::Completed,
            TransferStatus::Failed { .. } => TransferStatusTypeDTO::Failed,
        }
    }
}
