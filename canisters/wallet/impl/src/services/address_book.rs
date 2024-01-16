use crate::{
    core::generate_uuid_v4,
    errors::AddressBookError,
    mappers::AddressBookMapper,
    models::{
        AddAddressBookEntryOperationInput, AddressBookEntry, AddressBookEntryId, Blockchain,
        BlockchainStandard, EditAddressBookEntryOperationInput,
    },
    repositories::{AddressBookRepository, ADDRESS_BOOK_REPOSITORY},
};
use ic_canister_core::{api::ServiceResult, model::ModelValidator, repository::Repository};
use lazy_static::lazy_static;
use std::sync::Arc;
use uuid::Uuid;

lazy_static! {
    pub static ref ADDRESS_BOOK_SERVICE: Arc<AddressBookService> = Arc::new(
        AddressBookService::new(Arc::clone(&ADDRESS_BOOK_REPOSITORY),)
    );
}

#[derive(Default, Debug)]
pub struct AddressBookService {
    address_book_repository: Arc<AddressBookRepository>,
}

impl AddressBookService {
    pub fn new(address_book_repository: Arc<AddressBookRepository>) -> Self {
        Self {
            address_book_repository,
        }
    }

    /// Returns the address book entries associated with the given address.
    pub fn get_entry_by_id(&self, id: &AddressBookEntryId) -> ServiceResult<AddressBookEntry> {
        let address_book_entry_key = AddressBookEntry::key(*id);
        let address_book_entry = self
            .address_book_repository
            .get(&address_book_entry_key)
            .ok_or(AddressBookError::AddressBookEntryNotFound {
                id: Uuid::from_bytes(*id).hyphenated().to_string(),
            })?;

        Ok(address_book_entry)
    }

    /// Returns the address book entry associated with the given address.
    pub fn get_entry(
        &self,
        address: String,
        blockchain: Blockchain,
        standard: BlockchainStandard,
    ) -> ServiceResult<AddressBookEntry> {
        let entry = self
            .address_book_repository
            .find(address.clone(), blockchain, standard)
            .ok_or(AddressBookError::AddressNotFound { address })?;

        Ok(entry)
    }

    /// Creates a new address book entry.
    pub async fn create_entry(
        &self,
        input: AddAddressBookEntryOperationInput,
    ) -> ServiceResult<AddressBookEntry> {
        let uuid = generate_uuid_v4().await;
        let key = AddressBookEntry::key(*uuid.as_bytes());

        let new_entry = AddressBookMapper::from_create_input(input.to_owned(), *uuid.as_bytes())?;
        new_entry.validate()?;

        if let Some(v) = self.address_book_repository.find(
            new_entry.address.clone(),
            new_entry.blockchain.clone(),
            new_entry.standard.clone(),
        ) {
            return Err(AddressBookError::DuplicateAddress {
                id: Uuid::from_bytes(v.id).hyphenated().to_string(),
            })?;
        }

        // Inserting the address book entry into the repository and its associations is the last step of the address book entry creation
        // process to avoid potential consistency issues due to the fact that some of the calls to create the address book entry
        // happen in an asynchronous way.
        self.address_book_repository.insert(key, new_entry.clone());

        Ok(new_entry)
    }

    /// Edits an existing address book entry.
    pub async fn edit_entry(
        &self,
        input: EditAddressBookEntryOperationInput,
    ) -> ServiceResult<AddressBookEntry> {
        let mut entry = self.get_entry_by_id(&input.address_book_entry_id)?;

        if let Some(address_owner) = &input.address_owner {
            entry.address_owner = address_owner.to_owned();
        }

        if let Some(metadata) = &input.metadata {
            entry.metadata = metadata.to_owned();
        }

        entry.validate()?;

        self.address_book_repository
            .insert(entry.to_key(), entry.to_owned());

        Ok(entry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::test_utils,
        models::{
            address_book_entry_test_utils::mock_address_book_entry, AddAddressBookEntryOperation,
            AddAddressBookEntryOperationInput, Blockchain, BlockchainStandard,
        },
    };

    struct TestContext {
        repository: AddressBookRepository,
        service: AddressBookService,
    }

    fn setup() -> TestContext {
        test_utils::init_canister_config();

        TestContext {
            repository: AddressBookRepository::default(),
            service: AddressBookService::default(),
        }
    }

    #[test]
    fn get_entry() {
        let ctx = setup();
        let address_book_entry = mock_address_book_entry();

        ctx.repository
            .insert(address_book_entry.to_key(), address_book_entry.clone());

        let result = ctx.service.get_entry_by_id(&address_book_entry.id);
        assert_eq!(result, Ok(address_book_entry.clone()));

        let result = ctx.service.get_entry(
            address_book_entry.address.clone(),
            address_book_entry.blockchain.clone(),
            address_book_entry.standard.clone(),
        );
        assert_eq!(result, Ok(address_book_entry));
    }

    #[tokio::test]
    async fn create_entry() {
        let ctx = setup();
        let mut address_book_entry = mock_address_book_entry();

        let operation = AddAddressBookEntryOperation {
            address_book_entry_id: None,
            input: AddAddressBookEntryOperationInput {
                address_owner: "foo".to_string(),
                address: "0x1234".to_string(),
                blockchain: Blockchain::InternetComputer,
                standard: BlockchainStandard::Native,
                metadata: address_book_entry.metadata.clone(),
            },
        };

        let result = ctx.service.create_entry(operation.input.clone()).await;

        let new_entry = result.unwrap();

        address_book_entry.id = new_entry.id;

        assert_eq!(new_entry, address_book_entry);

        // adding a new entry for the same address should fail

        let result = ctx.service.create_entry(operation.input).await;

        result.unwrap_err();
    }

    #[tokio::test]
    async fn edit_entry() {
        let ctx = setup();
        let mut address_book_entry = mock_address_book_entry();

        ctx.repository
            .insert(address_book_entry.to_key(), address_book_entry.clone());

        let operation = EditAddressBookEntryOperationInput {
            address_book_entry_id: address_book_entry.id,
            address_owner: Some("test_edit".to_string()),
            metadata: Some(vec![]),
        };

        let result = ctx.service.edit_entry(operation).await;

        assert!(result.is_ok());

        let updated_entry = result.unwrap();

        address_book_entry.address_owner = "test_edit".to_string();
        address_book_entry.metadata = vec![];

        assert_eq!(updated_entry, address_book_entry);
    }

    #[tokio::test]
    async fn fail_create_entry_invalid_blockchain_standard() {
        let ctx = setup();

        let operation = AddAddressBookEntryOperation {
            address_book_entry_id: None,
            input: AddAddressBookEntryOperationInput {
                address_owner: "foo".to_string(),
                address: "0x1234".to_string(),
                blockchain: Blockchain::InternetComputer,
                standard: BlockchainStandard::ERC20,
                metadata: vec![],
            },
        };

        let result = ctx.service.create_entry(operation.input).await;

        assert!(result.is_err());
    }
}
