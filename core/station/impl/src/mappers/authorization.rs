use super::HelperMapper;
use crate::{
    core::{ic_cdk::api::trap, CallContext},
    models::{
        resource::{
            AccountResourceAction, CallExternalCanisterResourceTarget,
            ExecutionMethodResourceTarget, ExternalCanisterId, ExternalCanisterResourceAction,
            NotificationResourceAction, PermissionResourceAction, RequestResourceAction, Resource,
            ResourceAction, ResourceId, SystemResourceAction, UserResourceAction,
            ValidationMethodResourceTarget,
        },
        CanisterMethod, Transfer,
    },
    repositories::TRANSFER_REPOSITORY,
};
use orbit_essentials::repository::Repository;
use orbit_essentials::types::UUID;
use station_api::{RequestOperationInput, UserPrivilege};

pub const USER_PRIVILEGES: [UserPrivilege; 21] = [
    UserPrivilege::Capabilities,
    UserPrivilege::SystemInfo,
    UserPrivilege::ManageSystemInfo,
    UserPrivilege::ListUsers,
    UserPrivilege::AddUser,
    UserPrivilege::ListAccounts,
    UserPrivilege::AddAccount,
    UserPrivilege::ListPermissions,
    UserPrivilege::ListRequestPolicies,
    UserPrivilege::AddRequestPolicy,
    UserPrivilege::ListUserGroups,
    UserPrivilege::AddUserGroup,
    UserPrivilege::ListAddressBookEntries,
    UserPrivilege::AddAddressBookEntry,
    UserPrivilege::SystemUpgrade,
    UserPrivilege::ListRequests,
    UserPrivilege::CreateExternalCanister,
    UserPrivilege::ListExternalCanisters,
    UserPrivilege::CallAnyExternalCanister,
    UserPrivilege::AddAsset,
    UserPrivilege::ListAssets,
];

impl From<UserPrivilege> for Resource {
    fn from(privilege: UserPrivilege) -> Self {
        match privilege {
            UserPrivilege::Capabilities => Resource::System(SystemResourceAction::Capabilities),
            UserPrivilege::SystemInfo => Resource::System(SystemResourceAction::SystemInfo),
            UserPrivilege::ListUsers => Resource::User(UserResourceAction::List),
            UserPrivilege::AddUser => Resource::User(UserResourceAction::Create),
            UserPrivilege::ListAccounts => Resource::Account(AccountResourceAction::List),
            UserPrivilege::AddAccount => Resource::Account(AccountResourceAction::Create),
            UserPrivilege::ListPermissions => Resource::Permission(PermissionResourceAction::Read),
            UserPrivilege::ListRequestPolicies => Resource::RequestPolicy(ResourceAction::List),
            UserPrivilege::AddRequestPolicy => Resource::RequestPolicy(ResourceAction::Create),
            UserPrivilege::ListUserGroups => Resource::UserGroup(ResourceAction::List),
            UserPrivilege::AddUserGroup => Resource::UserGroup(ResourceAction::Create),
            UserPrivilege::ListAddressBookEntries => Resource::AddressBook(ResourceAction::List),
            UserPrivilege::AddAddressBookEntry => Resource::AddressBook(ResourceAction::Create),
            UserPrivilege::SystemUpgrade => Resource::System(SystemResourceAction::Upgrade),
            UserPrivilege::ListRequests => Resource::Request(RequestResourceAction::List),
            UserPrivilege::ManageSystemInfo => {
                Resource::System(SystemResourceAction::ManageSystemInfo)
            }
            UserPrivilege::CreateExternalCanister => {
                Resource::ExternalCanister(ExternalCanisterResourceAction::Create)
            }
            UserPrivilege::ListExternalCanisters => {
                Resource::ExternalCanister(ExternalCanisterResourceAction::List)
            }
            UserPrivilege::CallAnyExternalCanister => Resource::ExternalCanister(
                ExternalCanisterResourceAction::Call(CallExternalCanisterResourceTarget {
                    execution_method: ExecutionMethodResourceTarget::Any,
                    validation_method: ValidationMethodResourceTarget::No,
                }),
            ),
            UserPrivilege::AddAsset => Resource::Asset(ResourceAction::Create),
            UserPrivilege::ListAssets => Resource::Asset(ResourceAction::List),
        }
    }
}

impl From<&CallContext> for Resource {
    fn from(input: &CallContext) -> Self {
        Resource::User(UserResourceAction::Read(ResourceId::Id(
            input.user().expect("Caller does not exist as a user").id,
        )))
    }
}

impl From<&station_api::GetAccountInput> for Resource {
    fn from(input: &station_api::GetAccountInput) -> Self {
        Resource::Account(AccountResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.account_id.to_owned())
                .expect("Invalid account id")
                .as_bytes(),
        )))
    }
}

impl From<&station_api::ListAccountTransfersInput> for Resource {
    fn from(input: &station_api::ListAccountTransfersInput) -> Self {
        Resource::Account(AccountResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.account_id.to_owned())
                .expect("Invalid account id")
                .as_bytes(),
        )))
    }
}

impl From<&station_api::GetUserInput> for Resource {
    fn from(input: &station_api::GetUserInput) -> Self {
        Resource::User(UserResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.user_id.to_owned())
                .expect("Invalid user id")
                .as_bytes(),
        )))
    }
}

impl From<&station_api::GetRequestInput> for Resource {
    fn from(input: &station_api::GetRequestInput) -> Self {
        Resource::Request(RequestResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.request_id.to_owned())
                .expect("Invalid request id")
                .as_bytes(),
        )))
    }
}

impl From<&station_api::GetRequestPolicyInput> for Resource {
    fn from(input: &station_api::GetRequestPolicyInput) -> Self {
        Resource::RequestPolicy(ResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.id.to_owned())
                .expect("Invalid request policy id")
                .as_bytes(),
        )))
    }
}

impl From<&station_api::GetUserGroupInput> for Resource {
    fn from(input: &station_api::GetUserGroupInput) -> Self {
        Resource::UserGroup(ResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.user_group_id.to_owned())
                .expect("Invalid user group id")
                .as_bytes(),
        )))
    }
}

impl From<&station_api::GetAssetInput> for Resource {
    fn from(input: &station_api::GetAssetInput) -> Self {
        Resource::Asset(ResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.asset_id.to_owned())
                .expect("Invalid asset id")
                .as_bytes(),
        )))
    }
}

impl From<&station_api::SubmitRequestApprovalInput> for Resource {
    fn from(input: &station_api::SubmitRequestApprovalInput) -> Self {
        Resource::Request(RequestResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.request_id.to_owned())
                .expect("Invalid request id")
                .as_bytes(),
        )))
    }
}

impl From<&station_api::GetAddressBookEntryInputDTO> for Resource {
    fn from(input: &station_api::GetAddressBookEntryInputDTO) -> Self {
        Resource::AddressBook(ResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.address_book_entry_id.to_owned())
                .expect("Invalid address book entry id")
                .as_bytes(),
        )))
    }
}

impl From<&station_api::ListNotificationsInput> for Resource {
    fn from(_input: &station_api::ListNotificationsInput) -> Self {
        Resource::Notification(NotificationResourceAction::List)
    }
}

impl From<&station_api::CancelRequestInput> for Resource {
    fn from(input: &station_api::CancelRequestInput) -> Self {
        Resource::Request(RequestResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.request_id.to_owned())
                .expect("Invalid request id")
                .as_bytes(),
        )))
    }
}

impl From<&station_api::CreateRequestInput> for Resource {
    fn from(input: &station_api::CreateRequestInput) -> Self {
        match &input.operation {
            RequestOperationInput::AddAccount(_) => {
                Resource::Account(AccountResourceAction::Create)
            }
            RequestOperationInput::EditAccount(input) => {
                Resource::Account(AccountResourceAction::Update(ResourceId::Id(
                    *HelperMapper::to_uuid(input.account_id.to_owned())
                        .expect("Invalid account id")
                        .as_bytes(),
                )))
            }
            RequestOperationInput::AddAddressBookEntry(_) => {
                Resource::AddressBook(ResourceAction::Create)
            }
            RequestOperationInput::EditAddressBookEntry(input) => {
                Resource::AddressBook(ResourceAction::Update(ResourceId::Id(
                    *HelperMapper::to_uuid(input.address_book_entry_id.to_owned())
                        .expect("Invalid address book entry id")
                        .as_bytes(),
                )))
            }
            RequestOperationInput::RemoveAddressBookEntry(input) => {
                Resource::AddressBook(ResourceAction::Delete(ResourceId::Id(
                    *HelperMapper::to_uuid(input.address_book_entry_id.to_owned())
                        .expect("Invalid address book entry id")
                        .as_bytes(),
                )))
            }
            RequestOperationInput::Transfer(input) => {
                Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(
                    *HelperMapper::to_uuid(input.from_account_id.to_owned())
                        .expect("Invalid account id")
                        .as_bytes(),
                )))
            }
            RequestOperationInput::AddUser(_) => Resource::User(UserResourceAction::Create),
            RequestOperationInput::EditUser(input) => {
                Resource::User(UserResourceAction::Update(ResourceId::Id(
                    *HelperMapper::to_uuid(input.id.to_owned())
                        .expect("Invalid user id")
                        .as_bytes(),
                )))
            }
            RequestOperationInput::AddUserGroup(_) => Resource::UserGroup(ResourceAction::Create),
            RequestOperationInput::EditUserGroup(input) => {
                Resource::UserGroup(ResourceAction::Update(ResourceId::Id(
                    *HelperMapper::to_uuid(input.user_group_id.to_owned())
                        .expect("Invalid user group id")
                        .as_bytes(),
                )))
            }
            RequestOperationInput::RemoveUserGroup(input) => {
                Resource::UserGroup(ResourceAction::Delete(ResourceId::Id(
                    *HelperMapper::to_uuid(input.user_group_id.to_owned())
                        .expect("Invalid user group id")
                        .as_bytes(),
                )))
            }
            RequestOperationInput::SetDisasterRecovery(_)
            | RequestOperationInput::SystemUpgrade(_) => {
                Resource::System(SystemResourceAction::Upgrade)
            }
            RequestOperationInput::ChangeExternalCanister(input) => {
                Resource::ExternalCanister(ExternalCanisterResourceAction::Change(
                    ExternalCanisterId::Canister(input.canister_id),
                ))
            }
            // Configuration of external canisters share the same `Change` action privilege
            RequestOperationInput::ConfigureExternalCanister(input) => {
                Resource::ExternalCanister(ExternalCanisterResourceAction::Change(
                    ExternalCanisterId::Canister(input.canister_id),
                ))
            }
            RequestOperationInput::FundExternalCanister(input) => {
                Resource::ExternalCanister(ExternalCanisterResourceAction::Fund(
                    ExternalCanisterId::Canister(input.canister_id),
                ))
            }
            RequestOperationInput::CreateExternalCanister(_) => {
                Resource::ExternalCanister(ExternalCanisterResourceAction::Create)
            }
            RequestOperationInput::CallExternalCanister(input) => {
                let validation_method: Option<CanisterMethod> =
                    input.validation_method.clone().map(|m| m.into());
                let execution_method: CanisterMethod = input.execution_method.clone().into();
                Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                    CallExternalCanisterResourceTarget {
                        validation_method: validation_method.into(),
                        execution_method: execution_method.into(),
                    },
                ))
            }
            RequestOperationInput::SnapshotExternalCanister(input) => {
                Resource::ExternalCanister(ExternalCanisterResourceAction::Change(
                    ExternalCanisterId::Canister(input.canister_id),
                ))
            }
            RequestOperationInput::RestoreExternalCanister(input) => {
                Resource::ExternalCanister(ExternalCanisterResourceAction::Change(
                    ExternalCanisterId::Canister(input.canister_id),
                ))
            }
            RequestOperationInput::PruneExternalCanister(input) => {
                Resource::ExternalCanister(ExternalCanisterResourceAction::Change(
                    ExternalCanisterId::Canister(input.canister_id),
                ))
            }
            RequestOperationInput::EditPermission(_) => {
                Resource::Permission(PermissionResourceAction::Update)
            }
            RequestOperationInput::AddRequestPolicy(_) => {
                Resource::RequestPolicy(ResourceAction::Create)
            }
            RequestOperationInput::EditRequestPolicy(input) => {
                Resource::RequestPolicy(ResourceAction::Update(ResourceId::Id(
                    *HelperMapper::to_uuid(input.policy_id.to_owned())
                        .expect("Invalid request policy id")
                        .as_bytes(),
                )))
            }
            RequestOperationInput::RemoveRequestPolicy(input) => {
                Resource::RequestPolicy(ResourceAction::Delete(ResourceId::Id(
                    *HelperMapper::to_uuid(input.policy_id.to_owned())
                        .expect("Invalid request policy id")
                        .as_bytes(),
                )))
            }
            RequestOperationInput::ManageSystemInfo(_) => {
                Resource::System(SystemResourceAction::ManageSystemInfo)
            }
            RequestOperationInput::AddAsset(_) => Resource::Asset(ResourceAction::Create),
            RequestOperationInput::EditAsset(input) => {
                Resource::Asset(ResourceAction::Update(ResourceId::Id(
                    *HelperMapper::to_uuid(input.asset_id.to_owned())
                        .expect("Invalid asset id")
                        .as_bytes(),
                )))
            }
            RequestOperationInput::RemoveAsset(input) => {
                Resource::Asset(ResourceAction::Delete(ResourceId::Id(
                    *HelperMapper::to_uuid(input.asset_id.to_owned())
                        .expect("Invalid asset id")
                        .as_bytes(),
                )))
            }
        }
    }
}

pub(crate) struct FetchAccountBalancesInputRef<'a>(pub &'a station_api::FetchAccountBalancesInput);

impl FetchAccountBalancesInputRef<'_> {
    pub fn to_resources(&self) -> Vec<Resource> {
        let account_ids = self
            .0
            .account_ids
            .iter()
            .map(|account_id| {
                let account_id = *HelperMapper::to_uuid(account_id.to_owned())
                    .expect("Invalid account id")
                    .as_bytes();

                account_id
            })
            .collect::<Vec<UUID>>();

        account_ids
            .iter()
            .map(|account_id| {
                Resource::Account(AccountResourceAction::Read(ResourceId::Id(*account_id)))
            })
            .collect()
    }
}

pub(crate) struct GetTransfersInputRef<'a>(pub &'a station_api::GetTransfersInput);

impl GetTransfersInputRef<'_> {
    pub fn to_resources(&self) -> Vec<Resource> {
        let transfer_ids = self
            .0
            .transfer_ids
            .iter()
            .map(|transfer_id| {
                let transfer_id = *HelperMapper::to_uuid(transfer_id.to_owned())
                    .expect("Invalid transfer id")
                    .as_bytes();

                transfer_id
            })
            .collect::<Vec<UUID>>();

        let transfers = transfer_ids
            .iter()
            .map(|transfer_id| {
                TRANSFER_REPOSITORY
                    .get(&Transfer::key(*transfer_id))
                    .unwrap_or_else(|| trap("Failed to unwrap transfers input"))
            })
            .collect::<Vec<Transfer>>();

        let account_ids = transfers
            .iter()
            .map(|transfer| transfer.from_account)
            .collect::<Vec<_>>();

        account_ids
            .iter()
            .map(|account_id| {
                Resource::Account(AccountResourceAction::Read(ResourceId::Id(*account_id)))
            })
            .collect()
    }
}

pub(crate) struct MarkNotificationsReadInputRef<'a>(
    pub &'a station_api::MarkNotificationsReadInput,
);

impl MarkNotificationsReadInputRef<'_> {
    pub fn to_resources(&self) -> Vec<Resource> {
        let notification_ids = self
            .0
            .notification_ids
            .iter()
            .map(|notification_id| {
                let notification_id = *HelperMapper::to_uuid(notification_id.to_owned())
                    .expect("Invalid notification id")
                    .as_bytes();

                notification_id
            })
            .collect::<Vec<UUID>>();

        notification_ids
            .iter()
            .map(|notification_id| {
                Resource::Notification(NotificationResourceAction::Update(ResourceId::Id(
                    *notification_id,
                )))
            })
            .collect()
    }
}
