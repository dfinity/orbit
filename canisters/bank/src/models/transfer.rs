use super::{AccountId, WalletId};
use candid::{CandidType, Deserialize};
use ic_canister_core::{
    model::ModelValidator,
    types::{Timestamp, UUID},
};
use ic_canister_macros::stable_object;
use ic_cdk::api::time;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    hash::Hash,
};

pub const METADATA_MEMO_KEY: &str = "memo";

/// The transfer id, which is a UUID.
pub type TransferId = UUID;

#[stable_object(size = 128)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TransferExecutionPlan {
    Immediate,
    Scheduled { execution_time: Timestamp },
}

#[stable_object(size = 256)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TransferStatus {
    Cancelled {
        reason: Option<String>,
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
}

impl Display for TransferStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TransferStatus::Cancelled { .. } => write!(f, "cancelled"),
            TransferStatus::Submitted => write!(f, "submitted"),
            TransferStatus::Pending => write!(f, "pending"),
            TransferStatus::Completed { .. } => write!(f, "completed"),
            TransferStatus::Approved => write!(f, "approved"),
            TransferStatus::Rejected { .. } => write!(f, "rejected"),
        }
    }
}

/// Represents a transfer in the system.
#[stable_object(size = 1024)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Transfer {
    /// The transfer id, which is a UUID.
    pub id: TransferId,
    /// The account that initiated the transfer.
    pub initiator_account: AccountId,
    /// The wallet id that the transfer is from.
    pub from_wallet: WalletId,
    /// The destination address of the transfer.
    pub to_address: String,
    /// The current status of the transfer.
    pub status: TransferStatus,
    /// The amount of the transfer.
    pub amount: candid::Nat,
    /// The fee of the transfer.
    pub fee: candid::Nat,
    /// The expiration date of the transfer.
    pub expiration_dt: Timestamp,
    /// The execution plan of the transfer.
    pub execution_plan: TransferExecutionPlan,
    /// The blockchain network that the transfer will be executed on.
    pub blockchain_network: String,
    /// The transfer metadata (e.g. `memo`, `description`, etc.)
    pub metadata: Vec<(String, String)>,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
    /// The creation timestamp of the transfer.
    pub created_timestamp: Timestamp,
}

#[stable_object(size = 64)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferKey {
    /// The transfer id, which is a UUID.
    pub id: TransferId,
}

impl Transfer {
    /// Creates a new transfer key from the given key components.
    pub fn key(id: TransferId) -> TransferKey {
        TransferKey { id }
    }

    pub fn as_key(&self) -> TransferKey {
        Self::key(self.id)
    }

    pub fn metadata_map(&self) -> HashMap<String, String> {
        self.metadata
            .iter()
            .map(|(key, value)| (key.to_owned(), value.to_owned()))
            .collect()
    }

    pub fn default_expiration_dt() -> Timestamp {
        let five_days_in_ns: u64 = 5 * 24 * 60 * 60 * 1_000_000_000;

        time() + five_days_in_ns
    }
}

// TODO: add validation logic.

impl ModelValidator for Transfer {}
