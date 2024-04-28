use crate::models::{RequestOperation, RequestOperationType};
use crate::{
    models::{NotificationType, Request},
    repositories::REQUEST_REPOSITORY,
};
use orbit_essentials::repository::Repository;
use station_api::{NotificationTypeDTO, RequestCreatedNotificationDTO};
use uuid::Uuid;

use super::notification::NotificationMapperError;

impl TryFrom<NotificationType> for NotificationTypeDTO {
    type Error = NotificationMapperError;
    fn try_from(model: NotificationType) -> Result<NotificationTypeDTO, NotificationMapperError> {
        Ok(match model {
            NotificationType::SystemMessage => NotificationTypeDTO::SystemMessage,
            NotificationType::RequestCreated(ctx) => {
                let request = REQUEST_REPOSITORY
                    .get(&Request::key(ctx.request_id))
                    .ok_or(NotificationMapperError::RequestNotFound {
                        request_id: ctx.request_id,
                    })?;

                let account_id = match &request.operation {
                    RequestOperation::Transfer(operation) => Some(operation.input.from_account_id),
                    RequestOperation::EditAccount(operation) => Some(operation.input.account_id),
                    RequestOperation::AddAccount(_)
                    | RequestOperation::AddAddressBookEntry(_)
                    | RequestOperation::EditAddressBookEntry(_)
                    | RequestOperation::RemoveAddressBookEntry(_)
                    | RequestOperation::EditUser(_)
                    | RequestOperation::AddRequestPolicy(_)
                    | RequestOperation::AddUser(_)
                    | RequestOperation::AddUserGroup(_)
                    | RequestOperation::EditPermission(_)
                    | RequestOperation::EditRequestPolicy(_)
                    | RequestOperation::EditUserGroup(_)
                    | RequestOperation::RemoveRequestPolicy(_)
                    | RequestOperation::RemoveUserGroup(_)
                    | RequestOperation::ChangeCanister(_) => None,
                };

                let user_id: Option<[u8; 16]> = match &request.operation {
                    RequestOperation::EditUser(operation) => Some(operation.input.user_id),
                    RequestOperation::AddAccount(_)
                    | RequestOperation::AddAddressBookEntry(_)
                    | RequestOperation::AddRequestPolicy(_)
                    | RequestOperation::AddUser(_)
                    | RequestOperation::AddUserGroup(_)
                    | RequestOperation::EditPermission(_)
                    | RequestOperation::EditAccount(_)
                    | RequestOperation::EditAddressBookEntry(_)
                    | RequestOperation::RemoveAddressBookEntry(_)
                    | RequestOperation::EditRequestPolicy(_)
                    | RequestOperation::EditUserGroup(_)
                    | RequestOperation::RemoveRequestPolicy(_)
                    | RequestOperation::RemoveUserGroup(_)
                    | RequestOperation::Transfer(_)
                    | RequestOperation::ChangeCanister(_) => None,
                };

                NotificationTypeDTO::RequestCreated(RequestCreatedNotificationDTO {
                    request_id: Uuid::from_bytes(ctx.request_id).to_string(),
                    operation_type: RequestOperationType::from(request.operation).into(),
                    account_id: account_id.map(|id| Uuid::from_bytes(id).to_string()),
                    user_id: user_id.map(|id| Uuid::from_bytes(id).to_string()),
                })
            }
        })
    }
}
