use crate::models::{Operation, OperationId, UserId};
use candid::{CandidType, Deserialize};
use ic_canister_core::types::Timestamp;
use ic_canister_macros::stable_object;
use std::collections::HashSet;

/// Index of operations by the user id.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationUserIndex {
    /// The user thgat is associated with this operation.
    pub user_id: UserId,
    /// The time when the operation was created.
    pub created_at: Timestamp,
    /// The operation id, which is a UUID.
    pub id: OperationId,
}

#[derive(Clone, Debug)]
pub struct OperationUserIndexCriteria {
    pub user_id: UserId,
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl Operation {
    pub fn to_index_for_users(&self) -> Vec<OperationUserIndex> {
        let mut users = HashSet::<UserId>::new();
        if let Some(proposed_by) = &self.proposed_by {
            users.insert(proposed_by.to_owned());
        }
        self.decisions.iter().for_each(|d| {
            users.insert(d.user_id);
        });

        users
            .iter()
            .map(|user_id| OperationUserIndex {
                id: self.id.to_owned(),
                created_at: self.created_timestamp.to_owned(),
                user_id: user_id.to_owned(),
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
        let user_id = [u8::MAX; 16];
        let model = OperationUserIndex {
            id: operation_id,
            user_id,
            created_at: 0,
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = OperationUserIndex::from_bytes(serialized_model);

        assert_eq!(model.id, deserialized_model.id);
        assert_eq!(model.user_id, deserialized_model.user_id);
    }

    #[test]
    fn valid_user_operation_indexes() {
        let operation_id = [1; 16];
        let user_id = [u8::MAX; 16];
        let operation = Operation {
            id: operation_id,
            code: OperationCode::ApproveTransfer,
            created_timestamp: 0,
            last_modification_timestamp: 0,
            decisions: vec![
                OperationDecision {
                    user_id: [1; 16],
                    status_reason: None,
                    decided_dt: None,
                    last_modification_timestamp: 0,
                    read: false,
                    status: OperationStatus::Pending,
                },
                OperationDecision {
                    user_id: [2; 16],
                    status_reason: None,
                    decided_dt: None,
                    last_modification_timestamp: 0,
                    read: false,
                    status: OperationStatus::Pending,
                },
            ],
            metadata: vec![],
            proposed_by: Some(user_id),
            status: OperationStatus::Pending,
        };

        let indexes = operation.to_index_for_users();

        assert_eq!(indexes.len(), 3);
        assert!(indexes.iter().any(|i| i.user_id == user_id));
        assert!(indexes.iter().any(|i| i.user_id == [1; 16]));
        assert!(indexes.iter().any(|i| i.user_id == [2; 16]));
    }
}
