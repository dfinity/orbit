use super::{Transfer, TransferExecutionPlan, TransferId};
use candid::{CandidType, Deserialize};
use ic_canister_core::types::Timestamp;
use ic_canister_macros::stable_object;
use ic_cdk::api::time;
use std::hash::Hash;

/// Represents a transfer queue in the system.
#[stable_object(size = 128)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferQueue {
    /// The time the transfer is scheduled to be executed.
    pub execution_dt: Timestamp,
    /// The transfer id, which is a UUID.
    pub transfer_id: TransferId,
    /// Status of the transfer in the string format.
    pub transfer_status: String,
}

#[stable_object(size = 64)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferQueueKey {
    /// The execution time of the transfer.
    pub execution_dt: Timestamp,
    /// The transfer id, which is a UUID.
    pub transfer_id: TransferId,
}

impl TransferQueue {
    pub fn key(execution_dt: Timestamp, transfer_id: TransferId) -> TransferQueueKey {
        TransferQueueKey {
            execution_dt,
            transfer_id,
        }
    }

    pub fn as_key(&self) -> TransferQueueKey {
        Self::key(self.execution_dt, self.transfer_id)
    }
}

impl Transfer {
    pub fn as_transfer_queue_key(&self) -> TransferQueueKey {
        TransferQueueKey {
            execution_dt: match self.execution_plan {
                TransferExecutionPlan::Immediate => time(),
                TransferExecutionPlan::Scheduled { execution_time } => execution_time,
            },
            transfer_id: self.id,
        }
    }

    pub fn as_transfer_queue_item(&self) -> TransferQueue {
        TransferQueue {
            execution_dt: match self.execution_plan {
                TransferExecutionPlan::Immediate => time(),
                TransferExecutionPlan::Scheduled { execution_time } => execution_time,
            },
            transfer_id: self.id,
            transfer_status: self.status.to_string(),
        }
    }
}
