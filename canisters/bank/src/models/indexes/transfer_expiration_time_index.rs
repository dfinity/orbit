use crate::models::{Transfer, TransferId};
use candid::{CandidType, Deserialize};
use ic_canister_core::types::Timestamp;
use ic_canister_macros::stable_object;
use std::hash::Hash;

/// Represents a transfer index by execution time.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferExpirationTimeIndex {
    /// The time the transfer is scheduled to be set as expired if not executed.
    pub expiration_dt: Timestamp,
    /// The transfer id, which is a UUID.
    pub transfer_id: TransferId,
}

#[derive(Clone, Debug)]
pub struct TransferExpirationTimeIndexCriteria {
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl Transfer {
    pub fn to_index_by_expiration_dt(&self) -> TransferExpirationTimeIndex {
        TransferExpirationTimeIndex {
            expiration_dt: self.expiration_dt,
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
    fn test_transfer_to_index_by_expiration_dt() {
        let transfer = Transfer {
            id: [0; 16],
            amount: candid::Nat(BigUint::from(0u32)),
            blockchain_network: "icp".to_string(),
            created_timestamp: 0,
            expiration_dt: 5,
            fee: candid::Nat(BigUint::from(0u32)),
            from_wallet: [1; 16],
            to_address: "0x1234".to_string(),
            status: TransferStatus::Pending,
            initiator_account: [2; 16],
            last_modification_timestamp: 0,
            metadata: vec![],
            policy_snapshot: PolicySnapshot { min_approvals: 0 },
            execution_plan: TransferExecutionPlan::Immediate,
        };

        let index = transfer.to_index_by_expiration_dt();

        assert_eq!(index.transfer_id, transfer.id);
        assert_eq!(index.expiration_dt, 5);
    }
}
