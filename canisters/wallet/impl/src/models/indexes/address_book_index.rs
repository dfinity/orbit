use crate::models::{AddressBookEntry, AddressBookEntryId, Blockchain, BlockchainStandard};
use candid::{CandidType, Deserialize};
use ic_canister_macros::stable_object;

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// The fields `blockchain` and `standard` must come first so that
/// they are the primary key of the index.
/// The field `address_book_entry_id` should come next since it is easy to create
/// a minimum and maximum value of that field which is necessary to find a range
/// of all address book entries with a given blockchain and standard.
/// The remaining field `address` storing the actual address comes last.
pub struct AddressBookIndex {
    /// The blockchain type (e.g. `icp`, `eth`, `btc`)
    pub blockchain: Blockchain,
    /// The blockchain standard (e.g. `native`, `icrc1`, `erc20`, etc.)
    pub standard: BlockchainStandard,
    /// The address book entry id, which is a UUID.
    pub address_book_entry_id: AddressBookEntryId,
    /// The actual address.
    pub address: String,
}

#[derive(Clone, Debug)]
pub struct AddressBookIndexCriteria {
    pub blockchain: Blockchain,
    pub standard: BlockchainStandard,
    pub address: Option<String>,
}

impl AddressBookEntry {
    pub fn to_index(&self) -> AddressBookIndex {
        AddressBookIndex {
            blockchain: self.blockchain.to_owned(),
            standard: self.standard.to_owned(),
            address_book_entry_id: self.id,
            address: self.address.to_owned(),
        }
    }
}
