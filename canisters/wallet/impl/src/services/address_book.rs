use crate::{
    core::{
        access_control::evaluate_caller_access,
        generate_uuid_v4,
        utils::{paginated_items, PaginatedData, PaginatedItemsArgs},
        CallContext,
    },
    errors::AddressBookError,
    mappers::AddressBookMapper,
    models::{
        access_control::{CommonActionSpecifier, ResourceSpecifier, ResourceType},
        specifier::CommonSpecifier,
        AddAddressBookEntryOperationInput, AddressBookEntry, AddressBookEntryCallerPrivileges,
        AddressBookEntryId, EditAddressBookEntryOperationInput, ListAddressBookEntriesInput,
        RemoveAddressBookEntryOperationInput,
    },
    repositories::{AddressBookRepository, AddressBookWhereClause, ADDRESS_BOOK_REPOSITORY},
};
use ic_canister_core::{api::ServiceResult, model::ModelValidator, repository::Repository};
use lazy_static::lazy_static;
use std::sync::Arc;
use uuid::Uuid;
use wallet_api::PaginationInput;

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
    pub const DEFAULT_ENTRIES_LIMIT: u16 = 100;
    pub const MAX_LIST_ENTRIES_LIMIT: u16 = 1000;

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

    /// Returns the caller privileges for the given address book entry.
    pub async fn get_entry_caller_privileges(
        &self,
        id: &AddressBookEntryId,
        ctx: &CallContext,
    ) -> ServiceResult<AddressBookEntryCallerPrivileges> {
        let can_edit = evaluate_caller_access(
            ctx,
            &ResourceSpecifier::Common(
                ResourceType::AddressBook,
                CommonActionSpecifier::Update(CommonSpecifier::Id(vec![*id])),
            ),
        )
        .await;

        let can_delete = evaluate_caller_access(
            ctx,
            &ResourceSpecifier::Common(
                ResourceType::AddressBook,
                CommonActionSpecifier::Delete(CommonSpecifier::Id(vec![*id])),
            ),
        )
        .await;

        Ok(AddressBookEntryCallerPrivileges {
            id: *id,
            can_edit: can_edit.is_ok(),
            can_delete: can_delete.is_ok(),
        })
    }

    /// Returns all address book entries for the given blockchain standard.
    pub fn search_entries(
        &self,
        input: ListAddressBookEntriesInput,
        paginate: Option<PaginationInput>,
    ) -> ServiceResult<PaginatedData<AddressBookEntry>> {
        let entries = self
            .address_book_repository
            .find_where(AddressBookWhereClause {
                ids: input.ids,
                addresses: input.addresses,
                address_chain: input.address_chain,
            });

        Ok(paginated_items(PaginatedItemsArgs {
            offset: paginate.to_owned().and_then(|p| p.offset),
            limit: paginate.and_then(|p| p.limit),
            default_limit: Some(Self::DEFAULT_ENTRIES_LIMIT),
            max_limit: Some(Self::MAX_LIST_ENTRIES_LIMIT),
            items: &entries,
        })?)
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

        if let Some(v) = self.address_book_repository.find_by_address(
            new_entry.blockchain.clone(),
            new_entry.standard.clone(),
            new_entry.address.clone(),
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

        if let Some(change_metadata) = input.change_metadata {
            entry.metadata.change(change_metadata);
        }

        entry.validate()?;

        self.address_book_repository
            .insert(entry.to_key(), entry.to_owned());

        Ok(entry)
    }

    /// Removes an existing address book entry.
    pub async fn remove_entry(
        &self,
        input: RemoveAddressBookEntryOperationInput,
    ) -> ServiceResult<AddressBookEntry> {
        let entry = self.get_entry_by_id(&input.address_book_entry_id)?;

        self.address_book_repository.remove(&entry.to_key());

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
    use wallet_api::{ChangeMetadataDTO, MetadataDTO};

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
                metadata: address_book_entry.metadata.clone().into_vec_dto(),
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

        let metadata_dto = vec![
            MetadataDTO {
                key: "a".to_string(),
                value: "b".to_string(),
            },
            MetadataDTO {
                key: "b".to_string(),
                value: "c".to_string(),
            },
        ];
        let operation = EditAddressBookEntryOperationInput {
            address_book_entry_id: address_book_entry.id,
            address_owner: Some("test_edit".to_string()),
            change_metadata: Some(ChangeMetadataDTO::ReplaceAllBy(metadata_dto.clone())),
        };
        let result = ctx.service.edit_entry(operation).await;
        assert!(result.is_ok());
        let updated_entry = result.unwrap();
        address_book_entry.address_owner = "test_edit".to_string();
        address_book_entry.metadata = metadata_dto.into();
        assert_eq!(updated_entry, address_book_entry);

        let diff_metadata_dto = vec![
            MetadataDTO {
                key: "a".to_string(),
                value: "d".to_string(),
            },
            MetadataDTO {
                key: "c".to_string(),
                value: "e".to_string(),
            },
        ];
        let new_metadata_dto = vec![
            MetadataDTO {
                key: "a".to_string(),
                value: "d".to_string(),
            },
            MetadataDTO {
                key: "b".to_string(),
                value: "c".to_string(),
            },
            MetadataDTO {
                key: "c".to_string(),
                value: "e".to_string(),
            },
        ];
        let operation = EditAddressBookEntryOperationInput {
            address_book_entry_id: address_book_entry.id,
            address_owner: None,
            change_metadata: Some(ChangeMetadataDTO::OverrideSpecifiedBy(diff_metadata_dto)),
        };
        let result = ctx.service.edit_entry(operation).await;
        assert!(result.is_ok());
        let updated_entry = result.unwrap();
        address_book_entry.metadata = new_metadata_dto.into();
        assert_eq!(updated_entry, address_book_entry);

        let remove_keys = vec!["a".to_string(), "c".to_string()];
        let new_metadata_dto = vec![MetadataDTO {
            key: "b".to_string(),
            value: "c".to_string(),
        }];
        let operation = EditAddressBookEntryOperationInput {
            address_book_entry_id: address_book_entry.id,
            address_owner: None,
            change_metadata: Some(ChangeMetadataDTO::RemoveKeys(remove_keys)),
        };
        let result = ctx.service.edit_entry(operation).await;
        assert!(result.is_ok());
        let updated_entry = result.unwrap();
        address_book_entry.metadata = new_metadata_dto.into();
        assert_eq!(updated_entry, address_book_entry);
    }

    #[tokio::test]
    async fn remove_entry() {
        let ctx = setup();
        let address_book_entry = mock_address_book_entry();

        ctx.repository
            .insert(address_book_entry.to_key(), address_book_entry.clone());

        ctx.service.get_entry_by_id(&address_book_entry.id).unwrap();

        let operation = RemoveAddressBookEntryOperationInput {
            address_book_entry_id: address_book_entry.id,
        };

        ctx.service.remove_entry(operation).await.unwrap();

        ctx.service
            .get_entry_by_id(&address_book_entry.id)
            .unwrap_err();
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
