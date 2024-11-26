use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for factory errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum FactoryError {
    /// The selected account is not yet supported by the system.
    #[error(r#"The selected blockchain is not yet supported by the system."#)]
    UnsupportedBlockchain { blockchain: String },
}

impl DetailableError for FactoryError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        let FactoryError::UnsupportedBlockchain { blockchain } = self;
        details.insert("blockchain".to_string(), blockchain.to_string());

        Some(details)
    }
}
