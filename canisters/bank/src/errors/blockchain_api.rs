use ic_canister_core::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for blockchain api errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum BlockchainApiError {
    /// Failed to fetch latest wallet balance from the asset blockchain.
    #[error(r#"Failed to fetch latest wallet balance from the asset blockchain."#)]
    FetchBalanceFailed { wallet_id: String },
    /// The transaction failed to be submitted.
    #[error(r#"The transaction failed to be submitted."#)]
    TransactionSubmitFailed { info: String },
    /// The communication with the blockchain network returned an error.
    #[error(r#"The communication with the blockchain network returned an error."#)]
    BlockchainNetworkError { info: String },
}

impl DetailableError for BlockchainApiError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            BlockchainApiError::FetchBalanceFailed { wallet_id } => {
                details.insert("wallet_id".to_string(), wallet_id.to_string());
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
        }
    }
}
