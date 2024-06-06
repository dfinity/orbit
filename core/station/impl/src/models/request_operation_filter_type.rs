use super::AccountId;
use candid::Principal;
use orbit_essentials::storable;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RequestOperationFilterType {
    Transfer(Option<AccountId>),
    AddAccount,
    EditAccount,
    AddUser,
    EditUser,
    AddUserGroup,
    EditUserGroup,
    RemoveUserGroup,
    ChangeCanister,
    SetDisasterRecovery,
    ChangeManagedCanister(Option<Principal>),
    CreateManagedCanister,
    EditPermission,
    AddRequestPolicy,
    EditRequestPolicy,
    RemoveRequestPolicy,
    AddAddressBookEntry,
    EditAddressBookEntry,
    RemoveAddressBookEntry,
    ManageSystemInfo,
}
