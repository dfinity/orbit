use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum AddressFormatError {
    /// The address does not comply with the address format.
    #[error(r#"`{address}` does not comply with address format `{address_format}`"#)]
    InvalidAddressFormat {
        address: String,
        address_format: String,
    },
}

impl DetailableError for AddressFormatError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            AddressFormatError::InvalidAddressFormat {
                address,
                address_format,
            } => {
                details.insert("address".to_string(), address.to_string());
                details.insert("address_format".to_string(), address_format.to_string());
                Some(details)
            }
        }
    }
}
