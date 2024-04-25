use uuid::Uuid;

use crate::models::resource::{
    AccessPolicyResourceAction, AccountResourceAction, ChangeCanisterResourceAction,
    ProposalResourceAction, Resource, ResourceAction, ResourceId, SystemResourceAction,
    UserResourceAction,
};

use super::HelperMapper;

impl From<station_api::ResourceDTO> for Resource {
    fn from(dto: station_api::ResourceDTO) -> Self {
        match dto {
            station_api::ResourceDTO::User(action) => Resource::User(action.into()),
            station_api::ResourceDTO::Account(action) => Resource::Account(action.into()),
            station_api::ResourceDTO::AccessPolicy(action) => Resource::AccessPolicy(action.into()),
            station_api::ResourceDTO::ProposalPolicy(action) => {
                Resource::ProposalPolicy(action.into())
            }
            station_api::ResourceDTO::UserGroup(action) => Resource::UserGroup(action.into()),
            station_api::ResourceDTO::AddressBook(action) => Resource::AddressBook(action.into()),
            station_api::ResourceDTO::ChangeCanister(action) => {
                Resource::ChangeCanister(action.into())
            }
            station_api::ResourceDTO::Proposal(action) => Resource::Proposal(action.into()),
            station_api::ResourceDTO::System(action) => Resource::System(action.into()),
        }
    }
}

impl From<Resource> for station_api::ResourceDTO {
    fn from(resource: Resource) -> Self {
        match resource {
            Resource::User(action) => station_api::ResourceDTO::User(action.into()),
            Resource::Account(action) => station_api::ResourceDTO::Account(action.into()),
            Resource::AccessPolicy(action) => station_api::ResourceDTO::AccessPolicy(action.into()),
            Resource::ProposalPolicy(action) => {
                station_api::ResourceDTO::ProposalPolicy(action.into())
            }
            Resource::UserGroup(action) => station_api::ResourceDTO::UserGroup(action.into()),
            Resource::AddressBook(action) => station_api::ResourceDTO::AddressBook(action.into()),
            Resource::ChangeCanister(action) => {
                station_api::ResourceDTO::ChangeCanister(action.into())
            }
            Resource::Proposal(action) => station_api::ResourceDTO::Proposal(action.into()),
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

impl From<station_api::AccessPolicyResourceActionDTO> for AccessPolicyResourceAction {
    fn from(dto: station_api::AccessPolicyResourceActionDTO) -> Self {
        match dto {
            station_api::AccessPolicyResourceActionDTO::Read => AccessPolicyResourceAction::Read,
            station_api::AccessPolicyResourceActionDTO::Update => {
                AccessPolicyResourceAction::Update
            }
        }
    }
}

impl From<AccessPolicyResourceAction> for station_api::AccessPolicyResourceActionDTO {
    fn from(action: AccessPolicyResourceAction) -> Self {
        match action {
            AccessPolicyResourceAction::Read => station_api::AccessPolicyResourceActionDTO::Read,
            AccessPolicyResourceAction::Update => {
                station_api::AccessPolicyResourceActionDTO::Update
            }
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

impl From<station_api::ProposalResourceActionDTO> for ProposalResourceAction {
    fn from(dto: station_api::ProposalResourceActionDTO) -> Self {
        match dto {
            station_api::ProposalResourceActionDTO::List => ProposalResourceAction::List,
            station_api::ProposalResourceActionDTO::Read(id) => {
                ProposalResourceAction::Read(id.into())
            }
        }
    }
}

impl From<ProposalResourceAction> for station_api::ProposalResourceActionDTO {
    fn from(action: ProposalResourceAction) -> Self {
        match action {
            ProposalResourceAction::List => station_api::ProposalResourceActionDTO::List,
            ProposalResourceAction::Read(id) => {
                station_api::ProposalResourceActionDTO::Read(id.into())
            }
        }
    }
}
