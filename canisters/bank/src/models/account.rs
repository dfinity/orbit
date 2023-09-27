use candid::{CandidType, Deserialize};
use ic_canister_core::types::{Timestamp, UUID};
use ic_canister_macros::stable_object;

/// The wallet id, which is a UUID.
pub type AccountId = UUID;

/// Represents an account within the system.
///
/// An account can be associated with one or more principal ids.
#[stable_object(size = 1024)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Account {
    /// The account id, which is a UUID.
    pub id: AccountId,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}
