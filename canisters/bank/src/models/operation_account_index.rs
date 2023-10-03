use super::{AccountId, Operation, OperationCode, OperationId, OperationStatus};
use candid::{CandidType, Deserialize};
use ic_canister_macros::stable_object;

/// Index of operations by the account id.
#[stable_object(size = 128)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationAccountIndex {
    /// The account thgat is associated with this operation.
    pub account_id: AccountId,
    /// An operation code that represents the operation type, e.g. "transfer".
    pub code: OperationCode,
    /// The operation id, which is a UUID.
    pub id: OperationId,
}

#[derive(Clone, Debug)]
pub struct OperationAccountIndexCriteria {
    pub account_id: AccountId,
    pub code: Option<OperationCode>,
    pub status: Option<OperationStatus>,
    pub read: Option<bool>,
}

impl Operation {
    pub fn as_index_for_account(&self) -> OperationAccountIndex {
        OperationAccountIndex {
            id: self.id.to_owned(),
            code: self.code.to_owned(),
            account_id: self.account_id.to_owned(),
        }
    }
}
