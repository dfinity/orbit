use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    mappers::HelperMapper,
    models::{
        RemoveAddressBookEntryOperation, RemoveAddressBookEntryOperationInput, Request,
        RequestOperation,
    },
    services::ADDRESS_BOOK_SERVICE,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

pub struct RemoveAddressBookEntryRequestCreate {}

#[async_trait]
impl Create<station_api::RemoveAddressBookEntryOperationInput>
    for RemoveAddressBookEntryRequestCreate
{
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::RemoveAddressBookEntryOperationInput,
    ) -> Result<Request, RequestError> {
        let address_book_entry_id = HelperMapper::to_uuid(operation_input.address_book_entry_id)
            .map_err(|e| RequestError::ValidationError {
                info: format!("Invalid address book entry id: {}", e),
            })?;

        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::RemoveAddressBookEntry(RemoveAddressBookEntryOperation {
                input: RemoveAddressBookEntryOperationInput {
                    address_book_entry_id: *address_book_entry_id.as_bytes(),
                },
            }),
            "Remove address book entry".to_string(),
        );

        Ok(request)
    }
}

pub struct RemoveAddressBookEntryRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o RemoveAddressBookEntryOperation,
}

impl<'p, 'o> RemoveAddressBookEntryRequestExecute<'p, 'o> {
    pub fn new(request: &'p Request, operation: &'o RemoveAddressBookEntryOperation) -> Self {
        Self { request, operation }
    }
}

#[async_trait]
impl Execute for RemoveAddressBookEntryRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        ADDRESS_BOOK_SERVICE
            .remove_entry(self.operation.input.to_owned())
            .await
            .map_err(|e| RequestExecuteError::Failed {
                reason: format!("Failed to remove address book entry: {}", e),
            })?;

        Ok(RequestExecuteStage::Completed(
            self.request.operation.clone(),
        ))
    }
}
