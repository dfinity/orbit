use candid::{CandidType, Deserialize};
use ic_canister_core::types::Timestamp;
use ic_canister_macros::stable_object;
use std::hash::Hash;

/// Represents the balance of a wallet.
#[stable_object(size = 64)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WalletBalance {
    /// The wallet id, which is a UUID.
    pub balance: u128,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}
