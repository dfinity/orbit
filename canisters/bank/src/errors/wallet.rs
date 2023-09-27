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
}

impl DetailableError for WalletError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();

        if let WalletError::WalletNotFound { id } = self {
            details.insert("id".to_string(), id.to_string());
            return Some(details);
        }

        if let WalletError::UnknownBlockchain { blockchain } = self {
            details.insert("blockchain".to_string(), blockchain.to_string());
            return Some(details);
        }

        if let WalletError::UnknownBlockchainStandard {
            blockchain_standard,
        } = self
        {
            details.insert(
                "blockchain_standard".to_string(),
                blockchain_standard.to_string(),
            );
            return Some(details);
        }

        None
    }
}
