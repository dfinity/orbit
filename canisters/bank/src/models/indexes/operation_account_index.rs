use crate::models::{AccountId, Operation, OperationId};
use candid::{CandidType, Deserialize};
use ic_canister_core::types::Timestamp;
use ic_canister_macros::stable_object;
use std::collections::HashSet;

/// Index of operations by the account id.
#[stable_object]
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
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl Operation {
    pub fn to_index_for_accounts(&self) -> Vec<OperationAccountIndex> {
        let mut accounts = HashSet::<AccountId>::new();
        if let Some(originator_account_id) = &self.originator_account_id {
            accounts.insert(originator_account_id.to_owned());
        }
        self.decisions.iter().for_each(|d| {
            accounts.insert(d.account_id);
        });

        accounts
            .iter()
            .map(|account_id| OperationAccountIndex {
                id: self.id.to_owned(),
                created_at: self.created_timestamp.to_owned(),
                account_id: account_id.to_owned(),
            })
            .collect()
    }
}
