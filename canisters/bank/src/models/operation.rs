use super::{AccountId, OperationCode, OperationStatus};
use candid::{CandidType, Deserialize};
use ic_canister_core::types::{Timestamp, UUID};
use ic_canister_macros::stable_object;

/// The operation id, which is a UUID.
pub type OperationId = UUID;

#[stable_object(size = 256)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationFeedback {
    pub created_at: Timestamp,
    pub reason: Option<String>,
}

/// Represents an operation within the system.
#[stable_object(size = 256)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Operation {
    /// The operation id, which is a UUID.
    pub id: OperationId,
    /// The account id that this operation is assigned to.
    pub account_id: AccountId,
    /// The status of the operation.
    pub status: OperationStatus,
    /// An operation code that represents the operation type, e.g. "transfer".
    pub code: OperationCode,
    /// If the operation is marked as read by the account that it is associated with.
    pub read: bool,
    /// The timestamp of the operation creation.
    pub feedback: Option<OperationFeedback>,
    /// The timestamp of the operation creation.
    pub created_timestamp: Timestamp,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}

#[stable_object(size = 48)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationKey {
    /// The operation id, which is a UUID.
    pub id: OperationId,
}

impl Operation {
    /// Creates a new operation key from the given key components.
    pub fn key(operation_id: OperationId) -> OperationKey {
        OperationKey { id: operation_id }
    }

    pub fn as_key(&self) -> OperationKey {
        Operation::key(self.id.to_owned())
    }

    pub fn min_code() -> String {
        "".to_string()
    }

    pub fn max_code() -> String {
        "zzzzzzzzzzzzzzzzzzz".to_string()
    }
}
