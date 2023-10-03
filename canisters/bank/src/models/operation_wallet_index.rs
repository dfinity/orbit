use super::{
    Operation, OperationCode, OperationId, OperationStatus, WalletId,
    OPERATION_METADATA_KEY_WALLET_ID,
};
use crate::mappers::HelperMapper;
use candid::{CandidType, Deserialize};
use ic_canister_macros::stable_object;

/// Index of operations by wallet id.
#[stable_object(size = 128)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationWalletIndex {
    /// The wallet id that is associated with this operation.
    pub wallet_id: WalletId,
    /// An operation code that represents the operation type, e.g. "transfer".
    pub code: OperationCode,
    /// The operation id, which is a UUID.
    pub id: OperationId,
}

#[derive(Clone, Debug)]
pub struct OperationWalletIndexCriteria {
    pub wallet_id: WalletId,
    pub code: Option<OperationCode>,
    pub status: Option<OperationStatus>,
    pub read: Option<bool>,
}

impl Operation {
    pub fn as_index_for_wallet(&self) -> OperationWalletIndex {
        let metadata = self.metadata_map();
        let unparsed_wallet_id = metadata
            .get(OPERATION_METADATA_KEY_WALLET_ID)
            .expect("Operation metadata does not contain a transfer id");
        let wallet_id = HelperMapper::default()
            .uuid_from_str(unparsed_wallet_id.to_owned())
            .expect("Failed to parse transfer id");

        OperationWalletIndex {
            id: self.id.to_owned(),
            code: self.code.to_owned(),
            wallet_id: *wallet_id.as_bytes(),
        }
    }
}
