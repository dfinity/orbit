use crate::models::{AddressBookEntry, AddressBookEntryId, Blockchain, BlockchainStandard};
use candid::{CandidType, Deserialize};
use ic_canister_macros::stable_object;

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddressBookIndex {
    /// The actual address.
    pub address: String,
    /// The blockchain type (e.g. `icp`, `eth`, `btc`)
    pub blockchain: Blockchain,
    /// The blockchain standard (e.g. `native`, `icrc1`, `erc20`, etc.)
    pub standard: BlockchainStandard,
    /// The address book entry id, which is a UUID.
    pub address_book_entry_id: AddressBookEntryId,
}

#[derive(Clone, Debug)]
pub struct AddressBookIndexCriteria {
    pub address: String,
    pub blockchain: Blockchain,
    pub standard: BlockchainStandard,
}

impl AddressBookEntry {
    pub fn to_index(&self) -> AddressBookIndex {
        AddressBookIndex {
            address: self.address.to_owned(),
            blockchain: self.blockchain.to_owned(),
            standard: self.standard.to_owned(),
            address_book_entry_id: self.id,
        }
    }
}
