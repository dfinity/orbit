use crate::models::AddressBookEntry;
use ic_canister_core::utils::timestamp_to_rfc3339;
use uuid::Uuid;
use wallet_api::AddressBookEntryDTO;

#[derive(Default, Clone, Debug)]
pub struct AddressBookEntryMapper {}

impl AddressBookEntryMapper {
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
            metadata: address_book_entry.metadata,
            last_modification_timestamp: timestamp_to_rfc3339(
                &address_book_entry.last_modification_timestamp,
            ),
        }
    }
}

impl AddressBookEntry {
    pub fn to_dto(&self) -> AddressBookEntryDTO {
        AddressBookEntryMapper::to_dto(self.clone())
    }
}
