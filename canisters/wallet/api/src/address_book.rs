use crate::{MetadataDTO, UuidDTO};
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
