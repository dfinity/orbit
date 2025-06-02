use super::{AddressFormat, Blockchain};
use crate::errors::AddressBookError;
use crate::models::Metadata;
use candid::{CandidType, Deserialize};
use lazy_static::lazy_static;
use orbit_essentials::model::ModelKey;
use orbit_essentials::storable;
use orbit_essentials::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use std::{collections::HashMap, hash::Hash, sync::Arc};

use crate::core::validation::{
    StringFieldValidator, StringFieldValidatorBuilder, ValidateField, VecFieldValidator,
    VecFieldValidatorBuilder,
};

lazy_static! {
    pub static ref ADDRESS_BOOK_ADDRESS_OWNER_VALIDATOR: StringFieldValidator = {
        StringFieldValidatorBuilder::new("address_owner".to_string())
            .min_length(AddressBookEntry::ADDRESS_OWNER_RANGE.0 as usize)
            .max_length(AddressBookEntry::ADDRESS_OWNER_RANGE.1 as usize)
            .build()
    };
    pub static ref ADDRESS_BOOK_ADDRESS_VALIDATOR: StringFieldValidator = {
        StringFieldValidatorBuilder::new("address".to_string())
            .min_length(AddressBookEntry::ADDRESS_RANGE.0 as usize)
            .max_length(AddressBookEntry::ADDRESS_RANGE.1 as usize)
            .build()
    };
    pub static ref ADDRESS_BOOK_LABEL_VALIDATOR: StringFieldValidator = {
        StringFieldValidatorBuilder::new("label".to_string())
            .min_length(1)
            .max_length(AddressBookEntry::MAX_LABEL_LENGTH as usize)
            .build()
    };
    pub static ref ADDRESS_BOOK_LABELS_VALIDATOR: VecFieldValidator<String> = {
        VecFieldValidatorBuilder::new(
            "labels".to_string(),
            Arc::new(ADDRESS_BOOK_LABEL_VALIDATOR.clone()),
        )
        .max_length(AddressBookEntry::MAX_LABELS)
        .build()
    };
}

/// The address book entry id, which is a UUID.
pub type AddressBookEntryId = UUID;

/// Represents an address book entry in the system.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddressBookEntry {
    /// The address book entry id, which is a UUID.
    pub id: AddressBookEntryId,
    /// The owner of the address.
    pub address_owner: String,
    /// The actual address.
    pub address: String,
    /// The blockchain type (e.g. `icp`, `eth`, `btc`)
    pub blockchain: Blockchain,
    /// The address' format.
    pub address_format: AddressFormat,
    /// The address' metadata.
    pub metadata: Metadata,
    /// The labels associated with the address.
    #[serde(default)]
    pub labels: Vec<String>,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddressBookEntryKey {
    /// The address book entry id, which is a UUID.
    pub id: AddressBookEntryId,
}

impl ModelKey<AddressBookEntryKey> for AddressBookEntry {
    fn key(&self) -> AddressBookEntryKey {
        AddressBookEntryKey { id: self.id }
    }
}

impl ModelValidator<AddressBookError> for AddressBookEntry {
    fn validate(&self) -> ModelValidatorResult<AddressBookError> {
        ADDRESS_BOOK_ADDRESS_OWNER_VALIDATOR.validate_field(&self.address_owner)?;
        ADDRESS_BOOK_ADDRESS_VALIDATOR.validate_field(&self.address)?;
        ADDRESS_BOOK_LABELS_VALIDATOR.validate_field(&self.labels)?;

        self.metadata.validate()?;

        self.address_format.validate_address(&self.address)?;

        Ok(())
    }
}

impl AddressBookEntry {
    pub const ADDRESS_RANGE: (u16, u16) = (1, 255);
    pub const ADDRESS_OWNER_RANGE: (u16, u16) = (1, 255);
    pub const MAX_LABELS: usize = 10;
    pub const MAX_LABEL_LENGTH: usize = 150;

    /// Creates a new address_book_entry key from the given key components.
    pub fn key(id: AddressBookEntryId) -> AddressBookEntryKey {
        AddressBookEntryKey { id }
    }

    pub fn to_key(&self) -> AddressBookEntryKey {
        Self::key(self.id)
    }

    pub fn metadata_map(&self) -> HashMap<String, String> {
        self.metadata.map()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListAddressBookEntriesInput {
    pub ids: Option<Vec<UUID>>,
    pub addresses: Option<Vec<String>>,
    pub blockchain: Option<Blockchain>,
    pub labels: Option<Vec<String>>,
    pub address_formats: Option<Vec<AddressFormat>>,
    pub search_term: Option<String>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AddressBookEntryCallerPrivileges {
    pub id: UUID,
    pub can_edit: bool,
    pub can_delete: bool,
}

#[cfg(test)]
mod tests {
    use super::address_book_entry_test_utils::mock_address_book_entry;
    use super::*;

    #[test]
    fn test_address_book_entry_validation() {
        let address_book_entry = mock_address_book_entry();

        let result = address_book_entry.validate();

        assert!(result.is_ok());
    }

    #[test]
    fn fail_address_owner_too_short() {
        let mut address_book_entry = mock_address_book_entry();
        address_book_entry.address_owner = "".to_string();

        let result = address_book_entry.validate();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AddressBookError::ValidationError {
                info: "The field `address_owner` is invalid: Length cannot be shorter than 1."
                    .to_string(),
            }
        );
    }

    #[test]
    fn fail_address_owner_too_long() {
        let mut address_book_entry = mock_address_book_entry();
        address_book_entry.address_owner =
            "a".repeat(AddressBookEntry::ADDRESS_OWNER_RANGE.1 as usize + 1);

        let result = address_book_entry.validate();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AddressBookError::ValidationError {
                info: "The field `address_owner` is invalid: Length cannot be longer than 255."
                    .to_string(),
            }
        );
    }

    #[test]
    fn fail_address_too_short() {
        let mut address_book_entry = mock_address_book_entry();
        address_book_entry.address = "".to_string();

        let result = address_book_entry.validate();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AddressBookError::ValidationError {
                info: "The field `address` is invalid: Length cannot be shorter than 1."
                    .to_string(),
            }
        );
    }

    #[test]
    fn fail_address_too_long() {
        let mut address_book_entry = mock_address_book_entry();
        address_book_entry.address = "a".repeat(AddressBookEntry::ADDRESS_RANGE.1 as usize + 1);

        let result = address_book_entry.validate();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AddressBookError::ValidationError {
                info: "The field `address` is invalid: Length cannot be longer than 255."
                    .to_string(),
            }
        );
    }

    #[test]
    fn fail_label_too_long() {
        let mut address_book_entry = mock_address_book_entry();
        address_book_entry.labels = vec!["a".repeat(AddressBookEntry::MAX_LABEL_LENGTH + 1)];

        let result = address_book_entry.validate();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AddressBookError::ValidationError {
                info: "The field `label` is invalid: Length cannot be longer than 150.".to_string(),
            }
        );
    }
}

#[cfg(test)]
pub mod address_book_entry_test_utils {
    use super::*;
    use crate::{
        models::address_format_test_utils::VALID_ACCOUNT_IDENTIFIER,
        repositories::ADDRESS_BOOK_REPOSITORY,
    };
    use orbit_essentials::repository::Repository;
    use uuid::Uuid;

    pub fn mock_address_book_entry() -> AddressBookEntry {
        AddressBookEntry {
            id: *Uuid::new_v4().as_bytes(),
            address_owner: "foo".to_string(),
            address: VALID_ACCOUNT_IDENTIFIER.to_string(),
            address_format: AddressFormat::ICPAccountIdentifier,
            labels: Vec::new(),
            blockchain: Blockchain::InternetComputer,
            metadata: Metadata::mock(),
            last_modification_timestamp: 0,
        }
    }

    pub fn add_address_book_entry(id: &UUID) -> AddressBookEntry {
        let mut address_book_entry = mock_address_book_entry();
        id.clone_into(&mut address_book_entry.id);
        ADDRESS_BOOK_REPOSITORY.insert(address_book_entry.to_key(), address_book_entry.to_owned());

        address_book_entry
    }
}
