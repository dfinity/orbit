use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for account errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum AccountError {
    /// The requested account was not found.
    #[error(r#"The requested account was not found."#)]
    AccountNotFound { id: String },
    /// The given blockchain is unknown to the system.
    #[error(r#"The given blockchain is unknown to the system."#)]
    UnknownBlockchain { blockchain: String },
    /// The given blockchain standard is unknown to the system.
    #[error(r#"The given blockchain standard is unknown to the system."#)]
    UnknownBlockchainStandard { blockchain_standard: String },
    /// You don't have the necessary privileges to access the requested account.
    #[error(r#"You don't have the necessary privileges to access the requested account."#)]
    Forbidden,
    /// The account address is out of range.
    #[error(
        r#"The account address is out of range, it must be between {min_length} and {max_length}."#
    )]
    InvalidAddressLength { min_length: u8, max_length: u8 },
    /// The account owners selection is out of range.
    #[error(r#"The account owners selection is out of range, it must be between {min_owners} and {max_owners}."#)]
    InvalidOwnersRange { min_owners: u8, max_owners: u8 },
    /// The requested transfer was not found.
    #[error(r#"The requested transfer was not found."#)]
    TransferNotFound { transfer_id: String },
    /// Fetching account balances can only be done for a maximum of 10 accounts at a time.
    #[error(
        r#"Fetching account balances can only be done for a maximum of {max} accounts at a time."#
    )]
    AccountBalancesBatchRange { min: u8, max: u8 },
    /// The account has failed validation.
    #[error(r#"The account has failed validation."#)]
    ValidationError { info: String },
    /// An account with the given name already exists.
    #[error(r#"An account with the given name already exists."#)]
    AccountNameAlreadyExists,
}

impl DetailableError for AccountError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            AccountError::AccountNotFound { id } => {
                details.insert("id".to_string(), id.to_string());
                Some(details)
            }
            AccountError::UnknownBlockchain { blockchain } => {
                details.insert("blockchain".to_string(), blockchain.to_string());
                Some(details)
            }
            AccountError::ValidationError { info } => {
                details.insert("info".to_string(), info.to_string());
                Some(details)
            }
            AccountError::UnknownBlockchainStandard {
                blockchain_standard,
            } => {
                details.insert(
                    "blockchain_standard".to_string(),
                    blockchain_standard.to_string(),
                );
                Some(details)
            }
            AccountError::InvalidAddressLength {
                min_length,
                max_length,
            } => {
                details.insert("min_length".to_string(), min_length.to_string());
                details.insert("max_length".to_string(), max_length.to_string());
                Some(details)
            }
            AccountError::InvalidOwnersRange {
                min_owners,
                max_owners,
            } => {
                details.insert("min_owners".to_string(), min_owners.to_string());
                details.insert("max_owners".to_string(), max_owners.to_string());
                Some(details)
            }
            AccountError::TransferNotFound { transfer_id } => {
                details.insert("transfer_id".to_string(), transfer_id.to_string());
                Some(details)
            }
            AccountError::AccountBalancesBatchRange { min, max } => {
                details.insert("min".to_string(), min.to_string());
                details.insert("max".to_string(), max.to_string());
                Some(details)
            }
            _ => None,
        }
    }
}
