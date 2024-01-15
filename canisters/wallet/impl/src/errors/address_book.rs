use ic_canister_core::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for address book errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum AddressBookError {
    /// The given address owner length is out of range.
    #[error(
        r#"The adress owner length is out of range, it must be between {min_length} and {max_length}."#
    )]
    InvalidAddressOwnerLength { min_length: u16, max_length: u16 },
    /// The given address length is out of range.
    #[error(
        r#"The adress length is out of range, it must be between {min_length} and {max_length}."#
    )]
    InvalidAddressLength { min_length: u16, max_length: u16 },
    /// The given blockchain is unknown to the system.
    #[error(r#"The given blockchain is unknown to the system."#)]
    UnknownBlockchain { blockchain: String },
    /// The given blockchain standard is unknown to the system.
    #[error(r#"The given blockchain standard is unknown to the system."#)]
    UnknownBlockchainStandard { blockchain_standard: String },
    /// The account has failed validation.
    #[error(r#"The account has failed validation."#)]
    ValidationError { info: String },
}

impl DetailableError for AddressBookError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            AddressBookError::InvalidAddressOwnerLength {
                min_length,
                max_length,
            } => {
                details.insert("min_length".to_string(), min_length.to_string());
                details.insert("max_length".to_string(), max_length.to_string());
                Some(details)
            }
            AddressBookError::InvalidAddressLength {
                min_length,
                max_length,
            } => {
                details.insert("min_length".to_string(), min_length.to_string());
                details.insert("max_length".to_string(), max_length.to_string());
                Some(details)
            }
            AddressBookError::UnknownBlockchain { blockchain } => {
                details.insert("blockchain".to_string(), blockchain.to_string());
                Some(details)
            }
            AddressBookError::UnknownBlockchainStandard {
                blockchain_standard,
            } => {
                details.insert(
                    "blockchain_standard".to_string(),
                    blockchain_standard.to_string(),
                );
                Some(details)
            }
            AddressBookError::ValidationError { info } => {
                details.insert("info".to_string(), info.to_string());
                Some(details)
            }
        }
    }
}
