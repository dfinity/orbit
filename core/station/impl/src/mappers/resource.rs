use crate::models::resource::{
    AccountResourceAction, ChangeCanisterResourceAction, ChangeManagedCanisterResourceAction,
    ChangeManagedCanisterResourceTarget, PermissionResourceAction, RequestResourceAction, Resource,
    ResourceAction, ResourceId, SystemResourceAction, UserResourceAction,
};
use uuid::Uuid;

use super::HelperMapper;

impl From<station_api::ResourceDTO> for Resource {
    fn from(dto: station_api::ResourceDTO) -> Self {
        match dto {
            station_api::ResourceDTO::User(action) => Resource::User(action.into()),
            station_api::ResourceDTO::Account(action) => Resource::Account(action.into()),
            station_api::ResourceDTO::Permission(action) => Resource::Permission(action.into()),
            station_api::ResourceDTO::RequestPolicy(action) => {
                Resource::RequestPolicy(action.into())
            }
            station_api::ResourceDTO::UserGroup(action) => Resource::UserGroup(action.into()),
            station_api::ResourceDTO::AddressBook(action) => Resource::AddressBook(action.into()),
            station_api::ResourceDTO::ChangeCanister(action) => {
                Resource::ChangeCanister(action.into())
            }
            station_api::ResourceDTO::ChangeManagedCanister(action) => {
                Resource::ChangeManagedCanister(action.into())
            }
            station_api::ResourceDTO::Request(action) => Resource::Request(action.into()),
            station_api::ResourceDTO::System(action) => Resource::System(action.into()),
        }
    }
}

impl From<Resource> for station_api::ResourceDTO {
    fn from(resource: Resource) -> Self {
        match resource {
            Resource::User(action) => station_api::ResourceDTO::User(action.into()),
            Resource::Account(action) => station_api::ResourceDTO::Account(action.into()),
            Resource::Permission(action) => station_api::ResourceDTO::Permission(action.into()),
            Resource::RequestPolicy(action) => {
                station_api::ResourceDTO::RequestPolicy(action.into())
            }
            Resource::UserGroup(action) => station_api::ResourceDTO::UserGroup(action.into()),
            Resource::AddressBook(action) => station_api::ResourceDTO::AddressBook(action.into()),
            Resource::ChangeCanister(action) => {
                station_api::ResourceDTO::ChangeCanister(action.into())
            }
            Resource::ChangeManagedCanister(action) => {
                station_api::ResourceDTO::ChangeManagedCanister(action.into())
            }
            Resource::Request(action) => station_api::ResourceDTO::Request(action.into()),
            Resource::System(action) => station_api::ResourceDTO::System(action.into()),
        }
    }
}

impl From<station_api::ResourceIdDTO> for ResourceId {
    fn from(dto: station_api::ResourceIdDTO) -> Self {
        match dto {
            station_api::ResourceIdDTO::Any => ResourceId::Any,
            station_api::ResourceIdDTO::Id(id) => ResourceId::Id(
                *HelperMapper::to_uuid(id)
                    .expect("Invalid resource id")
                    .as_bytes(),
            ),
        }
    }
}

impl From<ResourceId> for station_api::ResourceIdDTO {
    fn from(id: ResourceId) -> Self {
        match id {
            ResourceId::Any => station_api::ResourceIdDTO::Any,
            ResourceId::Id(id) => {
                station_api::ResourceIdDTO::Id(Uuid::from_bytes(id).hyphenated().to_string())
            }
        }
    }
}

impl From<station_api::ResourceActionDTO> for ResourceAction {
    fn from(dto: station_api::ResourceActionDTO) -> Self {
        match dto {
            station_api::ResourceActionDTO::List => ResourceAction::List,
            station_api::ResourceActionDTO::Create => ResourceAction::Create,
            station_api::ResourceActionDTO::Read(id) => ResourceAction::Read(id.into()),
            station_api::ResourceActionDTO::Update(id) => ResourceAction::Update(id.into()),
            station_api::ResourceActionDTO::Delete(id) => ResourceAction::Delete(id.into()),
        }
    }
}

impl From<ResourceAction> for station_api::ResourceActionDTO {
    fn from(action: ResourceAction) -> Self {
        match action {
            ResourceAction::List => station_api::ResourceActionDTO::List,
            ResourceAction::Create => station_api::ResourceActionDTO::Create,
            ResourceAction::Read(id) => station_api::ResourceActionDTO::Read(id.into()),
            ResourceAction::Update(id) => station_api::ResourceActionDTO::Update(id.into()),
            ResourceAction::Delete(id) => station_api::ResourceActionDTO::Delete(id.into()),
        }
    }
}

impl From<station_api::PermissionResourceActionDTO> for PermissionResourceAction {
    fn from(dto: station_api::PermissionResourceActionDTO) -> Self {
        match dto {
            station_api::PermissionResourceActionDTO::Read => PermissionResourceAction::Read,
            station_api::PermissionResourceActionDTO::Update => PermissionResourceAction::Update,
        }
    }
}

impl From<PermissionResourceAction> for station_api::PermissionResourceActionDTO {
    fn from(action: PermissionResourceAction) -> Self {
        match action {
            PermissionResourceAction::Read => station_api::PermissionResourceActionDTO::Read,
            PermissionResourceAction::Update => station_api::PermissionResourceActionDTO::Update,
        }
    }
}

impl From<station_api::UserResourceActionDTO> for UserResourceAction {
    fn from(dto: station_api::UserResourceActionDTO) -> Self {
        match dto {
            station_api::UserResourceActionDTO::List => UserResourceAction::List,
            station_api::UserResourceActionDTO::Create => UserResourceAction::Create,
            station_api::UserResourceActionDTO::Read(id) => UserResourceAction::Read(id.into()),
            station_api::UserResourceActionDTO::Update(id) => UserResourceAction::Update(id.into()),
        }
    }
}

impl From<UserResourceAction> for station_api::UserResourceActionDTO {
    fn from(action: UserResourceAction) -> Self {
        match action {
            UserResourceAction::List => station_api::UserResourceActionDTO::List,
            UserResourceAction::Create => station_api::UserResourceActionDTO::Create,
            UserResourceAction::Read(id) => station_api::UserResourceActionDTO::Read(id.into()),
            UserResourceAction::Update(id) => station_api::UserResourceActionDTO::Update(id.into()),
        }
    }
}

impl From<station_api::AccountResourceActionDTO> for AccountResourceAction {
    fn from(dto: station_api::AccountResourceActionDTO) -> Self {
        match dto {
            station_api::AccountResourceActionDTO::List => AccountResourceAction::List,
            station_api::AccountResourceActionDTO::Create => AccountResourceAction::Create,
            station_api::AccountResourceActionDTO::Transfer(id) => {
                AccountResourceAction::Transfer(id.into())
            }
            station_api::AccountResourceActionDTO::Read(id) => {
                AccountResourceAction::Read(id.into())
            }
            station_api::AccountResourceActionDTO::Update(id) => {
                AccountResourceAction::Update(id.into())
            }
        }
    }
}

impl From<AccountResourceAction> for station_api::AccountResourceActionDTO {
    fn from(action: AccountResourceAction) -> Self {
        match action {
            AccountResourceAction::List => station_api::AccountResourceActionDTO::List,
            AccountResourceAction::Create => station_api::AccountResourceActionDTO::Create,
            AccountResourceAction::Transfer(id) => {
                station_api::AccountResourceActionDTO::Transfer(id.into())
            }
            AccountResourceAction::Read(id) => {
                station_api::AccountResourceActionDTO::Read(id.into())
            }
            AccountResourceAction::Update(id) => {
                station_api::AccountResourceActionDTO::Update(id.into())
            }
        }
    }
}

impl From<station_api::SystemResourceActionDTO> for SystemResourceAction {
    fn from(dto: station_api::SystemResourceActionDTO) -> Self {
        match dto {
            station_api::SystemResourceActionDTO::SystemInfo => SystemResourceAction::SystemInfo,
            station_api::SystemResourceActionDTO::Capabilities => {
                SystemResourceAction::Capabilities
            }
            station_api::SystemResourceActionDTO::ManageSystemInfo => {
                SystemResourceAction::ManageSystemInfo
            }
        }
    }
}

impl From<SystemResourceAction> for station_api::SystemResourceActionDTO {
    fn from(action: SystemResourceAction) -> Self {
        match action {
            SystemResourceAction::SystemInfo => station_api::SystemResourceActionDTO::SystemInfo,
            SystemResourceAction::Capabilities => {
                station_api::SystemResourceActionDTO::Capabilities
            }
            SystemResourceAction::ManageSystemInfo => {
                station_api::SystemResourceActionDTO::ManageSystemInfo
            }
        }
    }
}

impl From<station_api::ChangeCanisterResourceActionDTO> for ChangeCanisterResourceAction {
    fn from(_: station_api::ChangeCanisterResourceActionDTO) -> Self {
        ChangeCanisterResourceAction::Create
    }
}

impl From<ChangeCanisterResourceAction> for station_api::ChangeCanisterResourceActionDTO {
    fn from(_: ChangeCanisterResourceAction) -> Self {
        station_api::ChangeCanisterResourceActionDTO::Create
    }
}

impl From<station_api::ChangeManagedCanisterResourceTargetDTO>
    for ChangeManagedCanisterResourceTarget
{
    fn from(action: station_api::ChangeManagedCanisterResourceTargetDTO) -> Self {
        match action {
            station_api::ChangeManagedCanisterResourceTargetDTO::Any => {
                ChangeManagedCanisterResourceTarget::Any
            }
            station_api::ChangeManagedCanisterResourceTargetDTO::Canister(canister_id) => {
                ChangeManagedCanisterResourceTarget::Canister(canister_id)
            }
        }
    }
}

impl From<ChangeManagedCanisterResourceTarget>
    for station_api::ChangeManagedCanisterResourceTargetDTO
{
    fn from(action: ChangeManagedCanisterResourceTarget) -> Self {
        match action {
            ChangeManagedCanisterResourceTarget::Any => {
                station_api::ChangeManagedCanisterResourceTargetDTO::Any
            }
            ChangeManagedCanisterResourceTarget::Canister(canister_id) => {
                station_api::ChangeManagedCanisterResourceTargetDTO::Canister(canister_id)
            }
        }
    }
}

impl From<station_api::ChangeManagedCanisterResourceActionDTO>
    for ChangeManagedCanisterResourceAction
{
    fn from(action: station_api::ChangeManagedCanisterResourceActionDTO) -> Self {
        match action {
            station_api::ChangeManagedCanisterResourceActionDTO::Create(target) => {
                ChangeManagedCanisterResourceAction::Create(target.into())
            }
        }
    }
}

impl From<ChangeManagedCanisterResourceAction>
    for station_api::ChangeManagedCanisterResourceActionDTO
{
    fn from(action: ChangeManagedCanisterResourceAction) -> Self {
        match action {
            ChangeManagedCanisterResourceAction::Create(target) => {
                station_api::ChangeManagedCanisterResourceActionDTO::Create(target.into())
            }
        }
    }
}

impl From<station_api::RequestResourceActionDTO> for RequestResourceAction {
    fn from(dto: station_api::RequestResourceActionDTO) -> Self {
        match dto {
            station_api::RequestResourceActionDTO::List => RequestResourceAction::List,
            station_api::RequestResourceActionDTO::Read(id) => {
                RequestResourceAction::Read(id.into())
            }
        }
    }
}

impl From<RequestResourceAction> for station_api::RequestResourceActionDTO {
    fn from(action: RequestResourceAction) -> Self {
        match action {
            RequestResourceAction::List => station_api::RequestResourceActionDTO::List,
            RequestResourceAction::Read(id) => {
                station_api::RequestResourceActionDTO::Read(id.into())
            }
        }
    }
}
