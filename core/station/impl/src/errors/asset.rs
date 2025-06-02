use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

use super::ValidationError;

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
    /// The given blockchain is unknown to the system.
    #[error(r#"The given blockchain is unknown to the system."#)]
    UnknownBlockchain { blockchain: String },
    /// The given token standard is unknown to the system.
    #[error(r#"The given token standard is unknown to the system."#)]
    UnknownTokenStandard { token_standard: String },
    /// The asset has failed validation.
    #[error(r#"The account has failed validation."#)]
    ValidationError { info: String },
    /// The asset is in use.
    #[error(r#"The asset is used by {resource} `{id}`"#)]
    AssetInUse { id: String, resource: String },
    /// The asset is not unique.
    #[error(r#"The asset already exists."#)]
    AlreadyExists {
        /// The asset symbol.
        symbol: String,
        /// The asset blockchain.
        blockchain: String,
    },
    /// The asset with id `{id}` already exists.
    #[error(r#"The asset with id `{id}` already exists."#)]
    IdAlreadyExists { id: String },
}

impl DetailableError for AssetError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            AssetError::UnknownBlockchain { blockchain } => {
                details.insert("blockchain".to_string(), blockchain.to_string());
                Some(details)
            }
            AssetError::UnknownTokenStandard { token_standard } => {
                details.insert("token_standard".to_string(), token_standard.to_string());
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
            AssetError::NotFound { id } => {
                details.insert("id".to_string(), id.to_string());
                Some(details)
            }
            AssetError::AlreadyExists { symbol, blockchain } => {
                details.insert("symbol".to_string(), symbol.to_string());
                details.insert("blockchain".to_string(), blockchain.to_string());
                Some(details)
            }
            AssetError::AssetInUse { id, resource } => {
                details.insert("id".to_string(), id.to_string());
                details.insert("resource".to_string(), resource.to_string());
                Some(details)
            }
            AssetError::IdAlreadyExists { id } => {
                details.insert("id".to_string(), id.to_string());
                Some(details)
            }
        }
    }
}

impl From<ValidationError> for AssetError {
    fn from(err: ValidationError) -> Self {
        AssetError::ValidationError {
            info: err.to_string(),
        }
    }
}
