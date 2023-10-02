use super::{
    Operation, OperationCode, OperationId, TransferId, OPERATION_METADATA_KEY_TRANSFER_ID,
};
use crate::mappers::HelperMapper;
use candid::{CandidType, Deserialize};
use ic_canister_macros::stable_object;

/// Index of operations by transfer id.
#[stable_object(size = 128)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationTransferIndex {
    /// The transfer id that is associated with this operation.
    pub transfer_id: TransferId,
    /// An operation code that represents the operation type, e.g. "transfer".
    pub code: OperationCode,
    /// The operation id, which is a UUID.
    pub id: OperationId,
}

impl OperationTransferIndex {
    pub fn value(&self) {}
}

impl Operation {
    pub fn as_index_for_transfer(&self) -> OperationTransferIndex {
        let metadata = self.metadata_map();
        let unparsed_transfer_id = metadata
            .get(OPERATION_METADATA_KEY_TRANSFER_ID)
            .expect("Operation metadata does not contain a transfer id");
        let transfer_id = HelperMapper::default()
            .uuid_from_str(unparsed_transfer_id.to_owned())
            .expect("Failed to parse transfer id");

        OperationTransferIndex {
            id: self.id.to_owned(),
            code: self.code.to_owned(),
            transfer_id: *transfer_id.as_bytes(),
        }
    }
}
