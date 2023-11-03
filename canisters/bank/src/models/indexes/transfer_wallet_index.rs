use crate::models::{Transfer, TransferId, WalletId};
use candid::{CandidType, Deserialize};
use ic_canister_core::types::Timestamp;
use ic_canister_macros::stable_object;
use std::hash::Hash;

/// Represents a transfer list index in the system.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferWalletIndex {
    /// The wallet associated with the transfer.
    pub wallet_id: WalletId,
    /// The timestamp of the transfer creation.
    pub created_timestamp: Timestamp,
    /// The transfer associated with the wallet.
    pub transfer_id: TransferId,
}

#[derive(Clone, Debug)]
pub struct TransferWalletIndexCriteria {
    pub wallet_id: WalletId,
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl TransferWalletIndex {
    /// The default criteria interval in nanoseconds (7 days).
    pub const DEFAULT_CRITERIA_INTERVAL_NS: u64 = 7 * 24 * 60 * 60 * 1_000_000_000;
}

impl Transfer {
    pub fn to_index_by_wallet(&self) -> TransferWalletIndex {
        TransferWalletIndex {
            wallet_id: self.from_wallet,
            created_timestamp: self.created_timestamp,
            transfer_id: self.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{PolicySnapshot, TransferExecutionPlan, TransferStatus};
    use num_bigint::BigUint;

    #[test]
    fn test_transfer_to_wallet_index_association() {
        let transfer = Transfer {
            id: [0; 16],
            amount: candid::Nat(BigUint::from(0u32)),
            blockchain_network: "icp".to_string(),
            created_timestamp: 1,
            expiration_dt: 5,
            fee: candid::Nat(BigUint::from(0u32)),
            from_wallet: [1; 16],
            to_address: "0x1234".to_string(),
            status: TransferStatus::Pending,
            initiator_user: [2; 16],
            last_modification_timestamp: 0,
            metadata: vec![],
            policy_snapshot: PolicySnapshot { min_approvals: 0 },
            execution_plan: TransferExecutionPlan::Immediate,
        };

        let index = transfer.to_index_by_wallet();

        assert_eq!(index.transfer_id, transfer.id);
        assert_eq!(index.created_timestamp, 1);
        assert_eq!(index.wallet_id, transfer.from_wallet);
    }
}
