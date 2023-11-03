use crate::core::ic_cdk::api::time;
use crate::models::{Transfer, TransferExecutionPlan, TransferId};
use candid::{CandidType, Deserialize};
use ic_canister_core::types::Timestamp;
use ic_canister_macros::stable_object;
use std::hash::Hash;

/// Represents a transfer index by execution time.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferExecutionTimeIndex {
    /// The time the transfer is scheduled to be executed.
    pub execution_dt: Timestamp,
    /// The transfer id, which is a UUID.
    pub transfer_id: TransferId,
}

#[derive(Clone, Debug)]
pub struct TransferExecutionTimeIndexCriteria {
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl Transfer {
    pub fn to_index_by_execution_dt(&self) -> TransferExecutionTimeIndex {
        TransferExecutionTimeIndex {
            execution_dt: match self.execution_plan {
                TransferExecutionPlan::Immediate => time(),
                TransferExecutionPlan::Scheduled { execution_time } => execution_time,
            },
            transfer_id: self.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{PolicySnapshot, TransferStatus};
    use num_bigint::BigUint;

    #[test]
    fn test_transfer_to_index_by_execution_dt_scheduled() {
        let transfer = Transfer {
            id: [0; 16],
            amount: candid::Nat(BigUint::from(0u32)),
            blockchain_network: "ethereum".to_string(),
            created_timestamp: 0,
            expiration_dt: 0,
            fee: candid::Nat(BigUint::from(0u32)),
            from_wallet: [1; 16],
            to_address: "0x1234".to_string(),
            status: TransferStatus::Pending,
            initiator_user: [2; 16],
            last_modification_timestamp: 0,
            metadata: vec![],
            policy_snapshot: PolicySnapshot { min_approvals: 0 },
            execution_plan: TransferExecutionPlan::Scheduled { execution_time: 2 },
        };

        let index = transfer.to_index_by_execution_dt();

        assert_eq!(index.transfer_id, transfer.id);
        assert_eq!(index.execution_dt, 2);
    }

    #[test]
    fn test_transfer_to_index_by_execution_dt_immediate() {
        let transfer = Transfer {
            id: [0; 16],
            amount: candid::Nat(BigUint::from(0u32)),
            blockchain_network: "ethereum".to_string(),
            created_timestamp: 0,
            expiration_dt: 0,
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

        let index = transfer.to_index_by_execution_dt();

        assert_eq!(index.transfer_id, transfer.id);
        assert_eq!(index.execution_dt, time());
    }
}
