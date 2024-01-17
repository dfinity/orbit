use crate::core::ic_cdk::api::time;
use crate::errors::MapperError;
use crate::models::{AddAddressBookEntryOperationInput, AddressBookEntry};
use ic_canister_core::types::UUID;
use ic_canister_core::utils::timestamp_to_rfc3339;
use uuid::Uuid;
use wallet_api::AddressBookEntryDTO;

#[derive(Default, Clone, Debug)]
pub struct AddressBookMapper {}

impl AddressBookMapper {
    pub fn to_dto(address_book_entry: AddressBookEntry) -> AddressBookEntryDTO {
        AddressBookEntryDTO {
            id: Uuid::from_slice(&address_book_entry.id)
                .unwrap()
                .hyphenated()
                .to_string(),
            address_owner: address_book_entry.address_owner,
            address: address_book_entry.address,
            standard: address_book_entry.standard.to_string(),
            blockchain: address_book_entry.blockchain.to_string(),
            metadata: address_book_entry.metadata.into_vec_dto(),
            last_modification_timestamp: timestamp_to_rfc3339(
                &address_book_entry.last_modification_timestamp,
            ),
        }
    }

    pub fn from_create_input(
        input: AddAddressBookEntryOperationInput,
        entry_id: UUID,
    ) -> Result<AddressBookEntry, MapperError> {
        if !input
            .blockchain
            .supported_standards()
            .contains(&input.standard)
        {
            return Err(MapperError::UnsupportedBlockchainStandard {
                blockchain: input.blockchain.to_string(),
                supported_standards: input
                    .blockchain
                    .supported_standards()
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            });
        }

        let new_entry = AddressBookEntry {
            id: entry_id,
            address_owner: input.address_owner,
            address: input.address,
            blockchain: input.blockchain,
            standard: input.standard,
            metadata: input.metadata.into(),
            last_modification_timestamp: time(),
        };

        Ok(new_entry)
    }
}

impl AddressBookEntry {
    pub fn to_dto(&self) -> AddressBookEntryDTO {
        AddressBookMapper::to_dto(self.clone())
    }
}
