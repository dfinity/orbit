use crate::models::{Transfer, TransferId};
use candid::{CandidType, Deserialize};
use ic_canister_core::types::Timestamp;
use ic_canister_macros::stable_object;
use std::hash::Hash;

/// Represents a transfer index by execution time.
#[stable_object(size = 64)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferExpirationTimeIndex {
    /// The time the transfer is scheduled to be set as expired if not executed.
    pub expiration_dt: Timestamp,
    /// The transfer id, which is a UUID.
    pub transfer_id: TransferId,
}

#[derive(Clone, Debug)]
pub struct TransferExpirationTimeIndexCriteria {
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl Transfer {
    pub fn to_index_by_expiration_dt(&self) -> TransferExpirationTimeIndex {
        TransferExpirationTimeIndex {
            expiration_dt: self.expiration_dt,
            transfer_id: self.id,
        }
    }
}
