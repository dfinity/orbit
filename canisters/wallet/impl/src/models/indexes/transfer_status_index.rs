use crate::models::Transfer;
use candid::{CandidType, Deserialize};
use ic_canister_core::types::{Timestamp, UUID};
use ic_canister_macros::stable_object;
use std::hash::Hash;

/// Represents a transfer index by its status.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferStatusIndex {
    /// The status of the transfer.
    pub status: String,
    /// The last time the transfer was modified.
    pub last_modification_timestamp: Timestamp,
    /// The transfer id, which is a UUID.
    pub transfer_id: UUID,
}

#[derive(Clone, Debug)]
pub struct TransferStatusIndexCriteria {
    pub status: String,
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl Transfer {
    pub fn to_index_by_status(&self) -> TransferStatusIndex {
        TransferStatusIndex {
            status: self.status.to_string(),
            last_modification_timestamp: self.last_modification_timestamp,
            transfer_id: self.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{transfer_test_utils::mock_transfer, TransferStatus};

    #[test]
    fn test_transfer_to_index_by_status() {
        let mut transfer = mock_transfer();
        transfer.last_modification_timestamp = 5;
        transfer.status = TransferStatus::Created;

        let index = transfer.to_index_by_status();

        assert_eq!(index.transfer_id, transfer.id);
        assert_eq!(index.last_modification_timestamp, 5);
        assert_eq!(index.status, "created");
    }
}
