use crate::models::{AddressBookEntry, AddressBookEntryId, Blockchain, BlockchainStandard};
use ic_canister_macros::storable;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddressBookIndex {
    /// The blockchain type (e.g. `icp`, `eth`, `btc`)
    pub blockchain: Blockchain,
    /// The blockchain standard (e.g. `native`, `icrc1`, `erc20`, etc.)
    pub standard: BlockchainStandard,
    /// The actual address.
    pub address: String,
    /// The address book entry id, which is a UUID.
    pub address_book_entry_id: AddressBookEntryId,
}

#[derive(Clone, Debug)]
pub struct AddressBookIndexCriteria {
    pub blockchain: Blockchain,
    pub standard: BlockchainStandard,
    pub address: String,
}

impl AddressBookEntry {
    pub fn to_index(&self) -> AddressBookIndex {
        AddressBookIndex {
            blockchain: self.blockchain.to_owned(),
            standard: self.standard.to_owned(),
            address: self.address.to_owned(),
            address_book_entry_id: self.id,
        }
    }
}
