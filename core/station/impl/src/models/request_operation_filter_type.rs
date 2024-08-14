use super::{AccountId, RequestOperation};
use candid::Principal;
use orbit_essentials::storable;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RequestOperationFilterType {
    Transfer(AccountId),
    AddAccount,
    EditAccount,
    AddUser,
    EditUser,
    AddUserGroup,
    EditUserGroup,
    RemoveUserGroup,
    ChangeCanister,
    SetDisasterRecovery,
    ChangeExternalCanister(Principal),
    CreateExternalCanister,
    CallExternalCanister(Principal),
    EditPermission,
    AddRequestPolicy,
    EditRequestPolicy,
    RemoveRequestPolicy,
    AddAddressBookEntry,
    EditAddressBookEntry,
    RemoveAddressBookEntry,
    ManageSystemInfo,
    ConfigureExternalCanister(Principal),
    FundExternalCanister(Principal),
}

impl From<RequestOperation> for RequestOperationFilterType {
    fn from(operation: RequestOperation) -> Self {
        match operation {
            RequestOperation::Transfer(operation) => {
                RequestOperationFilterType::Transfer(operation.input.from_account_id)
            }
            RequestOperation::AddAccount(_) => RequestOperationFilterType::AddAccount,
            RequestOperation::EditAccount(_) => RequestOperationFilterType::EditAccount,
            RequestOperation::AddUser(_) => RequestOperationFilterType::AddUser,
            RequestOperation::EditUser(_) => RequestOperationFilterType::EditUser,
            RequestOperation::AddUserGroup(_) => RequestOperationFilterType::AddUserGroup,
            RequestOperation::EditUserGroup(_) => RequestOperationFilterType::EditUserGroup,
            RequestOperation::RemoveUserGroup(_) => RequestOperationFilterType::RemoveUserGroup,
            RequestOperation::ChangeCanister(_) => RequestOperationFilterType::ChangeCanister,
            RequestOperation::SetDisasterRecovery(_) => {
                RequestOperationFilterType::SetDisasterRecovery
            }
            RequestOperation::ChangeExternalCanister(operation) => {
                RequestOperationFilterType::ChangeExternalCanister(operation.input.canister_id)
            }
            RequestOperation::CreateExternalCanister(_) => {
                RequestOperationFilterType::CreateExternalCanister
            }
            RequestOperation::CallExternalCanister(operation) => {
                RequestOperationFilterType::CallExternalCanister(
                    operation.input.execution_method.canister_id,
                )
            }
            RequestOperation::EditPermission(_) => RequestOperationFilterType::EditPermission,
            RequestOperation::AddRequestPolicy(_) => RequestOperationFilterType::AddRequestPolicy,
            RequestOperation::EditRequestPolicy(_) => RequestOperationFilterType::EditRequestPolicy,
            RequestOperation::RemoveRequestPolicy(_) => {
                RequestOperationFilterType::RemoveRequestPolicy
            }
            RequestOperation::AddAddressBookEntry(_) => {
                RequestOperationFilterType::AddAddressBookEntry
            }
            RequestOperation::EditAddressBookEntry(_) => {
                RequestOperationFilterType::EditAddressBookEntry
            }
            RequestOperation::RemoveAddressBookEntry(_) => {
                RequestOperationFilterType::RemoveAddressBookEntry
            }
            RequestOperation::ManageSystemInfo(_) => RequestOperationFilterType::ManageSystemInfo,
            RequestOperation::ConfigureExternalCanister(operation) => {
                RequestOperationFilterType::ConfigureExternalCanister(operation.canister_id)
            }
            RequestOperation::FundExternalCanister(operation) => {
                RequestOperationFilterType::FundExternalCanister(operation.canister_id)
            }
        }
    }
}
