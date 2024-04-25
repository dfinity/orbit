use uuid::Uuid;

use crate::models::resource::{
    AccessPolicyResourceAction, AccountResourceAction, ChangeCanisterResourceAction,
    ProposalResourceAction, Resource, ResourceAction, ResourceId, SystemResourceAction,
    UserResourceAction,
};

use super::HelperMapper;

impl From<wallet_api::ResourceDTO> for Resource {
    fn from(dto: wallet_api::ResourceDTO) -> Self {
        match dto {
            wallet_api::ResourceDTO::User(action) => Resource::User(action.into()),
            wallet_api::ResourceDTO::Account(action) => Resource::Account(action.into()),
            wallet_api::ResourceDTO::AccessPolicy(action) => Resource::AccessPolicy(action.into()),
            wallet_api::ResourceDTO::ProposalPolicy(action) => {
                Resource::ProposalPolicy(action.into())
            }
            wallet_api::ResourceDTO::UserGroup(action) => Resource::UserGroup(action.into()),
            wallet_api::ResourceDTO::AddressBook(action) => Resource::AddressBook(action.into()),
            wallet_api::ResourceDTO::ChangeCanister(action) => {
                Resource::ChangeCanister(action.into())
            }
            wallet_api::ResourceDTO::Proposal(action) => Resource::Proposal(action.into()),
            wallet_api::ResourceDTO::System(action) => Resource::System(action.into()),
        }
    }
}

impl From<Resource> for wallet_api::ResourceDTO {
    fn from(resource: Resource) -> Self {
        match resource {
            Resource::User(action) => wallet_api::ResourceDTO::User(action.into()),
            Resource::Account(action) => wallet_api::ResourceDTO::Account(action.into()),
            Resource::AccessPolicy(action) => wallet_api::ResourceDTO::AccessPolicy(action.into()),
            Resource::ProposalPolicy(action) => {
                wallet_api::ResourceDTO::ProposalPolicy(action.into())
            }
            Resource::UserGroup(action) => wallet_api::ResourceDTO::UserGroup(action.into()),
            Resource::AddressBook(action) => wallet_api::ResourceDTO::AddressBook(action.into()),
            Resource::ChangeCanister(action) => {
                wallet_api::ResourceDTO::ChangeCanister(action.into())
            }
            Resource::Proposal(action) => wallet_api::ResourceDTO::Proposal(action.into()),
            Resource::System(action) => wallet_api::ResourceDTO::System(action.into()),
        }
    }
}

impl From<wallet_api::ResourceIdDTO> for ResourceId {
    fn from(dto: wallet_api::ResourceIdDTO) -> Self {
        match dto {
            wallet_api::ResourceIdDTO::Any => ResourceId::Any,
            wallet_api::ResourceIdDTO::Id(id) => ResourceId::Id(
                *HelperMapper::to_uuid(id)
                    .expect("Invalid resource id")
                    .as_bytes(),
            ),
        }
    }
}

impl From<ResourceId> for wallet_api::ResourceIdDTO {
    fn from(id: ResourceId) -> Self {
        match id {
            ResourceId::Any => wallet_api::ResourceIdDTO::Any,
            ResourceId::Id(id) => {
                wallet_api::ResourceIdDTO::Id(Uuid::from_bytes(id).hyphenated().to_string())
            }
        }
    }
}

impl From<wallet_api::ResourceActionDTO> for ResourceAction {
    fn from(dto: wallet_api::ResourceActionDTO) -> Self {
        match dto {
            wallet_api::ResourceActionDTO::List => ResourceAction::List,
            wallet_api::ResourceActionDTO::Create => ResourceAction::Create,
            wallet_api::ResourceActionDTO::Read(id) => ResourceAction::Read(id.into()),
            wallet_api::ResourceActionDTO::Update(id) => ResourceAction::Update(id.into()),
            wallet_api::ResourceActionDTO::Delete(id) => ResourceAction::Delete(id.into()),
        }
    }
}

impl From<ResourceAction> for wallet_api::ResourceActionDTO {
    fn from(action: ResourceAction) -> Self {
        match action {
            ResourceAction::List => wallet_api::ResourceActionDTO::List,
            ResourceAction::Create => wallet_api::ResourceActionDTO::Create,
            ResourceAction::Read(id) => wallet_api::ResourceActionDTO::Read(id.into()),
            ResourceAction::Update(id) => wallet_api::ResourceActionDTO::Update(id.into()),
            ResourceAction::Delete(id) => wallet_api::ResourceActionDTO::Delete(id.into()),
        }
    }
}

impl From<wallet_api::AccessPolicyResourceActionDTO> for AccessPolicyResourceAction {
    fn from(dto: wallet_api::AccessPolicyResourceActionDTO) -> Self {
        match dto {
            wallet_api::AccessPolicyResourceActionDTO::Read => AccessPolicyResourceAction::Read,
            wallet_api::AccessPolicyResourceActionDTO::Update => AccessPolicyResourceAction::Update,
        }
    }
}

impl From<AccessPolicyResourceAction> for wallet_api::AccessPolicyResourceActionDTO {
    fn from(action: AccessPolicyResourceAction) -> Self {
        match action {
            AccessPolicyResourceAction::Read => wallet_api::AccessPolicyResourceActionDTO::Read,
            AccessPolicyResourceAction::Update => wallet_api::AccessPolicyResourceActionDTO::Update,
        }
    }
}

impl From<wallet_api::UserResourceActionDTO> for UserResourceAction {
    fn from(dto: wallet_api::UserResourceActionDTO) -> Self {
        match dto {
            wallet_api::UserResourceActionDTO::List => UserResourceAction::List,
            wallet_api::UserResourceActionDTO::Create => UserResourceAction::Create,
            wallet_api::UserResourceActionDTO::Read(id) => UserResourceAction::Read(id.into()),
            wallet_api::UserResourceActionDTO::Update(id) => UserResourceAction::Update(id.into()),
        }
    }
}

impl From<UserResourceAction> for wallet_api::UserResourceActionDTO {
    fn from(action: UserResourceAction) -> Self {
        match action {
            UserResourceAction::List => wallet_api::UserResourceActionDTO::List,
            UserResourceAction::Create => wallet_api::UserResourceActionDTO::Create,
            UserResourceAction::Read(id) => wallet_api::UserResourceActionDTO::Read(id.into()),
            UserResourceAction::Update(id) => wallet_api::UserResourceActionDTO::Update(id.into()),
        }
    }
}

impl From<wallet_api::AccountResourceActionDTO> for AccountResourceAction {
    fn from(dto: wallet_api::AccountResourceActionDTO) -> Self {
        match dto {
            wallet_api::AccountResourceActionDTO::List => AccountResourceAction::List,
            wallet_api::AccountResourceActionDTO::Create => AccountResourceAction::Create,
            wallet_api::AccountResourceActionDTO::Transfer(id) => {
                AccountResourceAction::Transfer(id.into())
            }
            wallet_api::AccountResourceActionDTO::Read(id) => {
                AccountResourceAction::Read(id.into())
            }
            wallet_api::AccountResourceActionDTO::Update(id) => {
                AccountResourceAction::Update(id.into())
            }
        }
    }
}

impl From<AccountResourceAction> for wallet_api::AccountResourceActionDTO {
    fn from(action: AccountResourceAction) -> Self {
        match action {
            AccountResourceAction::List => wallet_api::AccountResourceActionDTO::List,
            AccountResourceAction::Create => wallet_api::AccountResourceActionDTO::Create,
            AccountResourceAction::Transfer(id) => {
                wallet_api::AccountResourceActionDTO::Transfer(id.into())
            }
            AccountResourceAction::Read(id) => {
                wallet_api::AccountResourceActionDTO::Read(id.into())
            }
            AccountResourceAction::Update(id) => {
                wallet_api::AccountResourceActionDTO::Update(id.into())
            }
        }
    }
}

impl From<wallet_api::SystemResourceActionDTO> for SystemResourceAction {
    fn from(dto: wallet_api::SystemResourceActionDTO) -> Self {
        match dto {
            wallet_api::SystemResourceActionDTO::SystemInfo => SystemResourceAction::SystemInfo,
            wallet_api::SystemResourceActionDTO::Capabilities => SystemResourceAction::Capabilities,
        }
    }
}

impl From<SystemResourceAction> for wallet_api::SystemResourceActionDTO {
    fn from(action: SystemResourceAction) -> Self {
        match action {
            SystemResourceAction::SystemInfo => wallet_api::SystemResourceActionDTO::SystemInfo,
            SystemResourceAction::Capabilities => wallet_api::SystemResourceActionDTO::Capabilities,
        }
    }
}

impl From<wallet_api::ChangeCanisterResourceActionDTO> for ChangeCanisterResourceAction {
    fn from(_: wallet_api::ChangeCanisterResourceActionDTO) -> Self {
        ChangeCanisterResourceAction::Create
    }
}

impl From<ChangeCanisterResourceAction> for wallet_api::ChangeCanisterResourceActionDTO {
    fn from(_: ChangeCanisterResourceAction) -> Self {
        wallet_api::ChangeCanisterResourceActionDTO::Create
    }
}

impl From<wallet_api::ProposalResourceActionDTO> for ProposalResourceAction {
    fn from(dto: wallet_api::ProposalResourceActionDTO) -> Self {
        match dto {
            wallet_api::ProposalResourceActionDTO::List => ProposalResourceAction::List,
            wallet_api::ProposalResourceActionDTO::Read(id) => {
                ProposalResourceAction::Read(id.into())
            }
        }
    }
}

impl From<ProposalResourceAction> for wallet_api::ProposalResourceActionDTO {
    fn from(action: ProposalResourceAction) -> Self {
        match action {
            ProposalResourceAction::List => wallet_api::ProposalResourceActionDTO::List,
            ProposalResourceAction::Read(id) => {
                wallet_api::ProposalResourceActionDTO::Read(id.into())
            }
        }
    }
}
