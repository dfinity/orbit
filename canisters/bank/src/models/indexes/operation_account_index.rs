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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{OperationCode, OperationDecision, OperationStatus};
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let operation_id = [1; 16];
        let account_id = [u8::MAX; 16];
        let model = OperationAccountIndex {
            id: operation_id,
            account_id,
            created_at: 0,
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = OperationAccountIndex::from_bytes(serialized_model);

        assert_eq!(model.id, deserialized_model.id);
        assert_eq!(model.account_id, deserialized_model.account_id);
    }

    #[test]
    fn valid_account_operation_indexes() {
        let operation_id = [1; 16];
        let account_id = [u8::MAX; 16];
        let operation = Operation {
            id: operation_id,
            code: OperationCode::ApproveTransfer,
            created_timestamp: 0,
            last_modification_timestamp: 0,
            decisions: vec![
                OperationDecision {
                    account_id: [1; 16],
                    status_reason: None,
                    decided_dt: None,
                    last_modification_timestamp: 0,
                    read: false,
                    status: OperationStatus::Pending,
                },
                OperationDecision {
                    account_id: [2; 16],
                    status_reason: None,
                    decided_dt: None,
                    last_modification_timestamp: 0,
                    read: false,
                    status: OperationStatus::Pending,
                },
            ],
            metadata: vec![],
            originator_account_id: Some(account_id),
            status: OperationStatus::Pending,
        };

        let indexes = operation.to_index_for_accounts();

        assert_eq!(indexes.len(), 3);
        assert!(indexes.iter().any(|i| i.account_id == account_id));
        assert!(indexes.iter().any(|i| i.account_id == [1; 16]));
        assert!(indexes.iter().any(|i| i.account_id == [2; 16]));
    }
}
