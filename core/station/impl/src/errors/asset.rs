use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for asset errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum AssetError {
    /// The asset was not found.
    #[error("The asset with id {id} was not found.")]
    NotFound {
        /// The asset id.
        id: String,
    },
    /// Invalid decimals value.
    #[error(r#"Decimals must be between {min} and {max}."#)]
    InvalidDecimals { min: u32, max: u32 },
    /// Invalid name length.
    #[error(r#"Name must be between {min_length} and {max_length}."#)]
    InvalidNameLength { min_length: u16, max_length: u16 },
    /// Invalid symbol length.
    #[error(r#"Symbol must be between {min_length} and {max_length}."#)]
    InvalidSymbolLength { min_length: u16, max_length: u16 },
    /// The given blockchain is unknown to the system.
    #[error(r#"The given blockchain is unknown to the system."#)]
    UnknownBlockchain { blockchain: String },
    /// The given blockchain standard is unknown to the system.
    #[error(r#"The given blockchain standard is unknown to the system."#)]
    UnknownBlockchainStandard { blockchain_standard: String },
    /// The asset has failed validation.
    #[error(r#"The account has failed validation."#)]
    ValidationError { info: String },
}

impl DetailableError for AssetError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            AssetError::UnknownBlockchain { blockchain } => {
                details.insert("blockchain".to_string(), blockchain.to_string());
                Some(details)
            }
            AssetError::UnknownBlockchainStandard {
                blockchain_standard,
            } => {
                details.insert(
                    "blockchain_standard".to_string(),
                    blockchain_standard.to_string(),
                );
                Some(details)
            }
            AssetError::ValidationError { info } => {
                details.insert("info".to_string(), info.to_string());
                Some(details)
            }
            AssetError::InvalidDecimals { min, max } => {
                details.insert("min".to_string(), min.to_string());
                details.insert("max".to_string(), max.to_string());
                Some(details)
            }
            AssetError::InvalidNameLength {
                min_length,
                max_length,
            } => {
                details.insert("min_length".to_string(), min_length.to_string());
                details.insert("max_length".to_string(), max_length.to_string());
                Some(details)
            }
            AssetError::InvalidSymbolLength {
                min_length,
                max_length,
            } => {
                details.insert("min_length".to_string(), min_length.to_string());
                details.insert("max_length".to_string(), max_length.to_string());
                Some(details)
            }
            AssetError::NotFound { id } => {
                details.insert("id".to_string(), id.to_string());
                Some(details)
            }
        }
    }
}
