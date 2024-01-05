use super::{AccountId, UserId};
use crate::core::ic_cdk::api::time;
use crate::errors::TransferError;
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
use wallet_api::TransferMetadataDTO;

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
    pub metadata: Vec<TransferMetadataDTO>,
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
    pub const MAX_METADATA: u8 = 10;
    pub const MAX_METADATA_KEY_LEN: u8 = 24;
    pub const MAX_METADATA_VALUE_LEN: u8 = 255;

    /// Creates a new transfer key from the given key components.
    pub fn key(id: TransferId) -> TransferKey {
        TransferKey { id }
    }

    pub fn to_key(&self) -> TransferKey {
        Self::key(self.id)
    }

    pub fn metadata_map(&self) -> HashMap<String, String> {
        self.metadata
            .iter()
            .map(|kv| (kv.key.to_owned(), kv.value.to_owned()))
            .collect()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        proposal_id: UUID,
        transfer_id: UUID,
        initiator_user: UUID,
        from_account: UUID,
        to_address: String,
        metadata: Vec<TransferMetadataDTO>,
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

fn validate_metadata(metadata: &Vec<TransferMetadataDTO>) -> ModelValidatorResult<TransferError> {
    if metadata.len() > Transfer::MAX_METADATA as usize {
        return Err(TransferError::ValidationError {
            info: format!(
                "Transfer metadata count exceeds the maximum allowed: {}",
                Transfer::MAX_METADATA
            ),
        });
    }

    for kv in metadata.iter() {
        if kv.key.len() > Transfer::MAX_METADATA_KEY_LEN as usize {
            return Err(TransferError::ValidationError {
                info: format!(
                    "Transfer metadata key length exceeds the maximum allowed: {}",
                    Transfer::MAX_METADATA_KEY_LEN
                ),
            });
        }

        if kv.value.len() > Transfer::MAX_METADATA_VALUE_LEN as usize {
            return Err(TransferError::ValidationError {
                info: format!(
                    "Transfer metadata value length exceeds the maximum allowed: {}",
                    Transfer::MAX_METADATA_VALUE_LEN
                ),
            });
        }
    }

    Ok(())
}

fn validate_to_address(to_address: &String) -> ModelValidatorResult<TransferError> {
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

fn validate_network(blockchain_network: &String) -> ModelValidatorResult<TransferError> {
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
        validate_metadata(&self.metadata)?;
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
    fn test_metadata_validation() {
        let mut transfer = mock_transfer();
        transfer.metadata = vec![TransferMetadataDTO {
            key: "foo".to_string(),
            value: "bar".to_string(),
        }];

        let result = validate_metadata(&transfer.metadata);

        assert!(result.is_ok());
    }

    #[test]
    fn fail_operation_metadata_too_many_entries() {
        let mut transfer = mock_transfer();
        transfer.metadata = vec![
            TransferMetadataDTO {
                key: "foo".to_string(),
                value: "bar".to_string()
            };
            Transfer::MAX_METADATA as usize + 1
        ];

        let result = validate_metadata(&transfer.metadata);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            TransferError::ValidationError {
                info: format!(
                    "Transfer metadata count exceeds the maximum allowed: {}",
                    Transfer::MAX_METADATA
                )
            }
        );
    }

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

    pub fn mock_transfer() -> Transfer {
        Transfer {
            id: [1; 16],
            initiator_user: [0; 16],
            from_account: [0; 16],
            proposal_id: [2; 16],
            to_address: "x".repeat(255),
            status: TransferStatus::Created,
            amount: candid::Nat::from(100),
            fee: candid::Nat::from(0),
            blockchain_network: "a".repeat(50),
            metadata: vec![],
            last_modification_timestamp: time(),
            created_timestamp: time(),
        }
    }
}
