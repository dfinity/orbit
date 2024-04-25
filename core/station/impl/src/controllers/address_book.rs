use crate::mappers::HelperMapper;
use crate::models::resource::{Resource, ResourceAction};
use crate::models::ListAddressBookEntriesInput;
use crate::{
    core::middlewares::{authorize, call_context},
    services::AddressBookService,
};
use ic_canister_core::api::ApiResult;
use ic_canister_macros::with_middleware;
use ic_cdk_macros::query;
use lazy_static::lazy_static;
use station_api::{
    AddressBookEntryCallerPrivilegesDTO, GetAddressBookEntryInputDTO,
    GetAddressBookEntryResponseDTO, ListAddressBookEntriesInputDTO,
    ListAddressBookEntriesResponseDTO,
};

// Canister entrypoints for the controller.
#[query(name = "get_address_book_entry")]
async fn get_address_book_entry(
    input: GetAddressBookEntryInputDTO,
) -> ApiResult<GetAddressBookEntryResponseDTO> {
    CONTROLLER.get_address_book_entry(input).await
}

#[query(name = "list_address_book_entries")]
async fn list_address_book_entries(
    input: ListAddressBookEntriesInputDTO,
) -> ApiResult<ListAddressBookEntriesResponseDTO> {
    CONTROLLER.list_address_book_entries(input).await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: AddressBookController =
        AddressBookController::new(AddressBookService::default());
}

#[derive(Debug)]
pub struct AddressBookController {
    address_book_service: AddressBookService,
}

impl AddressBookController {
    pub fn new(address_book_service: AddressBookService) -> Self {
        Self {
            address_book_service,
        }
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::from(&input)]))]
    async fn get_address_book_entry(
        &self,
        input: GetAddressBookEntryInputDTO,
    ) -> ApiResult<GetAddressBookEntryResponseDTO> {
        let ctx = call_context();
        let address_book_entry_id = HelperMapper::to_uuid(input.address_book_entry_id)?;

        let address_book_entry = self
            .address_book_service
            .get_entry_by_id(address_book_entry_id.as_bytes())?
            .to_dto();
        let privileges = self
            .address_book_service
            .get_caller_privileges_for_entry(address_book_entry_id.as_bytes(), &ctx)
            .await?;

        Ok(GetAddressBookEntryResponseDTO {
            address_book_entry,
            privileges: privileges.into(),
        })
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::AddressBook(ResourceAction::List)]))]
    async fn list_address_book_entries(
        &self,
        input_dto: ListAddressBookEntriesInputDTO,
    ) -> ApiResult<ListAddressBookEntriesResponseDTO> {
        let ctx = call_context();
        let paginate = input_dto.paginate.clone();
        let input: ListAddressBookEntriesInput = input_dto.into();
        let result = self.address_book_service.search_entries(input, paginate)?;

        let mut privileges = Vec::new();
        for entry in &result.items {
            let privilege = self
                .address_book_service
                .get_caller_privileges_for_entry(&entry.id, &ctx)
                .await?;

            privileges.push(AddressBookEntryCallerPrivilegesDTO::from(privilege));
        }

        Ok(ListAddressBookEntriesResponseDTO {
            address_book_entries: result
                .items
                .into_iter()
                .map(|address_book_entry| address_book_entry.to_dto())
                .collect(),
            next_offset: result.next_offset,
            total: result.total,
            privileges,
        })
    }
}
