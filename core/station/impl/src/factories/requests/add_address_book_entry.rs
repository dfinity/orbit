use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{AddAddressBookEntryOperation, Request, RequestOperation},
    services::ADDRESS_BOOK_SERVICE,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

pub struct AddAddressBookEntryRequestCreate {}

#[async_trait]
impl Create<station_api::AddAddressBookEntryOperationInput> for AddAddressBookEntryRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::AddAddressBookEntryOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::AddAddressBookEntry(AddAddressBookEntryOperation {
                address_book_entry_id: None,
                input: operation_input.into(),
            }),
            "Add address book entry".to_string(),
        );

        Ok(request)
    }
}

pub struct AddAddressBookEntryRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o AddAddressBookEntryOperation,
}

impl<'p, 'o> AddAddressBookEntryRequestExecute<'p, 'o> {
    pub fn new(request: &'p Request, operation: &'o AddAddressBookEntryOperation) -> Self {
        Self { request, operation }
    }
}

#[async_trait]
impl Execute for AddAddressBookEntryRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        let address_book_entry = ADDRESS_BOOK_SERVICE
            .create_entry(self.operation.input.to_owned())
            .await
            .map_err(|e| RequestExecuteError::Failed {
                reason: format!("Failed to create address book entry: {}", e),
            })?;

        let mut operation = self.request.operation.clone();

        if let RequestOperation::AddAddressBookEntry(ref mut operation) = operation {
            operation.address_book_entry_id = Some(address_book_entry.id);
        }

        Ok(RequestExecuteStage::Completed(operation))
    }
}
