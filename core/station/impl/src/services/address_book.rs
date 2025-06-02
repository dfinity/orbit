use crate::{
    core::{
        authorization::Authorization,
        generate_uuid_v4,
        utils::{paginated_items, PaginatedData, PaginatedItemsArgs},
        CallContext,
    },
    errors::AddressBookError,
    mappers::address_book::AddressBookMapper,
    models::{
        resource::{Resource, ResourceAction, ResourceId},
        AddAddressBookEntryOperationInput, AddressBookEntry, AddressBookEntryCallerPrivileges,
        AddressBookEntryId, EditAddressBookEntryOperationInput, ListAddressBookEntriesInput,
        RemoveAddressBookEntryOperationInput,
    },
    repositories::{AddressBookRepository, AddressBookWhereClause, ADDRESS_BOOK_REPOSITORY},
};
use lazy_static::lazy_static;
use orbit_essentials::{api::ServiceResult, model::ModelValidator, repository::Repository};
use station_api::PaginationInput;
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
    pub async fn get_caller_privileges_for_entry(
        &self,
        id: &AddressBookEntryId,
        ctx: &CallContext,
    ) -> ServiceResult<AddressBookEntryCallerPrivileges> {
        Ok(AddressBookEntryCallerPrivileges {
            id: *id,
            can_edit: Authorization::is_allowed(
                ctx,
                &Resource::AddressBook(ResourceAction::Update(ResourceId::Id(*id))),
            ),
            can_delete: Authorization::is_allowed(
                ctx,
                &Resource::AddressBook(ResourceAction::Delete(ResourceId::Id(*id))),
            ),
        })
    }

    /// Returns all address book entries for the given blockchain standard.
    pub fn search_entries(
        &self,
        input: ListAddressBookEntriesInput,
        paginate: Option<PaginationInput>,
    ) -> ServiceResult<PaginatedData<AddressBookEntry>> {
        let mut entries = self
            .address_book_repository
            .find_where(AddressBookWhereClause {
                ids: input.ids,
                addresses: input.addresses,
                blockchain: input.blockchain,
                labels: input.labels,
                address_formats: input.address_formats,
            });

        if let Some(search_term) = input.search_term {
            let search_term = search_term.to_lowercase();
            entries.retain(|entry| {
                entry.address_owner.to_lowercase().contains(&search_term)
                    || entry.address.to_lowercase().contains(&search_term)
            });
        }

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

        if let Some(v) = self
            .address_book_repository
            .find_by_address(new_entry.blockchain.clone(), new_entry.address.clone())
        {
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
            address_owner.clone_into(&mut entry.address_owner);
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
            address_book_entry_test_utils::mock_address_book_entry,
            address_format_test_utils::VALID_ACCOUNT_IDENTIFIER, AddAddressBookEntryOperation,
            AddAddressBookEntryOperationInput, AddressFormat, Blockchain, ChangeMetadata, Metadata,
            MetadataItem,
        },
    };
    use station_api::MetadataDTO;

    struct TestContext {
        repository: AddressBookRepository,
        service: AddressBookService,
    }

    fn setup() -> TestContext {
        test_utils::init_canister_system();

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
                address: VALID_ACCOUNT_IDENTIFIER.to_string(),
                blockchain: Blockchain::InternetComputer,
                metadata: address_book_entry.metadata.clone().into(),
                labels: vec![],
                address_format: AddressFormat::ICPAccountIdentifier,
            },
        };

        let result = ctx.service.create_entry(operation.input.clone()).await;

        let new_entry = result.unwrap();

        address_book_entry.id = new_entry.id;
        address_book_entry.last_modification_timestamp = new_entry.last_modification_timestamp;

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

        let metadata_items = vec![
            MetadataItem {
                key: "a".to_string(),
                value: "b".to_string(),
            },
            MetadataItem {
                key: "b".to_string(),
                value: "c".to_string(),
            },
        ];
        let metadata = Metadata::from(metadata_items.clone());
        let operation = EditAddressBookEntryOperationInput {
            address_book_entry_id: address_book_entry.id,
            address_owner: Some("test_edit".to_string()),
            change_metadata: Some(ChangeMetadata::ReplaceAllBy(
                metadata.as_btreemap().to_owned(),
            )),
            labels: None,
        };
        let result = ctx.service.edit_entry(operation).await;
        assert!(result.is_ok());
        let updated_entry = result.unwrap();
        address_book_entry.address_owner = "test_edit".to_string();
        address_book_entry.metadata = metadata.clone();
        assert_eq!(updated_entry, address_book_entry);

        let diff_metadata_dto = Metadata::from(vec![
            MetadataDTO {
                key: "a".to_string(),
                value: "d".to_string(),
            },
            MetadataDTO {
                key: "c".to_string(),
                value: "e".to_string(),
            },
        ]);
        let new_metadata_dto = Metadata::from(vec![
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
        ]);

        let operation = EditAddressBookEntryOperationInput {
            address_book_entry_id: address_book_entry.id,
            address_owner: None,
            change_metadata: Some(ChangeMetadata::OverrideSpecifiedBy(
                diff_metadata_dto.as_btreemap().to_owned(),
            )),
            labels: None,
        };
        let result = ctx.service.edit_entry(operation).await;
        assert!(result.is_ok());
        let updated_entry = result.unwrap();
        address_book_entry.metadata = new_metadata_dto;
        assert_eq!(updated_entry, address_book_entry);

        let remove_keys = vec!["a".to_string(), "c".to_string()];
        let new_metadata_dto = vec![MetadataDTO {
            key: "b".to_string(),
            value: "c".to_string(),
        }];
        let operation = EditAddressBookEntryOperationInput {
            address_book_entry_id: address_book_entry.id,
            address_owner: None,
            change_metadata: Some(ChangeMetadata::RemoveKeys(remove_keys)),
            labels: None,
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
}
