use crate::core::ic_cdk::api::time;
use crate::errors::MapperError;
use crate::mappers::BlockchainMapper;
use crate::models::{
    AddAddressBookEntryOperationInput, AddressBookEntry, AddressChain, ListAddressBookEntriesInput,
    ListAddressBookEntriesResponse,
};
use ic_canister_core::types::UUID;
use ic_canister_core::utils::timestamp_to_rfc3339;
use uuid::Uuid;
use wallet_api::{
    AddressBookEntryDTO, ListAddressBookEntriesInputDTO, ListAddressBookEntriesResponseDTO,
};

use super::HelperMapper;

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

impl From<ListAddressBookEntriesInputDTO> for ListAddressBookEntriesInput {
    fn from(input: ListAddressBookEntriesInputDTO) -> ListAddressBookEntriesInput {
        ListAddressBookEntriesInput {
            address_chain: input.address_chain.map(|address_chain| AddressChain {
                blockchain: BlockchainMapper::to_blockchain(address_chain.blockchain)
                    .expect("Invalid blockchain"),
                standard: BlockchainMapper::to_blockchain_standard(address_chain.standard)
                    .expect("Invalid blockchain standard"),
            }),
            addresses: input.addresses,
            ids: input.ids.map(|ids| {
                ids.into_iter()
                    .map(|id| {
                        HelperMapper::to_uuid(id)
                            .expect("Invalid UUID")
                            .into_bytes()
                    })
                    .collect()
            }),
        }
    }
}

impl From<ListAddressBookEntriesResponse> for ListAddressBookEntriesResponseDTO {
    fn from(input: ListAddressBookEntriesResponse) -> ListAddressBookEntriesResponseDTO {
        ListAddressBookEntriesResponseDTO {
            address_book_entries: input
                .address_book_entries
                .into_iter()
                .map(|address_book_entry| address_book_entry.to_dto())
                .collect(),
            next_offset: input.next_offset,
            total: input.total,
        }
    }
}
