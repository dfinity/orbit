use crate::{
    models::{AccountId, Operation, OperationCode, OperationId, OperationStatus},
    repositories::OperationRepository,
};
use candid::{CandidType, Deserialize};
use ic_canister_core::{repository::Repository, types::Timestamp};
use ic_canister_macros::stable_object;

/// Index of operations by the account id.
#[stable_object(size = 128)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationAccountIndex {
    /// The account thgat is associated with this operation.
    pub account_id: AccountId,
    /// The time when the operation was created.
    pub created_at: Timestamp,
    /// The operation id, which is a UUID.
    pub id: OperationId,
}

#[derive(Clone, Debug)]
pub struct OperationAccountIndexCriteria {
    pub account_id: AccountId,
    pub code: Option<OperationCode>,
    pub status: Option<OperationStatus>,
    pub read: Option<bool>,
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl Operation {
    pub fn as_index_for_account(&self) -> OperationAccountIndex {
        OperationAccountIndex {
            id: self.id.to_owned(),
            created_at: self.created_timestamp.to_owned(),
            account_id: self.account_id.to_owned(),
        }
    }
}

impl OperationAccountIndex {
    pub fn to_operation(&self) -> Operation {
        OperationRepository::default()
            .get(&Operation::key(self.id))
            .expect("Operation not found")
    }
}
