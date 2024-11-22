use candid::Principal;
use orbit_essentials::storable;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use super::{AccountId, RequestOperationFilterType};

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RequestOperationType {
    Transfer = 0,
    AddAccount = 1,
    EditAccount = 2,
    AddUser = 3,
    EditUser = 4,
    AddUserGroup = 6,
    EditUserGroup = 7,
    RemoveUserGroup = 8,
    SystemUpgrade = 9,
    EditPermission = 11,
    AddRequestPolicy = 13,
    EditRequestPolicy = 14,
    RemoveRequestPolicy = 15,
    AddAddressBookEntry = 16,
    EditAddressBookEntry = 17,
    RemoveAddressBookEntry = 18,
    ManageSystemInfo = 19,
    ChangeExternalCanister = 20,
    CreateExternalCanister = 21,
    CallExternalCanister = 22,
    SetDisasterRecovery = 23,
    ConfigureExternalCanister = 24,
    FundExternalCanister = 25,
    SnapshotExternalCanister = 26,
    RestoreExternalCanister = 27,
}

/// A helper enum to filter the requests based on the operation type and
/// optional additional data (e.g. account id).
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ListRequestsOperationType {
    Transfer(Option<AccountId>),
    AddAccount,
    EditAccount,
    AddUser,
    EditUser,
    AddUserGroup,
    EditUserGroup,
    RemoveUserGroup,
    SystemUpgrade,
    SetDisasterRecovery,
    CreateExternalCanister,
    ChangeExternalCanister(Option<Principal>),
    CallExternalCanister(Option<Principal>),
    ConfigureExternalCanister(Option<Principal>),
    FundExternalCanister(Option<Principal>),
    SnapshotExternalCanister(Option<Principal>),
    RestoreExternalCanister(Option<Principal>),
    EditPermission,
    AddRequestPolicy,
    EditRequestPolicy,
    RemoveRequestPolicy,
    AddAddressBookEntry,
    EditAddressBookEntry,
    RemoveAddressBookEntry,
    ManageSystemInfo,
}

impl PartialEq<ListRequestsOperationType> for RequestOperationFilterType {
    fn eq(&self, other: &ListRequestsOperationType) -> bool {
        match other {
            ListRequestsOperationType::Transfer(None) => {
                matches!(self, RequestOperationFilterType::Transfer(_))
            }
            ListRequestsOperationType::Transfer(Some(account_id)) => {
                matches!(self, RequestOperationFilterType::Transfer(id) if id == account_id)
            }
            ListRequestsOperationType::AddAccount => {
                matches!(self, RequestOperationFilterType::AddAccount)
            }
            ListRequestsOperationType::EditAccount => {
                matches!(self, RequestOperationFilterType::EditAccount)
            }
            ListRequestsOperationType::AddUser => {
                matches!(self, RequestOperationFilterType::AddUser)
            }
            ListRequestsOperationType::EditUser => {
                matches!(self, RequestOperationFilterType::EditUser)
            }
            ListRequestsOperationType::AddUserGroup => {
                matches!(self, RequestOperationFilterType::AddUserGroup)
            }
            ListRequestsOperationType::EditUserGroup => {
                matches!(self, RequestOperationFilterType::EditUserGroup)
            }
            ListRequestsOperationType::RemoveUserGroup => {
                matches!(self, RequestOperationFilterType::RemoveUserGroup)
            }
            ListRequestsOperationType::SystemUpgrade => {
                matches!(self, RequestOperationFilterType::SystemUpgrade)
            }
            ListRequestsOperationType::SetDisasterRecovery => {
                matches!(self, RequestOperationFilterType::SetDisasterRecovery)
            }
            ListRequestsOperationType::CreateExternalCanister => {
                matches!(self, RequestOperationFilterType::CreateExternalCanister)
            }
            ListRequestsOperationType::ChangeExternalCanister(None) => {
                matches!(self, RequestOperationFilterType::ChangeExternalCanister(_))
            }
            ListRequestsOperationType::ChangeExternalCanister(Some(canister_id)) => {
                matches!(
                    self,
                    RequestOperationFilterType::ChangeExternalCanister(id) if id == canister_id
                )
            }
            ListRequestsOperationType::CallExternalCanister(None) => {
                matches!(self, RequestOperationFilterType::CallExternalCanister(_))
            }
            ListRequestsOperationType::CallExternalCanister(Some(canister_id)) => {
                matches!(
                    self,
                    RequestOperationFilterType::CallExternalCanister(id) if id == canister_id
                )
            }
            ListRequestsOperationType::ConfigureExternalCanister(None) => matches!(
                self,
                RequestOperationFilterType::ConfigureExternalCanister(_)
            ),
            ListRequestsOperationType::ConfigureExternalCanister(Some(canister_id)) => {
                matches!(
                    self,
                    RequestOperationFilterType::ConfigureExternalCanister(id) if id == canister_id
                )
            }
            ListRequestsOperationType::FundExternalCanister(None) => {
                matches!(self, RequestOperationFilterType::FundExternalCanister(_))
            }
            ListRequestsOperationType::FundExternalCanister(Some(canister_id)) => {
                matches!(
                    self,
                    RequestOperationFilterType::FundExternalCanister(id) if id == canister_id
                )
            }
            ListRequestsOperationType::SnapshotExternalCanister(None) => {
                matches!(
                    self,
                    RequestOperationFilterType::SnapshotExternalCanister(_)
                )
            }
            ListRequestsOperationType::SnapshotExternalCanister(Some(canister_id)) => {
                matches!(
                    self,
                    RequestOperationFilterType::SnapshotExternalCanister(id) if id == canister_id
                )
            }
            ListRequestsOperationType::RestoreExternalCanister(None) => {
                matches!(self, RequestOperationFilterType::RestoreExternalCanister(_))
            }
            ListRequestsOperationType::RestoreExternalCanister(Some(canister_id)) => {
                matches!(
                    self,
                    RequestOperationFilterType::RestoreExternalCanister(id) if id == canister_id
                )
            }
            ListRequestsOperationType::EditPermission => {
                matches!(self, RequestOperationFilterType::EditPermission)
            }
            ListRequestsOperationType::AddRequestPolicy => {
                matches!(self, RequestOperationFilterType::AddRequestPolicy)
            }
            ListRequestsOperationType::EditRequestPolicy => {
                matches!(self, RequestOperationFilterType::EditRequestPolicy)
            }
            ListRequestsOperationType::RemoveRequestPolicy => {
                matches!(self, RequestOperationFilterType::RemoveRequestPolicy)
            }
            ListRequestsOperationType::AddAddressBookEntry => {
                matches!(self, RequestOperationFilterType::AddAddressBookEntry)
            }
            ListRequestsOperationType::EditAddressBookEntry => {
                matches!(self, RequestOperationFilterType::EditAddressBookEntry)
            }
            ListRequestsOperationType::RemoveAddressBookEntry => {
                matches!(self, RequestOperationFilterType::RemoveAddressBookEntry)
            }
            ListRequestsOperationType::ManageSystemInfo => {
                matches!(self, RequestOperationFilterType::ManageSystemInfo)
            }
        }
    }
}

impl FromStr for RequestOperationType {
    type Err = ();

    fn from_str(variant: &str) -> Result<RequestOperationType, Self::Err> {
        match variant {
            "transfer" => Ok(RequestOperationType::Transfer),
            "add_account" => Ok(RequestOperationType::AddAccount),
            "edit_account" => Ok(RequestOperationType::EditAccount),
            "add_address_book_entry" => Ok(RequestOperationType::AddAddressBookEntry),
            "edit_address_book_entry" => Ok(RequestOperationType::EditAddressBookEntry),
            "remove_address_book_entry" => Ok(RequestOperationType::RemoveAddressBookEntry),
            "add_user" => Ok(RequestOperationType::AddUser),
            "edit_user" => Ok(RequestOperationType::EditUser),
            "add_user_group" => Ok(RequestOperationType::AddUserGroup),
            "edit_user_group" => Ok(RequestOperationType::EditUserGroup),
            "remove_user_group" => Ok(RequestOperationType::RemoveUserGroup),
            "system_upgrade" => Ok(RequestOperationType::SystemUpgrade),
            "change_external_canister" => Ok(RequestOperationType::ChangeExternalCanister),
            "create_external_canister" => Ok(RequestOperationType::CreateExternalCanister),
            "call_external_canister" => Ok(RequestOperationType::CallExternalCanister),
            "edit_permission" => Ok(RequestOperationType::EditPermission),
            "add_request_policy" => Ok(RequestOperationType::AddRequestPolicy),
            "edit_request_policy" => Ok(RequestOperationType::EditRequestPolicy),
            "remove_request_policy" => Ok(RequestOperationType::RemoveRequestPolicy),
            "manage_system_info" => Ok(RequestOperationType::ManageSystemInfo),
            "set_disaster_recovery_committee" => Ok(RequestOperationType::SetDisasterRecovery),
            "configure_external_canister" => Ok(RequestOperationType::ConfigureExternalCanister),
            "fund_external_canister" => Ok(RequestOperationType::FundExternalCanister),
            _ => Err(()),
        }
    }
}

impl Display for RequestOperationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestOperationType::Transfer => write!(f, "transfer"),
            RequestOperationType::AddAccount => write!(f, "add_account"),
            RequestOperationType::EditAccount => write!(f, "edit_account"),
            RequestOperationType::AddAddressBookEntry => write!(f, "add_address_book_entry"),
            RequestOperationType::EditAddressBookEntry => write!(f, "edit_address_book_entry"),
            RequestOperationType::RemoveAddressBookEntry => write!(f, "remove_address_book_entry"),
            RequestOperationType::AddUser => write!(f, "add_user"),
            RequestOperationType::EditUser => write!(f, "edit_user"),
            RequestOperationType::AddUserGroup => write!(f, "add_user_group"),
            RequestOperationType::EditUserGroup => write!(f, "edit_user_group"),
            RequestOperationType::RemoveUserGroup => write!(f, "remove_user_group"),
            RequestOperationType::SystemUpgrade => write!(f, "system_upgrade"),
            RequestOperationType::ChangeExternalCanister => write!(f, "change_external_canister"),
            RequestOperationType::CreateExternalCanister => write!(f, "create_external_canister"),
            RequestOperationType::CallExternalCanister => write!(f, "call_external_canister"),
            RequestOperationType::SnapshotExternalCanister => {
                write!(f, "snapshot_external_canister")
            }
            RequestOperationType::RestoreExternalCanister => {
                write!(f, "restore_external_canister")
            }
            RequestOperationType::EditPermission => write!(f, "edit_permission"),
            RequestOperationType::AddRequestPolicy => write!(f, "add_request_policy"),
            RequestOperationType::EditRequestPolicy => write!(f, "edit_request_policy"),
            RequestOperationType::RemoveRequestPolicy => write!(f, "remove_request_policy"),
            RequestOperationType::ManageSystemInfo => write!(f, "manage_system_info"),
            RequestOperationType::SetDisasterRecovery => {
                write!(f, "set_disaster_recovery_committee")
            }
            RequestOperationType::ConfigureExternalCanister => {
                write!(f, "configure_external_canister")
            }
            RequestOperationType::FundExternalCanister => write!(f, "fund_external_canister"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation_code_match_string_representation() {
        assert_eq!(RequestOperationType::Transfer.to_string(), "transfer");
        assert_eq!(
            RequestOperationType::from_str("transfer").unwrap(),
            RequestOperationType::Transfer
        );
        assert_eq!(
            RequestOperationType::EditAccount.to_string(),
            "edit_account"
        );
        assert_eq!(
            RequestOperationType::from_str("edit_account").unwrap(),
            RequestOperationType::EditAccount
        );
        assert_eq!(RequestOperationType::AddAccount.to_string(), "add_account");
        assert_eq!(
            RequestOperationType::from_str("add_account").unwrap(),
            RequestOperationType::AddAccount
        );
        assert_eq!(
            RequestOperationType::AddAddressBookEntry.to_string(),
            "add_address_book_entry"
        );
        assert_eq!(
            RequestOperationType::from_str("add_address_book_entry").unwrap(),
            RequestOperationType::AddAddressBookEntry
        );
        assert_eq!(
            RequestOperationType::EditAddressBookEntry.to_string(),
            "edit_address_book_entry"
        );
        assert_eq!(
            RequestOperationType::from_str("edit_address_book_entry").unwrap(),
            RequestOperationType::EditAddressBookEntry
        );
        assert_eq!(
            RequestOperationType::RemoveAddressBookEntry.to_string(),
            "remove_address_book_entry"
        );
        assert_eq!(
            RequestOperationType::from_str("remove_address_book_entry").unwrap(),
            RequestOperationType::RemoveAddressBookEntry
        );
        assert_eq!(RequestOperationType::AddUser.to_string(), "add_user");
        assert_eq!(
            RequestOperationType::from_str("add_user").unwrap(),
            RequestOperationType::AddUser
        );
        assert_eq!(RequestOperationType::EditUser.to_string(), "edit_user");
        assert_eq!(
            RequestOperationType::from_str("edit_user").unwrap(),
            RequestOperationType::EditUser
        );
        assert_eq!(
            RequestOperationType::AddUserGroup.to_string(),
            "add_user_group"
        );
        assert_eq!(
            RequestOperationType::from_str("add_user_group").unwrap(),
            RequestOperationType::AddUserGroup
        );
        assert_eq!(
            RequestOperationType::EditUserGroup.to_string(),
            "edit_user_group"
        );
        assert_eq!(
            RequestOperationType::from_str("edit_user_group").unwrap(),
            RequestOperationType::EditUserGroup
        );
        assert_eq!(
            RequestOperationType::RemoveUserGroup.to_string(),
            "remove_user_group"
        );
        assert_eq!(
            RequestOperationType::from_str("remove_user_group").unwrap(),
            RequestOperationType::RemoveUserGroup
        );
        assert_eq!(
            RequestOperationType::from_str("system_upgrade").unwrap(),
            RequestOperationType::SystemUpgrade
        );
        assert_eq!(
            RequestOperationType::from_str("change_external_canister").unwrap(),
            RequestOperationType::ChangeExternalCanister
        );
        assert_eq!(
            RequestOperationType::from_str("create_external_canister").unwrap(),
            RequestOperationType::CreateExternalCanister
        );
        assert_eq!(
            RequestOperationType::from_str("call_external_canister").unwrap(),
            RequestOperationType::CallExternalCanister
        );
        assert_eq!(
            RequestOperationType::from_str("edit_permission").unwrap(),
            RequestOperationType::EditPermission
        );
        assert_eq!(
            RequestOperationType::from_str("add_request_policy").unwrap(),
            RequestOperationType::AddRequestPolicy
        );
        assert_eq!(
            RequestOperationType::from_str("edit_request_policy").unwrap(),
            RequestOperationType::EditRequestPolicy
        );
        assert_eq!(
            RequestOperationType::from_str("remove_request_policy").unwrap(),
            RequestOperationType::RemoveRequestPolicy
        );
        assert_eq!(
            RequestOperationType::from_str("manage_system_info").unwrap(),
            RequestOperationType::ManageSystemInfo
        );
        assert_eq!(
            RequestOperationType::from_str("set_disaster_recovery_committee").unwrap(),
            RequestOperationType::SetDisasterRecovery
        );
        assert_eq!(
            RequestOperationType::from_str("configure_external_canister").unwrap(),
            RequestOperationType::ConfigureExternalCanister
        );
        assert_eq!(
            RequestOperationType::from_str("fund_external_canister").unwrap(),
            RequestOperationType::FundExternalCanister
        );
    }
}
