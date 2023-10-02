use super::{Transfer, TransferId, WalletId};
use candid::{CandidType, Deserialize};
use ic_canister_core::types::Timestamp;
use ic_canister_macros::stable_object;
use std::hash::Hash;

/// Represents a transfer list index in the system.
#[stable_object(size = 64)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferListIndexKey {
    /// The wallet associated with the transfer.
    pub wallet_id: WalletId,
    /// The timestamp of the transfer creation.
    pub created_timestamp: Timestamp,
}

#[stable_object(size = 128)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferListIndex {
    /// The transfer id, which is a UUID.
    pub transfer_id: TransferId,
    /// The string representation of the transfer status.
    pub status: String,
    /// The wallet associated with the transfer.
    pub wallet_id: WalletId,
    /// The timestamp of the transfer creation.
    pub created_timestamp: Timestamp,
}

impl TransferListIndex {
    /// The default criteria interval in nanoseconds (30 days).
    pub const DEFAULT_CRITERIA_INTERVAL_NS: u64 = 30 * 24 * 60 * 60 * 1_000_000_000;

    pub fn key(wallet_id: WalletId, created_timestamp: Timestamp) -> TransferListIndexKey {
        TransferListIndexKey {
            wallet_id,
            created_timestamp,
        }
    }

    pub fn as_key(&self) -> TransferListIndexKey {
        Self::key(self.wallet_id, self.created_timestamp)
    }
}

impl Transfer {
    pub fn as_list_index(&self) -> TransferListIndex {
        TransferListIndex {
            transfer_id: self.id,
            status: self.status.to_string(),
            wallet_id: self.from_wallet,
            created_timestamp: self.created_timestamp,
        }
    }
}
