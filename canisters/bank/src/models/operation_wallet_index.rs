use super::{OperationCode, OperationId, OperationStatus, WalletId};
use candid::{CandidType, Deserialize};
use ic_canister_macros::stable_object;

/// Index of operations by wallet id.
#[stable_object(size = 128)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationWalletIndex {
    /// The wallet id that is associated with this operation.
    pub wallet_id: WalletId,
    /// If the operation is marked as read by the account that it is associated with.
    pub read: bool,
    /// The status of the operation.
    pub status: OperationStatus,
    /// An operation code that represents the operation type, e.g. "transfer".
    pub code: OperationCode,
    /// The operation id, which is a UUID.
    pub id: OperationId,
}

impl OperationWalletIndex {
    pub fn value(&self) {}
}
