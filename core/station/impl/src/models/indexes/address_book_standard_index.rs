use crate::models::{AddressBookEntry, AddressBookEntryId, Blockchain, BlockchainStandard};
use orbit_essentials::storable;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddressBookStandardIndex {
    /// The blockchain type (e.g. `icp`, `eth`, `btc`)
    pub blockchain: Blockchain,
    /// The blockchain standard (e.g. `native`, `icrc1`, `erc20`, etc.)
    pub standard: BlockchainStandard,
    /// The address book entry id, which is a UUID.
    pub address_book_entry_id: AddressBookEntryId,
}

#[derive(Clone, Debug)]
pub struct AddressBookStandardIndexCriteria {
    pub blockchain: Blockchain,
    pub standard: BlockchainStandard,
}

impl AddressBookEntry {
    pub fn to_standard_index(&self) -> AddressBookStandardIndex {
        AddressBookStandardIndex {
            blockchain: self.blockchain.to_owned(),
            standard: self.standard.to_owned(),
            address_book_entry_id: self.id,
        }
    }
}
