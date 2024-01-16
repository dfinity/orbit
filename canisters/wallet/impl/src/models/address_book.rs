use super::{Blockchain, BlockchainStandard};
use crate::errors::AddressBookError;
use crate::models::Metadata;
use candid::{CandidType, Deserialize};
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use ic_canister_macros::stable_object;
use std::{collections::HashMap, hash::Hash};

/// The address book entry id, which is a UUID.
pub type AddressBookEntryId = UUID;

/// Represents an address book entry in the system.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddressBookEntry {
    /// The address book entry id, which is a UUID.
    pub id: AddressBookEntryId,
    /// The owner of the address.
    pub address_owner: String,
    /// The actual address.
    pub address: String,
    /// The blockchain type (e.g. `icp`, `eth`, `btc`)
    pub blockchain: Blockchain,
    /// The blockchain standard (e.g. `native`, `icrc1`, `erc20`, etc.)
    pub standard: BlockchainStandard,
    /// The address' metadata.
    pub metadata: Vec<Metadata>,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddressBookEntryKey {
    /// The address book entry id, which is a UUID.
    pub id: AddressBookEntryId,
}

fn validate_address_owner(address_owner: &String) -> ModelValidatorResult<AddressBookError> {
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

fn validate_address(address: &String) -> ModelValidatorResult<AddressBookError> {
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

impl ModelValidator<AddressBookError> for AddressBookEntry {
    fn validate(&self) -> ModelValidatorResult<AddressBookError> {
        validate_address_owner(&self.address_owner)?;
        validate_address(&self.address)?;
        Metadata::validate(&self.metadata)?;

        Ok(())
    }
}

impl AddressBookEntry {
    pub const ADDRESS_RANGE: (u16, u16) = (1, 255);
    pub const ADDRESS_OWNER_RANGE: (u16, u16) = (1, 255);

    /// Creates a new address_book_entry key from the given key components.
    pub fn key(id: AddressBookEntryId) -> AddressBookEntryKey {
        AddressBookEntryKey { id }
    }

    pub fn to_key(&self) -> AddressBookEntryKey {
        Self::key(self.id)
    }

    pub fn metadata_map(&self) -> HashMap<String, String> {
        self.metadata
            .iter()
            .map(|kv| (kv.key.to_owned(), kv.value.to_owned()))
            .collect()
    }
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
}

#[cfg(test)]
pub mod address_book_entry_test_utils {
    use super::*;
    use crate::repositories::ADDRESS_BOOK_REPOSITORY;
    use ic_canister_core::repository::Repository;

    pub fn mock_address_book_entry() -> AddressBookEntry {
        AddressBookEntry {
            id: [0; 16],
            address_owner: "foo".to_string(),
            address: "0x1234".to_string(),
            blockchain: Blockchain::InternetComputer,
            standard: BlockchainStandard::Native,
            metadata: vec![
                Metadata {
                    key: "a".repeat(Metadata::MAX_METADATA_KEY_LEN.into()),
                    value: "b".repeat(Metadata::MAX_METADATA_VALUE_LEN.into())
                };
                Metadata::MAX_METADATA as usize
            ],
            last_modification_timestamp: 0,
        }
    }

    pub fn add_address_book_entry(id: &UUID) -> AddressBookEntry {
        let mut address_book_entry = mock_address_book_entry();
        address_book_entry.id = id.to_owned();
        ADDRESS_BOOK_REPOSITORY.insert(address_book_entry.to_key(), address_book_entry.to_owned());

        address_book_entry
    }
}
