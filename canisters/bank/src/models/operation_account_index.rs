use super::{AccountId, OperationCode, OperationId, OperationStatus};
use candid::{CandidType, Deserialize};
use ic_canister_macros::stable_object;

/// Index of operations by the account id.
#[stable_object(size = 128)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationAccountIndex {
    /// The account thgat is associated with this operation.
    pub account_id: AccountId,
    /// If the operation is marked as read by the account that it is associated with.
    pub read: bool,
    /// The status of the operation.
    pub status: OperationStatus,
    /// An operation code that represents the operation type, e.g. "transfer".
    pub code: OperationCode,
    /// The operation id, which is a UUID.
    pub id: OperationId,
}

impl OperationAccountIndex {
    pub fn value(&self) {}
}
