use crate::{MetadataDTO, PaginationInput, UuidDTO};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AddressBookEntryDTO {
    pub id: UuidDTO,
    pub address_owner: String,
    pub address: String,
    pub blockchain: String,
    pub standard: String,
    pub metadata: Vec<MetadataDTO>,
    pub last_modification_timestamp: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetAddressBookEntryInputDTO {
    pub address_book_entry_id: UuidDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetAddressBookEntryResponseDTO {
    pub address_book_entry: AddressBookEntryDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListAddressBookEntriesInputDTO {
    pub blockchain: String,
    pub standard: String,
    pub paginate: PaginationInput,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListAddressBookEntriesResponseDTO {
    pub address_book_entries: Vec<AddressBookEntryDTO>,
    pub next_offset: Option<u64>,
    pub total: u64,
}
