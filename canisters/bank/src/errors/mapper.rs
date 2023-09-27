use ic_canister_core::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for mapper errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum MapperError {
    /// The given blockchain is unknown to the system.
    #[error(r#"The given blockchain is unknown to the system."#)]
    UnknownBlockchain { blockchain: String },
    /// The given blockchain standard is unknown to the system.
    #[error(r#"The given blockchain standard is unknown to the system."#)]
    UnknownBlockchainStandard { blockchain_standard: String },
    /// The selected standard is not supported by the given blockchain.
    #[error(r#"The selected standard is not supported by the given blockchain."#)]
    UnsupportedBlockchainStandard {
        blockchain: String,
        supported_standards: Vec<String>,
    },
    /// Cannot set the symbol for native wallets.
    #[error(r#"Cannot set the symbol for native wallets."#)]
    NativeWalletSymbolMetadataNotAllowed,
    /// Wallets for non native assets are required to have a defined token symbol.
    #[error(r#"Wallets for non native assets are required to have a defined token symbol."#)]
    NonNativeWalletSymbolRequired,
}

impl DetailableError for MapperError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();

        if let MapperError::UnknownBlockchain { blockchain } = self {
            details.insert("blockchain".to_string(), blockchain.to_string());
            return Some(details);
        }

        if let MapperError::UnknownBlockchainStandard {
            blockchain_standard,
        } = self
        {
            details.insert(
                "blockchain_standard".to_string(),
                blockchain_standard.to_string(),
            );
            return Some(details);
        }

        if let MapperError::UnsupportedBlockchainStandard {
            blockchain,
            supported_standards,
        } = self
        {
            details.insert("blockchain".to_string(), blockchain.to_string());
            details.insert(
                "supported_standards".to_string(),
                supported_standards.join(",").to_string(),
            );
            return Some(details);
        }

        None
    }
}
