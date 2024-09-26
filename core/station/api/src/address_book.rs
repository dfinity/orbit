use crate::{ChangeMetadataDTO, MetadataDTO, PaginationInput, UuidDTO};
use candid::{CandidType, Deserialize};

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AddressBookEntryDTO {
    pub id: UuidDTO,
    pub address_owner: String,
    pub address: String,
    pub blockchain: String,
    pub labels: Vec<String>,
    pub metadata: Vec<MetadataDTO>,
    pub last_modification_timestamp: String,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct AddressBookEntryCallerPrivilegesDTO {
    pub id: UuidDTO,
    pub can_edit: bool,
    pub can_delete: bool,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct AddAddressBookEntryOperationDTO {
    pub address_book_entry: Option<AddressBookEntryDTO>,
    pub input: AddAddressBookEntryOperationInput,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct AddAddressBookEntryOperationInput {
    pub address_owner: String,
    pub address: String,
    pub address_format: String,
    pub blockchain: String,
    pub metadata: Vec<MetadataDTO>,
    pub labels: Vec<String>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct EditAddressBookEntryOperationDTO {
    pub input: EditAddressBookEntryOperationInput,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct EditAddressBookEntryOperationInput {
    pub address_book_entry_id: UuidDTO,
    pub address_owner: Option<String>,
    pub labels: Option<Vec<String>>,
    pub change_metadata: Option<ChangeMetadataDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct RemoveAddressBookEntryOperationDTO {
    pub input: RemoveAddressBookEntryOperationInput,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct RemoveAddressBookEntryOperationInput {
    pub address_book_entry_id: UuidDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct GetAddressBookEntryInputDTO {
    pub address_book_entry_id: UuidDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct GetAddressBookEntryResponseDTO {
    pub address_book_entry: AddressBookEntryDTO,
    pub privileges: AddressBookEntryCallerPrivilegesDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ListAddressBookEntriesInputDTO {
    pub ids: Option<Vec<UuidDTO>>,
    pub addresses: Option<Vec<String>>,
    pub blockchain: Option<String>,
    pub labels: Option<Vec<String>>,
    pub paginate: Option<PaginationInput>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ListAddressBookEntriesResponseDTO {
    pub address_book_entries: Vec<AddressBookEntryDTO>,
    pub next_offset: Option<u64>,
    pub total: u64,
    pub privileges: Vec<AddressBookEntryCallerPrivilegesDTO>,
}
