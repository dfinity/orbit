use ic_canister_core::types::Timestamp;
use ic_canister_macros::storable;
use std::hash::Hash;

/// Represents the balance of a account.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccountBalance {
    /// The account id, which is a UUID.
    pub balance: candid::Nat,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}
