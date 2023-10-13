use crate::{
    mappers::HelperMapper,
    models::{Operation, OperationId, TransferId, OPERATION_METADATA_KEY_TRANSFER_ID},
};
use candid::{CandidType, Deserialize};
use ic_canister_core::types::Timestamp;
use ic_canister_macros::stable_object;

/// Index of operations by transfer id.
#[stable_object(size = 128)]
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
    pub fn to_index_for_transfer(&self) -> OperationTransferIndex {
        let metadata = self.metadata_map();
        let unparsed_transfer_id = metadata
            .get(OPERATION_METADATA_KEY_TRANSFER_ID)
            .expect("Operation metadata does not contain a transfer id");
        let transfer_id = HelperMapper::to_uuid(unparsed_transfer_id.to_owned())
            .expect("Failed to parse transfer id");

        OperationTransferIndex {
            id: self.id.to_owned(),
            created_at: self.created_timestamp.to_owned(),
            transfer_id: *transfer_id.as_bytes(),
        }
    }
}
