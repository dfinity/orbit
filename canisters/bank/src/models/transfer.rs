use super::{AccountId, ApprovalThresholdPolicy, Wallet, WalletId, WalletPolicy};
use crate::errors::TransferError;
use candid::{CandidType, Deserialize};
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
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

impl Display for TransferStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TransferStatus::Cancelled { .. } => write!(f, "cancelled"),
            TransferStatus::Submitted => write!(f, "submitted"),
            TransferStatus::Pending => write!(f, "pending"),
            TransferStatus::Processing { .. } => write!(f, "processing"),
            TransferStatus::Completed { .. } => write!(f, "completed"),
            TransferStatus::Approved => write!(f, "approved"),
            TransferStatus::Rejected { .. } => write!(f, "rejected"),
            TransferStatus::Failed { .. } => write!(f, "failed"),
        }
    }
}

#[stable_object(size = 256)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PolicySnapshot {
    /// The minimum number of approvals required for the transfer to be approved.
    pub min_approvals: u8,
}

/// Represents a transfer in the system.
#[stable_object(size = 2048)]
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
    /// The transfer policies, which define the rules for the transfer.
    ///
    /// It holds a snapshot of the wallet policies at the time of the transfer creation.
    pub policy_snapshot: PolicySnapshot,
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

    pub fn make_policy_snapshot(&mut self, wallet: &Wallet) {
        let mut policy_snapshot = PolicySnapshot { min_approvals: 1 };

        for policy in wallet.policies.iter() {
            match policy {
                WalletPolicy::ApprovalThreshold(threshold) => match threshold {
                    ApprovalThresholdPolicy::FixedThreshold(min_approvals) => {
                        policy_snapshot.min_approvals = *min_approvals;
                    }
                    ApprovalThresholdPolicy::VariableThreshold(percentage) => {
                        policy_snapshot.min_approvals = ((wallet.owners.len() as f64
                            * (*percentage as f64 / 100.0))
                            .ceil() as u8)
                            .max(1);
                    }
                },
            }
        }

        self.policy_snapshot = policy_snapshot;
    }
}

pub struct TransferValidator<'model> {
    transfer: &'model Transfer,
}

impl<'model> TransferValidator<'model> {
    pub const ADDRESS_RANGE: (u8, u8) = (1, 255);
    pub const NETWORK_RANGE: (u8, u8) = (1, 50);
    pub const MAX_METADATA: u8 = 10;
    pub const MAX_METADATA_KEY_LEN: u8 = 24;
    pub const MAX_METADATA_VALUE_LEN: u8 = 255;

    pub fn new(transfer: &'model Transfer) -> Self {
        Self { transfer }
    }

    pub fn validate_metadata(&self) -> ModelValidatorResult<TransferError> {
        if self.transfer.metadata.len() > Self::MAX_METADATA as usize {
            return Err(TransferError::ValidationError {
                info: format!(
                    "Transfer metadata count exceeds the maximum allowed: {}",
                    Self::MAX_METADATA
                ),
            });
        }

        for (key, value) in self.transfer.metadata.iter() {
            if key.len() > Self::MAX_METADATA_KEY_LEN as usize {
                return Err(TransferError::ValidationError {
                    info: format!(
                        "Transfer metadata key length exceeds the maximum allowed: {}",
                        Self::MAX_METADATA_KEY_LEN
                    ),
                });
            }

            if value.len() > Self::MAX_METADATA_VALUE_LEN as usize {
                return Err(TransferError::ValidationError {
                    info: format!(
                        "Transfer metadata value length exceeds the maximum allowed: {}",
                        Self::MAX_METADATA_VALUE_LEN
                    ),
                });
            }
        }

        Ok(())
    }

    pub fn validate_to_address(&self) -> ModelValidatorResult<TransferError> {
        if (self.transfer.to_address.len() < Self::ADDRESS_RANGE.0 as usize)
            || (self.transfer.to_address.len() > Self::ADDRESS_RANGE.1 as usize)
        {
            return Err(TransferError::ValidationError {
                info: format!(
                    "Transfer destination address length exceeds the allowed range: {} to {}",
                    Self::ADDRESS_RANGE.0,
                    Self::ADDRESS_RANGE.1
                ),
            });
        }

        Ok(())
    }

    pub fn validate_network(&self) -> ModelValidatorResult<TransferError> {
        if (self.transfer.blockchain_network.len() < Self::NETWORK_RANGE.0 as usize)
            || (self.transfer.blockchain_network.len() > Self::NETWORK_RANGE.1 as usize)
        {
            return Err(TransferError::ValidationError {
                info: format!(
                    "Transfer network length exceeds the allowed range: {} to {}",
                    Self::NETWORK_RANGE.0,
                    Self::NETWORK_RANGE.1
                ),
            });
        }

        Ok(())
    }

    pub fn validate_expiration_dt(&self) -> ModelValidatorResult<TransferError> {
        if let TransferExecutionPlan::Scheduled { execution_time } = &self.transfer.execution_plan {
            if self.transfer.expiration_dt < *execution_time {
                return Err(TransferError::ValidationError {
                    info:
                        "Transfer expiration date must be greater then the planned execution_time"
                            .to_string(),
                });
            }
        }

        Ok(())
    }

    pub fn validate(&self) -> ModelValidatorResult<TransferError> {
        self.validate_metadata()?;
        self.validate_to_address()?;
        self.validate_network()?;
        self.validate_expiration_dt()?;

        Ok(())
    }
}

impl ModelValidator<TransferError> for Transfer {
    fn validate(&self) -> ModelValidatorResult<TransferError> {
        TransferValidator::new(self).validate()
    }
}
