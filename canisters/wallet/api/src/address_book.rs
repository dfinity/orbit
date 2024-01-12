use crate::UuidDTO;
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddressBookMetadataDTO {
    pub key: String,
    pub value: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AddressBookEntryDTO {
    pub id: UuidDTO,
    pub address_owner: String,
    pub address: String,
    pub blockchain: String,
    pub standard: String,
    pub metadata: Vec<AddressBookMetadataDTO>,
    pub last_modification_timestamp: String,
}
