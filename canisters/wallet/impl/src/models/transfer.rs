use super::{AccountId, AccountKey, ProposalKey, UserId, UserKey};
use crate::errors::TransferError;
use crate::models::Metadata;
use crate::repositories::{ACCOUNT_REPOSITORY, PROPOSAL_REPOSITORY};
use crate::{core::ic_cdk::api::time, repositories::USER_REPOSITORY};
use ic_canister_core::repository::Repository;
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use ic_canister_macros::storable;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    hash::Hash,
};

pub const METADATA_MEMO_KEY: &str = "memo";

/// The transfer id, which is a UUID.
pub type TransferId = UUID;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

fn validate_initiator_user(initiator_user: &UserId) -> ModelValidatorResult<TransferError> {
    USER_REPOSITORY
        .get(&UserKey {
            id: *initiator_user,
        })
        .ok_or(TransferError::ValidationError {
            info: "The initiator_user does not exist".to_owned(),
        })?;
    Ok(())
}

fn validate_from_account(from_account: &AccountId) -> ModelValidatorResult<TransferError> {
    ACCOUNT_REPOSITORY
        .get(&AccountKey { id: *from_account })
        .ok_or(TransferError::ValidationError {
            info: "The from_account does not exist".to_owned(),
        })?;
    Ok(())
}

fn validate_proposal_id(proposal_id: &UUID) -> ModelValidatorResult<TransferError> {
    PROPOSAL_REPOSITORY
        .get(&ProposalKey { id: *proposal_id })
        .ok_or(TransferError::ValidationError {
            info: "The proposal_id does not exist".to_owned(),
        })?;
    Ok(())
}

impl ModelValidator<TransferError> for Transfer {
    fn validate(&self) -> ModelValidatorResult<TransferError> {
        self.metadata.validate()?;
        validate_to_address(&self.to_address)?;
        validate_network(&self.blockchain_network)?;

        validate_initiator_user(&self.initiator_user)?;
        validate_from_account(&self.from_account)?;
        validate_proposal_id(&self.proposal_id)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{
        account_test_utils::mock_account, proposal_test_utils::mock_proposal,
        user_test_utils::mock_user,
    };

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

    #[test]
    fn fail_initiator_user_missing() {
        assert!(validate_initiator_user(&[0; 16]).is_err());
    }

    #[test]
    fn test_initiator_user_exists() {
        let user = mock_user();
        USER_REPOSITORY.insert(user.to_key(), user.clone());
        let result = validate_initiator_user(&user.id);
        assert!(result.is_ok());
    }

    #[test]
    fn fail_from_account_missing() {
        assert!(validate_from_account(&[0; 16]).is_err());
    }

    #[test]
    fn test_from_account_exists() {
        let account = mock_account();
        ACCOUNT_REPOSITORY.insert(account.to_key(), account.clone());
        let result = validate_from_account(&account.id);
        assert!(result.is_ok());
    }

    #[test]
    fn fail_proposal_id_missing() {
        assert!(validate_proposal_id(&[0; 16]).is_err());
    }

    #[test]
    fn test_proposal_id_exists() {
        let proposal = mock_proposal();
        PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.clone());
        let result = validate_proposal_id(&proposal.id);
        assert!(result.is_ok());
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
