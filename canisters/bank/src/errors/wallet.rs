use ic_canister_core::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for wallet errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum WalletError {
    /// The requested wallet was not found.
    #[error(r#"The requested wallet was not found."#)]
    WalletNotFound {
        /// The wallet id.
        id: String,
    },
}

impl DetailableError for WalletError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        let WalletError::WalletNotFound { id } = self;
        details.insert("id".to_string(), id.to_string());

        return Some(details);
    }
}
