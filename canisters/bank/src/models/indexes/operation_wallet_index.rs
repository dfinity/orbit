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
    pub fn to_index_for_wallet(&self) -> OperationWalletIndex {
        let metadata = self.metadata_map();
        let unparsed_wallet_id = metadata
            .get(OPERATION_METADATA_KEY_WALLET_ID)
            .expect("Operation metadata does not contain a transfer id");
        let wallet_id = HelperMapper::to_uuid(unparsed_wallet_id.to_owned())
            .expect("Failed to parse transfer id");

        OperationWalletIndex {
            id: self.id.to_owned(),
            created_at: self.created_timestamp.to_owned(),
            wallet_id: *wallet_id.as_bytes(),
        }
    }
}
