use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    mappers::HelperMapper,
    models::{
        EditAddressBookEntryOperation, EditAddressBookEntryOperationInput, Request,
        RequestExecutionPlan, RequestOperation,
    },
    services::ADDRESS_BOOK_SERVICE,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

pub struct EditAddressBookEntryRequestCreate {}

#[async_trait]
impl Create<station_api::EditAddressBookEntryOperationInput> for EditAddressBookEntryRequestCreate {
    async fn create(
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::EditAddressBookEntryOperationInput,
    ) -> Result<Request, RequestError> {
        let address_book_entry_id = HelperMapper::to_uuid(operation_input.address_book_entry_id)
            .map_err(|e| RequestError::ValidationError {
                info: format!("Invalid address book entry id: {}", e),
            })?;

        let request = Request::new(
            request_id,
            requested_by_user,
            Request::default_expiration_dt_ns(),
            RequestOperation::EditAddressBookEntry(EditAddressBookEntryOperation {
                input: EditAddressBookEntryOperationInput {
                    address_book_entry_id: *address_book_entry_id.as_bytes(),
                    address_owner: operation_input.address_owner,
                    change_metadata: operation_input.change_metadata.map(|m| m.into()),
                },
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(RequestExecutionPlan::Immediate),
            input
                .title
                .unwrap_or_else(|| "Address book entry update".to_string()),
            input.summary,
        );

        Ok(request)
    }
}

pub struct EditAddressBookEntryRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o EditAddressBookEntryOperation,
}

impl<'p, 'o> EditAddressBookEntryRequestExecute<'p, 'o> {
    pub fn new(request: &'p Request, operation: &'o EditAddressBookEntryOperation) -> Self {
        Self { request, operation }
    }
}

#[async_trait]
impl Execute for EditAddressBookEntryRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        ADDRESS_BOOK_SERVICE
            .edit_entry(self.operation.input.to_owned())
            .await
            .map_err(|e| RequestExecuteError::Failed {
                reason: format!("Failed to update address book entry: {}", e),
            })?;

        Ok(RequestExecuteStage::Completed(
            self.request.operation.clone(),
        ))
    }
}
