use candid::{CandidType, Deserialize};
use ic_canister_core::types::{Timestamp, UUID};
use ic_canister_macros::stable_object;

use super::UserId;

/// The upgrade id, which is a UUID.
pub type UpgradeId = UUID;

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UpgradeStatus {
    Cancelled {
        reason: Option<String>,
    },
    Processing {
        started_at: Timestamp,
    },
    Submitted,
    Pending,
    Completed {
        signature: Option<String>,
        hash: Option<String>,
        completed_at: Timestamp,
    },
    Approved,
    Rejected {
        reason: String,
    },
    Failed {
        reason: String,
    },
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UpgradeExecutionPlan {
    Immediate,
    Scheduled { execution_time: Timestamp },
}

/// Represents an upgrade in the system.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Upgrade {
    /// The upgrade id, which is a UUID.
    pub id: UpgradeId,
    /// The user that initiated the upgrade.
    pub initiator_user: UserId,
    /// The current status of the upgrade.
    pub status: UpgradeStatus,
    /// The expiration date of the upgrade.
    pub expiration_dt: Timestamp,
    /// The execution plan of the upgrade.
    pub execution_plan: UpgradeExecutionPlan,
    /// The transfer metadata (e.g. `memo`, `description`, etc.)
    pub metadata: Vec<(String, String)>,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
    /// The creation timestamp of the upgrade.
    pub created_timestamp: Timestamp,
}
