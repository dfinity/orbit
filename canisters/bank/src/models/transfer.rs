use super::WalletId;
use candid::{CandidType, Deserialize};
use ic_canister_core::{
    model::ModelValidator,
    types::{Timestamp, UUID},
};
use ic_canister_macros::stable_object;
use std::hash::Hash;

/// The transfer id, which is a UUID.
pub type TransferId = UUID;

/// Represents a transfer in the system.
#[stable_object(size = 1024)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Transfer {
    /// The transfer id, which is a UUID.
    pub id: TransferId,
    /// The wallet id that the transfer is from.
    pub from_wallet: WalletId,
    /// The destination address of the transfer.
    pub to_address: String,
    /// The current status of the transfer.
    pub status: String,
    /// The amount of the transfer.
    pub amount: candid::Nat,
    /// The fee of the transfer.
    pub fee: candid::Nat,
    /// The expiration date of the transfer.
    pub expiration_dt: Timestamp,
    /// The execution plan of the transfer.
    pub execution_plan: Timestamp,
    /// The blockchain network that the transfer will be executed on.
    pub blockchain_network: String,
    /// The transfer metadata (e.g. `memo`, `description`, etc.)
    pub metadata: Vec<(String, String)>,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}

#[stable_object(size = 64)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferKey {
    /// The transfer id, which is a UUID.
    pub id: TransferId,
}

impl Transfer {
    /// Creates a new transfer key from the given key components.
    pub fn key(id: TransferId) -> TransferKey {
        TransferKey { id }
    }

    pub fn as_key(&self) -> TransferKey {
        Self::key(self.id)
    }
}

impl ModelValidator for Transfer {}
