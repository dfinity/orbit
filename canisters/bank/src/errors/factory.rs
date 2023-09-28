use ic_canister_core::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for factory errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum FactoryError {
    /// The selected wallet is not yet supported by the system.
    #[error(r#"The selected wallet is not yet supported by the system."#)]
    UnsupportedBlockchainWallet {
        blockchain: String,
        standard: String,
    },
}

impl DetailableError for FactoryError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        let FactoryError::UnsupportedBlockchainWallet {
            blockchain,
            standard,
        } = self;
        details.insert("blockchain".to_string(), blockchain.to_string());
        details.insert("standard".to_string(), standard.to_string());

        return Some(details);
    }
}
