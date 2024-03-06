use super::{AccountId, UserId};
use crate::core::ic_cdk::api::time;
use crate::errors::TransferError;
use crate::models::Metadata;
use candid::{CandidType, Deserialize};
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use ic_canister_macros::stable_object;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    hash::Hash,
};

pub const METADATA_MEMO_KEY: &str = "memo";

/// The transfer id, which is a UUID.
pub type TransferId = UUID;

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TransferStatus {
    Created,
    Processing {
        started_at: Timestamp,
    },
    Completed {
        signature: Option<String>,
        hash: Option<String>,
        completed_at: Timestamp,
    },
    Failed {
        reason: String,
    },
}

impl Display for TransferStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TransferStatus::Created => write!(f, "created"),
            TransferStatus::Processing { .. } => write!(f, "processing"),
            TransferStatus::Completed { .. } => write!(f, "completed"),
            TransferStatus::Failed { .. } => write!(f, "failed"),
        }
    }
}

/// Represents a transfer in the system.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Transfer {
    /// The transfer id, which is a UUID.
    pub id: TransferId,
    /// The user that initiated the transfer.
    pub initiator_user: UserId,
    /// The account id that the transfer is from.
    pub from_account: AccountId,
    /// The destination address of the transfer.
    pub to_address: String,
    /// The current status of the transfer.
    pub status: TransferStatus,
    /// The amount of the transfer.
    pub amount: candid::Nat,
    /// The proposal id that the transfer is associated with.
    pub proposal_id: UUID,
    /// The fee of the transfer.
    pub fee: candid::Nat,
    /// The blockchain network that the transfer will be executed on.
    pub blockchain_network: String,
    /// The transfer metadata (e.g. `memo`, `description`, etc.)
    pub metadata: Metadata,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
    /// The creation timestamp of the transfer.
    pub created_timestamp: Timestamp,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferKey {
    /// The transfer id, which is a UUID.
    pub id: TransferId,
}

impl Transfer {
    pub const ADDRESS_RANGE: (u8, u8) = (1, 255);
    pub const NETWORK_RANGE: (u8, u8) = (1, 50);

    /// Creates a new transfer key from the given key components.
    pub fn key(id: TransferId) -> TransferKey {
        TransferKey { id }
    }

    pub fn to_key(&self) -> TransferKey {
        Self::key(self.id)
    }

    pub fn metadata_map(&self) -> HashMap<String, String> {
        self.metadata.map()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        proposal_id: UUID,
        transfer_id: UUID,
        initiator_user: UUID,
        from_account: UUID,
        to_address: String,
        metadata: Metadata,
        amount: candid::Nat,
        fee: candid::Nat,
        blockchain_network: String,
    ) -> Self {
        Self {
            id: transfer_id,
            initiator_user,
            from_account,
            to_address,
            proposal_id,
            status: TransferStatus::Created,
            amount,
            fee,
            blockchain_network,
            metadata,
            last_modification_timestamp: time(),
            created_timestamp: time(),
        }
    }
}

fn validate_to_address(to_address: &str) -> ModelValidatorResult<TransferError> {
    if (to_address.len() < Transfer::ADDRESS_RANGE.0 as usize)
        || (to_address.len() > Transfer::ADDRESS_RANGE.1 as usize)
    {
        return Err(TransferError::ValidationError {
            info: format!(
                "Transfer destination address length exceeds the allowed range: {} to {}",
                Transfer::ADDRESS_RANGE.0,
                Transfer::ADDRESS_RANGE.1
            ),
        });
    }

    Ok(())
}

fn validate_network(blockchain_network: &str) -> ModelValidatorResult<TransferError> {
    if (blockchain_network.len() < Transfer::NETWORK_RANGE.0 as usize)
        || (blockchain_network.len() > Transfer::NETWORK_RANGE.1 as usize)
    {
        return Err(TransferError::ValidationError {
            info: format!(
                "Transfer network length exceeds the allowed range: {} to {}",
                Transfer::NETWORK_RANGE.0,
                Transfer::NETWORK_RANGE.1
            ),
        });
    }

    Ok(())
}

impl ModelValidator<TransferError> for Transfer {
    fn validate(&self) -> ModelValidatorResult<TransferError> {
        self.metadata.validate()?;
        validate_to_address(&self.to_address)?;
        validate_network(&self.blockchain_network)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use transfer_test_utils::mock_transfer;

    #[test]
    fn test_address_validation() {
        let mut transfer = mock_transfer();
        transfer.to_address = "a".repeat(255);

        let result = validate_to_address(&transfer.to_address);

        assert!(result.is_ok());
    }

    #[test]
    fn fail_address_too_long() {
        let mut transfer = mock_transfer();
        transfer.to_address = "a".repeat(Transfer::ADDRESS_RANGE.1 as usize + 1);

        let result = validate_to_address(&transfer.to_address);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            TransferError::ValidationError {
                info: format!(
                    "Transfer destination address length exceeds the allowed range: {} to {}",
                    Transfer::ADDRESS_RANGE.0,
                    Transfer::ADDRESS_RANGE.1
                )
            }
        );
    }

    #[test]
    fn fail_address_too_short() {
        let mut transfer = mock_transfer();
        transfer.to_address = "".to_string();

        let result = validate_to_address(&transfer.to_address);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            TransferError::ValidationError {
                info: format!(
                    "Transfer destination address length exceeds the allowed range: {} to {}",
                    Transfer::ADDRESS_RANGE.0,
                    Transfer::ADDRESS_RANGE.1
                )
            }
        );
    }

    #[test]
    fn test_network_validation() {
        let mut transfer = mock_transfer();
        transfer.blockchain_network = "icp:mainnet".to_string();

        let result = validate_network(&transfer.blockchain_network);

        assert!(result.is_ok());
    }

    #[test]
    fn fail_network_too_long() {
        let mut transfer = mock_transfer();
        transfer.blockchain_network = "a".repeat(Transfer::NETWORK_RANGE.1 as usize + 1);

        let result = validate_network(&transfer.blockchain_network);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            TransferError::ValidationError {
                info: format!(
                    "Transfer network length exceeds the allowed range: {} to {}",
                    Transfer::NETWORK_RANGE.0,
                    Transfer::NETWORK_RANGE.1
                )
            }
        );
    }

    #[test]
    fn fail_network_too_short() {
        let mut transfer = mock_transfer();
        transfer.blockchain_network = "".to_string();

        let result = validate_network(&transfer.blockchain_network);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            TransferError::ValidationError {
                info: format!(
                    "Transfer network length exceeds the allowed range: {} to {}",
                    Transfer::NETWORK_RANGE.0,
                    Transfer::NETWORK_RANGE.1
                )
            }
        );
    }
}

#[cfg(test)]
pub mod transfer_test_utils {
    use super::*;
    use uuid::Uuid;

    pub fn mock_transfer() -> Transfer {
        Transfer {
            id: *Uuid::new_v4().as_bytes(),
            initiator_user: [0; 16],
            from_account: [0; 16],
            proposal_id: [2; 16],
            to_address: "x".repeat(255),
            status: TransferStatus::Created,
            amount: candid::Nat::from(100_u64),
            fee: candid::Nat::from(0_u64),
            blockchain_network: "a".repeat(50),
            metadata: Metadata::default(),
            last_modification_timestamp: time(),
            created_timestamp: time(),
        }
    }
}
