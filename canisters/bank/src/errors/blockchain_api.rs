use ic_canister_core::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for blockchain api errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum BlockchainApiError {
    /// Failed to fetch latest wallet balance from the asset blockchain.
    #[error(r#"Failed to fetch latest wallet balance from the asset blockchain."#)]
    FetchBalanceFailed { wallet_id: String },
}

impl DetailableError for BlockchainApiError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        let BlockchainApiError::FetchBalanceFailed { wallet_id } = self;
        details.insert("wallet_id".to_string(), wallet_id.to_string());

        Some(details)
    }
}
