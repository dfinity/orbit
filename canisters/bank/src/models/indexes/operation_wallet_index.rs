use crate::mappers::HelperMapper;
use crate::models::{Operation, OperationId, WalletId, OPERATION_METADATA_KEY_WALLET_ID};
use candid::{CandidType, Deserialize};
use ic_canister_core::types::Timestamp;
use ic_canister_macros::stable_object;

/// Index of operations by wallet id.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationWalletIndex {
    /// The wallet id that is associated with this operation.
    pub wallet_id: WalletId,
    /// The time when the operation was created.
    pub created_at: Timestamp,
    /// The operation id, which is a UUID.
    pub id: OperationId,
}

#[derive(Clone, Debug)]
pub struct OperationWalletIndexCriteria {
    pub wallet_id: WalletId,
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl Operation {
    pub fn to_index_for_wallet(&self) -> Option<OperationWalletIndex> {
        let metadata = self.metadata_map();
        if let Some(unparsed_wallet_id) = metadata.get(OPERATION_METADATA_KEY_WALLET_ID) {
            let wallet_id = HelperMapper::to_uuid(unparsed_wallet_id.to_owned())
                .expect("Failed to parse wallet id");

            return Some(OperationWalletIndex {
                id: self.id.to_owned(),
                created_at: self.created_timestamp.to_owned(),
                wallet_id: *wallet_id.as_bytes(),
            });
        }

        // This operation is not related to a wallet.
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
        let wallet_id = [0; 16];
        let operation_id = [1; 16];
        let model = OperationWalletIndex {
            id: operation_id,
            wallet_id,
            created_at: 0,
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = OperationWalletIndex::from_bytes(serialized_model);

        assert_eq!(model.id, deserialized_model.id);
        assert_eq!(model.wallet_id, deserialized_model.wallet_id);
    }

    #[test]
    fn correct_operation_wallet_index_mapping() {
        let operation_id = [1; 16];
        let wallet_id = [0; 16];
        let operation = Operation {
            id: operation_id,
            code: OperationCode::ApproveTransfer,
            created_timestamp: 0,
            last_modification_timestamp: 0,
            decisions: vec![],
            metadata: vec![(
                OPERATION_METADATA_KEY_WALLET_ID.to_string(),
                Uuid::from_bytes(wallet_id).to_string(),
            )],
            originator_account_id: None,
            status: OperationStatus::Pending,
        };

        let index = operation.to_index_for_wallet();

        assert!(index.is_some());
    }

    #[test]
    #[should_panic]
    fn fail_operation_wallet_index_on_malformed_metadata() {
        let operation_id = [1; 16];
        let operation = Operation {
            id: operation_id,
            code: OperationCode::ApproveTransfer,
            created_timestamp: 0,
            last_modification_timestamp: 0,
            decisions: vec![],
            metadata: vec![(
                OPERATION_METADATA_KEY_WALLET_ID.to_string(),
                "abcd".to_string(),
            )],
            originator_account_id: None,
            status: OperationStatus::Pending,
        };

        operation.to_index_for_wallet();
    }
}
