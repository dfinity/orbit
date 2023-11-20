use crate::models::{AccountId, Transfer, TransferId};
use candid::{CandidType, Deserialize};
use ic_canister_core::types::Timestamp;
use ic_canister_macros::stable_object;
use std::hash::Hash;

/// Represents a transfer list index in the system.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferAccountIndex {
    /// The account associated with the transfer.
    pub account_id: AccountId,
    /// The timestamp of the transfer creation.
    pub created_timestamp: Timestamp,
    /// The transfer associated with the account.
    pub transfer_id: TransferId,
}

#[derive(Clone, Debug)]
pub struct TransferAccountIndexCriteria {
    pub account_id: AccountId,
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl TransferAccountIndex {
    /// The default criteria interval in nanoseconds (7 days).
    pub const DEFAULT_CRITERIA_INTERVAL_NS: u64 = 7 * 24 * 60 * 60 * 1_000_000_000;
}

impl Transfer {
    pub fn to_index_by_account(&self) -> TransferAccountIndex {
        TransferAccountIndex {
            account_id: self.from_account,
            created_timestamp: self.created_timestamp,
            transfer_id: self.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::TransferStatus;
    use num_bigint::BigUint;

    #[test]
    fn test_transfer_to_account_index_association() {
        let transfer = Transfer {
            id: [0; 16],
            amount: candid::Nat(BigUint::from(0u32)),
            blockchain_network: "icp".to_string(),
            created_timestamp: 1,
            fee: candid::Nat(BigUint::from(0u32)),
            from_account: [1; 16],
            to_address: "0x1234".to_string(),
            status: TransferStatus::Created,
            initiator_user: [2; 16],
            last_modification_timestamp: 0,
            metadata: vec![],
        };

        let index = transfer.to_index_by_account();

        assert_eq!(index.transfer_id, transfer.id);
        assert_eq!(index.created_timestamp, 1);
        assert_eq!(index.account_id, transfer.from_account);
    }
}
