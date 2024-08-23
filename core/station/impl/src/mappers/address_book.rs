use super::HelperMapper;
use crate::core::ic_cdk::next_time;
use crate::errors::MapperError;
use crate::mappers::blockchain::BlockchainMapper;
use crate::models::{
    AddAddressBookEntryOperationInput, AddressBookEntry, AddressBookEntryCallerPrivileges,
    ListAddressBookEntriesInput,
};
use orbit_essentials::types::UUID;
use orbit_essentials::utils::timestamp_to_rfc3339;
use station_api::{
    AddressBookEntryCallerPrivilegesDTO, AddressBookEntryDTO, ListAddressBookEntriesInputDTO,
};
use uuid::Uuid;

#[derive(Default, Clone, Debug)]
pub struct AddressBookMapper {}

impl AddressBookMapper {
    pub fn to_dto(address_book_entry: AddressBookEntry) -> AddressBookEntryDTO {
        AddressBookEntryDTO {
            id: Uuid::from_bytes(address_book_entry.id)
                .hyphenated()
                .to_string(),
            address_owner: address_book_entry.address_owner,
            address: address_book_entry.address,
            blockchain: address_book_entry.blockchain.to_string(),
            metadata: address_book_entry.metadata.into_vec_dto(),
            labels: address_book_entry.labels,
            last_modification_timestamp: timestamp_to_rfc3339(
                &address_book_entry.last_modification_timestamp,
            ),
        }
    }

    pub fn from_create_input(
        input: AddAddressBookEntryOperationInput,
        entry_id: UUID,
    ) -> Result<AddressBookEntry, MapperError> {
        let new_entry = AddressBookEntry {
            id: entry_id,
            address_owner: input.address_owner,
            address: input.address,
            blockchain: input.blockchain,
            labels: input.labels,
            metadata: input.metadata.into(),
            last_modification_timestamp: next_time(),
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
            blockchain: input.blockchain.map(|blockchain| {
                BlockchainMapper::to_blockchain(blockchain).expect("Invalid blockchain")
            }),
            labels: input.labels,
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

impl From<AddressBookEntryCallerPrivileges> for AddressBookEntryCallerPrivilegesDTO {
    fn from(input: AddressBookEntryCallerPrivileges) -> AddressBookEntryCallerPrivilegesDTO {
        AddressBookEntryCallerPrivilegesDTO {
            id: Uuid::from_bytes(input.id).hyphenated().to_string(),
            can_edit: input.can_edit,
            can_delete: input.can_delete,
        }
    }
}
