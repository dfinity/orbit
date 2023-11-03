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
    /// Cannot set the symbol for native assets.
    #[error(r#"Cannot set the symbol for native assets."#)]
    NativeAccountSymbolMetadataNotAllowed,
    /// Accounts for non native assets are required to have a defined token symbol.
    #[error(r#"Accounts for non native assets are required to have a defined token symbol."#)]
    NonNativeAccountSymbolRequired,
    /// The provided format is not compatible with a UUID.
    #[error(r#"The provided format is not compatible with a UUID."#)]
    MalformedUuid {
        /// The malformed UUID.
        malformed_uuid: String,
    },
    /// The provided biguint cannot be converted to u64.
    #[error(r#"The provided biguint cannot be converted to u64."#)]
    BigUintConversionError {
        /// The biguint that failed to be converted.
        biguint: String,
    },
    /// The provided string cannot be converted to u64.
    #[error(r#"The provided string cannot be converted to u64."#)]
    StringToNumberConversionError {
        /// The string provided.
        input: String,
    },
    /// The requested operation code is unknown to the system.
    #[error(r#"The requested operation code is unknown to the system."#)]
    UnknownOperationCode {
        /// The string provided.
        code: String,
    },
}

impl DetailableError for MapperError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            MapperError::UnknownBlockchain { blockchain } => {
                details.insert("blockchain".to_string(), blockchain.to_string());
                Some(details)
            }
            MapperError::UnknownBlockchainStandard {
                blockchain_standard,
            } => {
                details.insert(
                    "blockchain_standard".to_string(),
                    blockchain_standard.to_string(),
                );
                Some(details)
            }
            MapperError::UnsupportedBlockchainStandard {
                blockchain,
                supported_standards,
            } => {
                details.insert("blockchain".to_string(), blockchain.to_string());
                details.insert(
                    "supported_standards".to_string(),
                    supported_standards.join(",").to_string(),
                );
                Some(details)
            }
            MapperError::MalformedUuid { malformed_uuid } => {
                details.insert("malformed_uuid".to_string(), malformed_uuid.to_string());
                Some(details)
            }
            MapperError::BigUintConversionError { biguint } => {
                details.insert("biguint".to_string(), biguint.to_string());
                Some(details)
            }
            MapperError::StringToNumberConversionError { input } => {
                details.insert("input".to_string(), input.to_string());
                Some(details)
            }
            MapperError::UnknownOperationCode { code } => {
                details.insert("code".to_string(), code.to_string());
                Some(details)
            }
            _ => None,
        }
    }
}
