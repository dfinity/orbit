use crate::mappers::HelperMapper;
use crate::models::request_operation_filter_type::RequestOperationFilterType;
use crate::models::{RequestOperation, RequestOperationType};
use station_api::{ListRequestsOperationTypeDTO, RequestOperationTypeDTO};

impl From<RequestOperationTypeDTO> for RequestOperationType {
    fn from(dto: RequestOperationTypeDTO) -> Self {
        match dto {
            RequestOperationTypeDTO::Transfer => RequestOperationType::Transfer,
            RequestOperationTypeDTO::AddAccount => RequestOperationType::AddAccount,
            RequestOperationTypeDTO::EditAccount => RequestOperationType::EditAccount,
            RequestOperationTypeDTO::AddAddressBookEntry => {
                RequestOperationType::AddAddressBookEntry
            }
            RequestOperationTypeDTO::EditAddressBookEntry => {
                RequestOperationType::EditAddressBookEntry
            }
            RequestOperationTypeDTO::RemoveAddressBookEntry => {
                RequestOperationType::RemoveAddressBookEntry
            }
            RequestOperationTypeDTO::AddUser => RequestOperationType::AddUser,
            RequestOperationTypeDTO::EditUser => RequestOperationType::EditUser,
            RequestOperationTypeDTO::AddUserGroup => RequestOperationType::AddUserGroup,
            RequestOperationTypeDTO::EditUserGroup => RequestOperationType::EditUserGroup,
            RequestOperationTypeDTO::RemoveUserGroup => RequestOperationType::RemoveUserGroup,
            RequestOperationTypeDTO::ChangeCanister => RequestOperationType::ChangeCanister,
            RequestOperationTypeDTO::ChangeExternalCanister => {
                RequestOperationType::ChangeExternalCanister
            }
            RequestOperationTypeDTO::CreateExternalCanister => {
                RequestOperationType::CreateExternalCanister
            }
            RequestOperationTypeDTO::CallExternalCanister => {
                RequestOperationType::CallExternalCanister
            }
            RequestOperationTypeDTO::EditPermission => RequestOperationType::EditPermission,
            RequestOperationTypeDTO::AddRequestPolicy => RequestOperationType::AddRequestPolicy,
            RequestOperationTypeDTO::EditRequestPolicy => RequestOperationType::EditRequestPolicy,
            RequestOperationTypeDTO::RemoveRequestPolicy => {
                RequestOperationType::RemoveRequestPolicy
            }
            RequestOperationTypeDTO::ManageSystemInfo => RequestOperationType::ManageSystemInfo,
            RequestOperationTypeDTO::SetDisasterRecovery => {
                RequestOperationType::SetDisasterRecovery
            }
            RequestOperationTypeDTO::ConfigureExternalCanister => {
                RequestOperationType::ConfigureExternalCanister
            }
        }
    }
}

impl From<RequestOperationType> for RequestOperationTypeDTO {
    fn from(operation_type: RequestOperationType) -> Self {
        match operation_type {
            RequestOperationType::Transfer => RequestOperationTypeDTO::Transfer,
            RequestOperationType::AddAccount => RequestOperationTypeDTO::AddAccount,
            RequestOperationType::EditAccount => RequestOperationTypeDTO::EditAccount,
            RequestOperationType::AddAddressBookEntry => {
                RequestOperationTypeDTO::AddAddressBookEntry
            }
            RequestOperationType::EditAddressBookEntry => {
                RequestOperationTypeDTO::EditAddressBookEntry
            }
            RequestOperationType::RemoveAddressBookEntry => {
                RequestOperationTypeDTO::RemoveAddressBookEntry
            }
            RequestOperationType::AddUser => RequestOperationTypeDTO::AddUser,
            RequestOperationType::EditUser => RequestOperationTypeDTO::EditUser,
            RequestOperationType::AddUserGroup => RequestOperationTypeDTO::AddUserGroup,
            RequestOperationType::EditUserGroup => RequestOperationTypeDTO::EditUserGroup,
            RequestOperationType::RemoveUserGroup => RequestOperationTypeDTO::RemoveUserGroup,
            RequestOperationType::ChangeCanister => RequestOperationTypeDTO::ChangeCanister,
            RequestOperationType::ChangeExternalCanister => {
                RequestOperationTypeDTO::ChangeExternalCanister
            }
            RequestOperationType::CreateExternalCanister => {
                RequestOperationTypeDTO::CreateExternalCanister
            }
            RequestOperationType::CallExternalCanister => {
                RequestOperationTypeDTO::CallExternalCanister
            }
            RequestOperationType::EditPermission => RequestOperationTypeDTO::EditPermission,
            RequestOperationType::AddRequestPolicy => RequestOperationTypeDTO::AddRequestPolicy,
            RequestOperationType::EditRequestPolicy => RequestOperationTypeDTO::EditRequestPolicy,
            RequestOperationType::RemoveRequestPolicy => {
                RequestOperationTypeDTO::RemoveRequestPolicy
            }
            RequestOperationType::ManageSystemInfo => RequestOperationTypeDTO::ManageSystemInfo,
            RequestOperationType::SetDisasterRecovery => {
                RequestOperationTypeDTO::SetDisasterRecovery
            }
            RequestOperationType::ConfigureExternalCanister => {
                RequestOperationTypeDTO::ConfigureExternalCanister
            }
        }
    }
}

impl From<RequestOperation> for RequestOperationType {
    fn from(operation: RequestOperation) -> Self {
        match operation {
            RequestOperation::Transfer(_) => RequestOperationType::Transfer,
            RequestOperation::AddAccount(_) => RequestOperationType::AddAccount,
            RequestOperation::EditAccount(_) => RequestOperationType::EditAccount,
            RequestOperation::AddAddressBookEntry(_) => RequestOperationType::AddAddressBookEntry,
            RequestOperation::EditAddressBookEntry(_) => RequestOperationType::EditAddressBookEntry,
            RequestOperation::RemoveAddressBookEntry(_) => {
                RequestOperationType::RemoveAddressBookEntry
            }
            RequestOperation::AddUser(_) => RequestOperationType::AddUser,
            RequestOperation::EditUser(_) => RequestOperationType::EditUser,
            RequestOperation::AddUserGroup(_) => RequestOperationType::AddUserGroup,
            RequestOperation::EditUserGroup(_) => RequestOperationType::EditUserGroup,
            RequestOperation::RemoveUserGroup(_) => RequestOperationType::RemoveUserGroup,
            RequestOperation::ChangeCanister(_) => RequestOperationType::ChangeCanister,
            RequestOperation::ChangeExternalCanister(_) => {
                RequestOperationType::ChangeExternalCanister
            }
            RequestOperation::ConfigureExternalCanister(_) => {
                RequestOperationType::ConfigureExternalCanister
            }
            RequestOperation::CreateExternalCanister(_) => {
                RequestOperationType::CreateExternalCanister
            }
            RequestOperation::CallExternalCanister(_) => RequestOperationType::CallExternalCanister,
            RequestOperation::EditPermission(_) => RequestOperationType::EditPermission,
            RequestOperation::AddRequestPolicy(_) => RequestOperationType::AddRequestPolicy,
            RequestOperation::EditRequestPolicy(_) => RequestOperationType::EditRequestPolicy,
            RequestOperation::RemoveRequestPolicy(_) => RequestOperationType::RemoveRequestPolicy,
            RequestOperation::ManageSystemInfo(_) => RequestOperationType::ManageSystemInfo,
            RequestOperation::SetDisasterRecovery(_) => RequestOperationType::SetDisasterRecovery,
        }
    }
}

impl RequestOperation {
    pub fn is_of_type(&self, operation: &ListRequestsOperationTypeDTO) -> bool {
        match (self, operation) {
            (
                RequestOperation::Transfer(transfer_operation),
                ListRequestsOperationTypeDTO::Transfer(from_account_id),
            ) => {
                if let Some(account_id) = from_account_id {
                    HelperMapper::to_uuid(account_id.clone()).map(|uuid| *uuid.as_bytes())
                        == Ok(transfer_operation.input.from_account_id)
                } else {
                    true
                }
            }
            (RequestOperation::AddAccount(_), ListRequestsOperationTypeDTO::AddAccount) => true,
            (RequestOperation::EditAccount(_), ListRequestsOperationTypeDTO::EditAccount) => true,
            (
                RequestOperation::AddAddressBookEntry(_),
                ListRequestsOperationTypeDTO::AddAddressBookEntry,
            ) => true,
            (
                RequestOperation::EditAddressBookEntry(_),
                ListRequestsOperationTypeDTO::EditAddressBookEntry,
            ) => true,
            (
                RequestOperation::RemoveAddressBookEntry(_),
                ListRequestsOperationTypeDTO::RemoveAddressBookEntry,
            ) => true,
            (RequestOperation::AddUser(_), ListRequestsOperationTypeDTO::AddUser) => true,
            (RequestOperation::EditUser(_), ListRequestsOperationTypeDTO::EditUser) => true,
            (RequestOperation::AddUserGroup(_), ListRequestsOperationTypeDTO::AddUserGroup) => true,
            (RequestOperation::EditUserGroup(_), ListRequestsOperationTypeDTO::EditUserGroup) => {
                true
            }
            (
                RequestOperation::RemoveUserGroup(_),
                ListRequestsOperationTypeDTO::RemoveUserGroup,
            ) => true,
            (RequestOperation::ChangeCanister(_), ListRequestsOperationTypeDTO::ChangeCanister) => {
                true
            }
            (
                RequestOperation::ChangeExternalCanister(operation),
                ListRequestsOperationTypeDTO::ChangeExternalCanister(target),
            ) => {
                if let Some(canister_id) = target {
                    operation.input.canister_id == *canister_id
                } else {
                    true
                }
            }
            (
                RequestOperation::CreateExternalCanister(_),
                ListRequestsOperationTypeDTO::CreateExternalCanister,
            ) => true,
            (
                RequestOperation::CallExternalCanister(operation),
                ListRequestsOperationTypeDTO::CallExternalCanister(target),
            ) => {
                if let Some(canister_id) = target {
                    operation.input.execution_method.canister_id == *canister_id
                } else {
                    true
                }
            }
            (RequestOperation::EditPermission(_), ListRequestsOperationTypeDTO::EditPermission) => {
                true
            }
            (
                RequestOperation::AddRequestPolicy(_),
                ListRequestsOperationTypeDTO::AddRequestPolicy,
            ) => true,
            (
                RequestOperation::EditRequestPolicy(_),
                ListRequestsOperationTypeDTO::EditRequestPolicy,
            ) => true,
            (
                RequestOperation::RemoveRequestPolicy(_),
                ListRequestsOperationTypeDTO::RemoveRequestPolicy,
            ) => true,
            (
                RequestOperation::ManageSystemInfo(_),
                ListRequestsOperationTypeDTO::ManageSystemInfo,
            ) => true,
            _ => false,
        }
    }
}

impl From<station_api::ListRequestsOperationTypeDTO> for RequestOperationFilterType {
    fn from(dto: station_api::ListRequestsOperationTypeDTO) -> Self {
        match dto {
            station_api::ListRequestsOperationTypeDTO::Transfer(from_account_id) => {
                RequestOperationFilterType::Transfer(from_account_id.map(|id| {
                    *HelperMapper::to_uuid(id)
                        .expect("Invalid account id")
                        .as_bytes()
                }))
            }
            station_api::ListRequestsOperationTypeDTO::AddAccount => {
                RequestOperationFilterType::AddAccount
            }
            station_api::ListRequestsOperationTypeDTO::EditAccount => {
                RequestOperationFilterType::EditAccount
            }
            station_api::ListRequestsOperationTypeDTO::AddAddressBookEntry => {
                RequestOperationFilterType::AddAddressBookEntry
            }
            station_api::ListRequestsOperationTypeDTO::EditAddressBookEntry => {
                RequestOperationFilterType::EditAddressBookEntry
            }
            station_api::ListRequestsOperationTypeDTO::RemoveAddressBookEntry => {
                RequestOperationFilterType::RemoveAddressBookEntry
            }
            station_api::ListRequestsOperationTypeDTO::AddUser => {
                RequestOperationFilterType::AddUser
            }
            station_api::ListRequestsOperationTypeDTO::EditUser => {
                RequestOperationFilterType::EditUser
            }
            station_api::ListRequestsOperationTypeDTO::AddUserGroup => {
                RequestOperationFilterType::AddUserGroup
            }
            station_api::ListRequestsOperationTypeDTO::EditUserGroup => {
                RequestOperationFilterType::EditUserGroup
            }
            station_api::ListRequestsOperationTypeDTO::RemoveUserGroup => {
                RequestOperationFilterType::RemoveUserGroup
            }
            station_api::ListRequestsOperationTypeDTO::ChangeCanister => {
                RequestOperationFilterType::ChangeCanister
            }
            station_api::ListRequestsOperationTypeDTO::ChangeExternalCanister(target) => {
                RequestOperationFilterType::ChangeExternalCanister(target)
            }
            station_api::ListRequestsOperationTypeDTO::CreateExternalCanister => {
                RequestOperationFilterType::CreateExternalCanister
            }
            station_api::ListRequestsOperationTypeDTO::CallExternalCanister(target) => {
                RequestOperationFilterType::CallExternalCanister(target)
            }
            station_api::ListRequestsOperationTypeDTO::ConfigureExternalCanister(target) => {
                RequestOperationFilterType::ConfigureExternalCanister(target)
            }
            station_api::ListRequestsOperationTypeDTO::EditPermission => {
                RequestOperationFilterType::EditPermission
            }
            station_api::ListRequestsOperationTypeDTO::AddRequestPolicy => {
                RequestOperationFilterType::AddRequestPolicy
            }
            station_api::ListRequestsOperationTypeDTO::EditRequestPolicy => {
                RequestOperationFilterType::EditRequestPolicy
            }
            station_api::ListRequestsOperationTypeDTO::RemoveRequestPolicy => {
                RequestOperationFilterType::RemoveRequestPolicy
            }
            station_api::ListRequestsOperationTypeDTO::ManageSystemInfo => {
                RequestOperationFilterType::ManageSystemInfo
            }
            station_api::ListRequestsOperationTypeDTO::SetDisasterRecovery => {
                RequestOperationFilterType::SetDisasterRecovery
            }
        }
    }
}
