use ic_canister_core::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for wallet errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum WalletError {
    /// The requested wallet was not found.
    #[error(r#"The requested wallet was not found."#)]
    WalletNotFound { id: String },
    /// The given blockchain is unknown to the system.
    #[error(r#"The given blockchain is unknown to the system."#)]
    UnknownBlockchain { blockchain: String },
    /// The given blockchain standard is unknown to the system.
    #[error(r#"The given blockchain standard is unknown to the system."#)]
    UnknownBlockchainStandard { blockchain_standard: String },
    /// You don't have the necessary privileges to access the requested wallet.
    #[error(r#"You don't have the necessary privileges to access the requested wallet."#)]
    Forbidden,
    /// The wallet address is out of range.
    #[error(
        r#"The wallet address is out of range, it must be between {min_length} and {max_length}."#
    )]
    InvalidAddressLength { min_length: u8, max_length: u8 },
    /// The wallet owners selection is out of range.
    #[error(r#"The wallet owners selection is out of range, it must be between {min_owners} and {max_owners}."#)]
    InvalidOwnersRange { min_owners: u8, max_owners: u8 },
    /// The requested transfer was not found.
    #[error(r#"The requested transfer was not found."#)]
    TransferNotFound { transfer_id: String },
    /// Fetching wallet balances can only be done for a maximum of 10 wallets at a time.
    #[error(
        r#"Fetching wallet balances can only be done for a maximum of {max} wallets at a time."#
    )]
    WalletBalancesBatchRange { min: u8, max: u8 },
}

impl DetailableError for WalletError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            WalletError::WalletNotFound { id } => {
                details.insert("id".to_string(), id.to_string());
                Some(details)
            }
            WalletError::UnknownBlockchain { blockchain } => {
                details.insert("blockchain".to_string(), blockchain.to_string());
                Some(details)
            }
            WalletError::UnknownBlockchainStandard {
                blockchain_standard,
            } => {
                details.insert(
                    "blockchain_standard".to_string(),
                    blockchain_standard.to_string(),
                );
                Some(details)
            }
            WalletError::InvalidAddressLength {
                min_length,
                max_length,
            } => {
                details.insert("min_length".to_string(), min_length.to_string());
                details.insert("max_length".to_string(), max_length.to_string());
                Some(details)
            }
            WalletError::InvalidOwnersRange {
                min_owners,
                max_owners,
            } => {
                details.insert("min_owners".to_string(), min_owners.to_string());
                details.insert("max_owners".to_string(), max_owners.to_string());
                Some(details)
            }
            WalletError::TransferNotFound { transfer_id } => {
                details.insert("transfer_id".to_string(), transfer_id.to_string());
                Some(details)
            }
            WalletError::WalletBalancesBatchRange { min, max } => {
                details.insert("min".to_string(), min.to_string());
                details.insert("max".to_string(), max.to_string());
                Some(details)
            }
            _ => None,
        }
    }
}
