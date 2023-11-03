use crate::{
    mappers::HelperMapper,
    models::{Operation, OperationId, TransferId, OPERATION_METADATA_KEY_TRANSFER_ID},
};
use candid::{CandidType, Deserialize};
use ic_canister_core::types::Timestamp;
use ic_canister_macros::stable_object;

/// Index of operations by transfer id.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationTransferIndex {
    /// The transfer id that is associated with this operation.
    pub transfer_id: TransferId,
    /// The time when the operation was created.
    pub created_at: Timestamp,
    /// The operation id, which is a UUID.
    pub id: OperationId,
}

#[derive(Clone, Debug)]
pub struct OperationTransferIndexCriteria {
    pub transfer_id: TransferId,
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl Operation {
    pub fn to_index_for_transfer(&self) -> Option<OperationTransferIndex> {
        let metadata = self.metadata_map();
        if let Some(unparsed_transfer_id) = metadata.get(OPERATION_METADATA_KEY_TRANSFER_ID) {
            let transfer_id = HelperMapper::to_uuid(unparsed_transfer_id.to_owned())
                .expect("Failed to parse transfer id");

            return Some(OperationTransferIndex {
                id: self.id.to_owned(),
                created_at: self.created_timestamp.to_owned(),
                transfer_id: *transfer_id.as_bytes(),
            });
        }

        // This operation is not related to a transfer.
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
        let transfer_id = [0; 16];
        let operation_id = [1; 16];
        let model = OperationTransferIndex {
            id: operation_id,
            transfer_id,
            created_at: 0,
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = OperationTransferIndex::from_bytes(serialized_model);

        assert_eq!(model.id, deserialized_model.id);
        assert_eq!(model.transfer_id, deserialized_model.transfer_id);
    }

    #[test]
    fn correct_operation_transfer_index_mapping() {
        let operation_id = [1; 16];
        let transfer_id = [0; 16];
        let operation = Operation {
            id: operation_id,
            code: OperationCode::ApproveTransfer,
            created_timestamp: 0,
            last_modification_timestamp: 0,
            decisions: vec![],
            metadata: vec![(
                OPERATION_METADATA_KEY_TRANSFER_ID.to_string(),
                Uuid::from_bytes(transfer_id).to_string(),
            )],
            proposed_by: None,
            status: OperationStatus::Pending,
        };

        let index = operation.to_index_for_transfer();

        assert!(index.is_some());
    }

    #[test]
    #[should_panic]
    fn fail_operation_transfer_index_on_malformed_metadata() {
        let operation_id = [1; 16];
        let operation = Operation {
            id: operation_id,
            code: OperationCode::ApproveTransfer,
            created_timestamp: 0,
            last_modification_timestamp: 0,
            decisions: vec![],
            metadata: vec![(
                OPERATION_METADATA_KEY_TRANSFER_ID.to_string(),
                "abcd".to_string(),
            )],
            proposed_by: None,
            status: OperationStatus::Pending,
        };

        operation.to_index_for_transfer();
    }
}
