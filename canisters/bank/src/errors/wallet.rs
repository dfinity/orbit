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
    #[error(r#"You don't have the necessary privileges to access the requested wallet."#)]
    Forbidden { wallet: String },
}

impl DetailableError for WalletError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            WalletError::Forbidden { wallet } => {
                details.insert("wallet".to_string(), wallet.to_string());
                Some(details)
            }
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
        }
    }
}
