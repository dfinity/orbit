use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for factory errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum FactoryError {
    /// The selected account is not yet supported by the system.
    #[error(r#"The selected account is not yet supported by the system."#)]
    UnsupportedBlockchainAccount {
        blockchain: String,
        standard: String,
    },
}

impl DetailableError for FactoryError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        let FactoryError::UnsupportedBlockchainAccount {
            blockchain,
            standard,
        } = self;
        details.insert("blockchain".to_string(), blockchain.to_string());
        details.insert("standard".to_string(), standard.to_string());

        Some(details)
    }
}
