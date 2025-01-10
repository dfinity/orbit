use super::{AddressFormat, Blockchain};
use crate::errors::AddressBookError;
use crate::models::Metadata;
use candid::{CandidType, Deserialize};
use orbit_essentials::model::ModelKey;
use orbit_essentials::storable;
use orbit_essentials::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use std::{collections::HashMap, hash::Hash};

/// The address book entry id, which is a UUID.
pub type AddressBookEntryId = UUID;

/// Represents an address book entry in the system.
#[storable(skip_deserialize = true)]
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

fn validate_address_owner(address_owner: &str) -> ModelValidatorResult<AddressBookError> {
    if (address_owner.len() < AddressBookEntry::ADDRESS_OWNER_RANGE.0 as usize)
        || (address_owner.len() > AddressBookEntry::ADDRESS_OWNER_RANGE.1 as usize)
    {
        return Err(AddressBookError::InvalidAddressOwnerLength {
            min_length: AddressBookEntry::ADDRESS_OWNER_RANGE.0,
            max_length: AddressBookEntry::ADDRESS_OWNER_RANGE.1,
        });
    }

    Ok(())
}

fn validate_address(address: &str) -> ModelValidatorResult<AddressBookError> {
    if (address.len() < AddressBookEntry::ADDRESS_RANGE.0 as usize)
        || (address.len() > AddressBookEntry::ADDRESS_RANGE.1 as usize)
    {
        return Err(AddressBookError::InvalidAddressLength {
            min_length: AddressBookEntry::ADDRESS_RANGE.0,
            max_length: AddressBookEntry::ADDRESS_RANGE.1,
        });
    }

    Ok(())
}

fn validate_labels(labels: &[String]) -> ModelValidatorResult<AddressBookError> {
    for label in labels {
        if label.is_empty() {
            return Err(AddressBookError::ValidationError {
                info: "Label entry cannot be empty".to_string(),
            });
        }

        if label.len() > AddressBookEntry::MAX_LABEL_LENGTH {
            return Err(AddressBookError::ValidationError {
                info: format!(
                    "Label entry cannot be longer than {} characters",
                    AddressBookEntry::MAX_LABEL_LENGTH
                ),
            });
        }
    }

    if labels.len() > AddressBookEntry::MAX_LABELS {
        return Err(AddressBookError::ValidationError {
            info: format!(
                "Address book entry cannot have more than {} labels",
                AddressBookEntry::MAX_LABELS
            ),
        });
    }

    Ok(())
}

impl ModelValidator<AddressBookError> for AddressBookEntry {
    fn validate(&self) -> ModelValidatorResult<AddressBookError> {
        validate_address_owner(&self.address_owner)?;
        validate_address(&self.address)?;
        validate_labels(&self.labels)?;

        self.metadata.validate()?;

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
            AddressBookError::InvalidAddressOwnerLength {
                min_length: AddressBookEntry::ADDRESS_OWNER_RANGE.0,
                max_length: AddressBookEntry::ADDRESS_OWNER_RANGE.1,
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
            AddressBookError::InvalidAddressOwnerLength {
                min_length: AddressBookEntry::ADDRESS_OWNER_RANGE.0,
                max_length: AddressBookEntry::ADDRESS_OWNER_RANGE.1,
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
            AddressBookError::InvalidAddressLength {
                min_length: AddressBookEntry::ADDRESS_RANGE.0,
                max_length: AddressBookEntry::ADDRESS_RANGE.1,
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
            AddressBookError::InvalidAddressLength {
                min_length: AddressBookEntry::ADDRESS_RANGE.0,
                max_length: AddressBookEntry::ADDRESS_RANGE.1,
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
                info: format!(
                    "Label entry cannot be longer than {} characters",
                    AddressBookEntry::MAX_LABEL_LENGTH
                ),
            }
        );
    }
}

#[cfg(test)]
pub mod address_book_entry_test_utils {
    use super::*;
    use crate::repositories::ADDRESS_BOOK_REPOSITORY;
    use orbit_essentials::repository::Repository;
    use uuid::Uuid;

    pub fn mock_address_book_entry() -> AddressBookEntry {
        AddressBookEntry {
            id: *Uuid::new_v4().as_bytes(),
            address_owner: "foo".to_string(),
            address: "0x1234".to_string(),
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
