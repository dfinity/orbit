use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for blockchain api errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum BlockchainApiError {
    /// Failed to fetch latest account balance from the asset blockchain.
    #[error(r#"Failed to fetch latest account balance from the asset blockchain."#)]
    FetchBalanceFailed { account_id: String },
    /// The transaction failed to be submitted.
    #[error(r#"The transaction failed to be submitted."#)]
    TransactionSubmitFailed { info: String },
    /// The communication with the blockchain network returned an error.
    #[error(r#"The communication with the blockchain network returned an error."#)]
    BlockchainNetworkError { info: String },
    /// The to address is invalid.
    #[error("The to address '{address}' is invalid: {error}")]
    InvalidToAddress { address: String, error: String },
}

impl DetailableError for BlockchainApiError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            BlockchainApiError::FetchBalanceFailed { account_id } => {
                details.insert("account_id".to_string(), account_id.to_string());
                Some(details)
            }
            BlockchainApiError::TransactionSubmitFailed { info } => {
                details.insert("info".to_string(), info.to_string());
                Some(details)
            }
            BlockchainApiError::BlockchainNetworkError { info } => {
                details.insert("info".to_string(), info.to_string());
                Some(details)
            }
            BlockchainApiError::InvalidToAddress { address, error } => {
                details.insert("address".to_string(), address.to_string());
                details.insert("error".to_string(), error.to_string());
                Some(details)
            }
        }
    }
}
