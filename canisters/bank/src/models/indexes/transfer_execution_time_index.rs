use crate::models::{Transfer, TransferExecutionPlan, TransferId};
use candid::{CandidType, Deserialize};
use ic_canister_core::types::Timestamp;
use ic_canister_macros::stable_object;
use ic_cdk::api::time;
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
