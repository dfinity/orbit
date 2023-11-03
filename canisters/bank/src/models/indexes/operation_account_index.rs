use crate::mappers::HelperMapper;
use crate::models::{AccountId, Operation, OperationId, OPERATION_METADATA_KEY_ACCOUNT_ID};
use candid::{CandidType, Deserialize};
use ic_canister_core::types::Timestamp;
use ic_canister_macros::stable_object;

/// Index of operations by account id.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationAccountIndex {
    /// The account id that is associated with this operation.
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
    pub fn to_index_for_account(&self) -> Option<OperationAccountIndex> {
        let metadata = self.metadata_map();
        if let Some(unparsed_account_id) = metadata.get(OPERATION_METADATA_KEY_ACCOUNT_ID) {
            let account_id = HelperMapper::to_uuid(unparsed_account_id.to_owned())
                .expect("Failed to parse account id");

            return Some(OperationAccountIndex {
                id: self.id.to_owned(),
                created_at: self.created_timestamp.to_owned(),
                account_id: *account_id.as_bytes(),
            });
        }

        // This operation is not related to a account.
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{OperationCode, OperationStatus};
    use ic_stable_structures::Storable;
    use uuid::Uuid;

    #[test]
    fn valid_model_serialization() {
        let account_id = [0; 16];
        let operation_id = [1; 16];
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
    fn correct_operation_account_index_mapping() {
        let operation_id = [1; 16];
        let account_id = [0; 16];
        let operation = Operation {
            id: operation_id,
            code: OperationCode::ApproveTransfer,
            created_timestamp: 0,
            last_modification_timestamp: 0,
            decisions: vec![],
            metadata: vec![(
                OPERATION_METADATA_KEY_ACCOUNT_ID.to_string(),
                Uuid::from_bytes(account_id).to_string(),
            )],
            proposed_by: None,
            status: OperationStatus::Pending,
        };

        let index = operation.to_index_for_account();

        assert!(index.is_some());
    }

    #[test]
    #[should_panic]
    fn fail_operation_account_index_on_malformed_metadata() {
        let operation_id = [1; 16];
        let operation = Operation {
            id: operation_id,
            code: OperationCode::ApproveTransfer,
            created_timestamp: 0,
            last_modification_timestamp: 0,
            decisions: vec![],
            metadata: vec![(
                OPERATION_METADATA_KEY_ACCOUNT_ID.to_string(),
                "abcd".to_string(),
            )],
            proposed_by: None,
            status: OperationStatus::Pending,
        };

        operation.to_index_for_account();
    }
}
