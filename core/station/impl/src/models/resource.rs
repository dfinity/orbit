use orbit_essentials::storable;
use orbit_essentials::{model::ModelValidator, types::UUID};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

use crate::{
    core::validation::{
        EnsureAccount, EnsureAddressBookEntry, EnsureProposal, EnsureProposalPolicy,
        EnsureResourceIdExists, EnsureUser, EnsureUserGroup,
    },
    errors::RecordValidationError,
};

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Resource {
    Permission(PermissionResourceAction),
    Account(AccountResourceAction),
    AddressBook(ResourceAction),
    ChangeCanister(ChangeCanisterResourceAction),
    Proposal(ProposalResourceAction),
    ProposalPolicy(ResourceAction),
    System(SystemResourceAction),
    User(UserResourceAction),
    UserGroup(ResourceAction),
}

impl ModelValidator<RecordValidationError> for Resource {
    fn validate(&self) -> Result<(), RecordValidationError> {
        match self {
            Resource::Permission(action) => match action {
                PermissionResourceAction::Read | PermissionResourceAction::Update => Ok(()),
            },

            Resource::Account(action) => match action {
                AccountResourceAction::List | AccountResourceAction::Create => Ok(()),
                AccountResourceAction::Transfer(resource_id)
                | AccountResourceAction::Read(resource_id)
                | AccountResourceAction::Update(resource_id) => {
                    EnsureAccount::resource_id_exists(resource_id)
                }
            },
            Resource::AddressBook(action) => match action {
                ResourceAction::List | ResourceAction::Create => Ok(()),
                ResourceAction::Read(resource_id)
                | ResourceAction::Update(resource_id)
                | ResourceAction::Delete(resource_id) => {
                    EnsureAddressBookEntry::resource_id_exists(resource_id)
                }
            },
            Resource::ChangeCanister(action) => match action {
                ChangeCanisterResourceAction::Create => Ok(()),
            },
            Resource::Proposal(action) => match action {
                ProposalResourceAction::List => Ok(()),
                ProposalResourceAction::Read(resource_id) => {
                    EnsureProposal::resource_id_exists(resource_id)
                }
            },
            Resource::ProposalPolicy(action) => match action {
                ResourceAction::List | ResourceAction::Create => Ok(()),
                ResourceAction::Read(resource_id)
                | ResourceAction::Update(resource_id)
                | ResourceAction::Delete(resource_id) => {
                    EnsureProposalPolicy::resource_id_exists(resource_id)
                }
            },
            Resource::System(action) => match action {
                SystemResourceAction::SystemInfo | SystemResourceAction::Capabilities => Ok(()),
            },
            Resource::User(action) => match action {
                UserResourceAction::List | UserResourceAction::Create => Ok(()),
                UserResourceAction::Read(resource_id) | UserResourceAction::Update(resource_id) => {
                    EnsureUser::resource_id_exists(resource_id)
                }
            },
            Resource::UserGroup(action) => match action {
                ResourceAction::List | ResourceAction::Create => Ok(()),
                ResourceAction::Read(resource_id)
                | ResourceAction::Update(resource_id)
                | ResourceAction::Delete(resource_id) => {
                    EnsureUserGroup::resource_id_exists(resource_id)
                }
            },
        }
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceAction {
    List,
    Create,
    Read(ResourceId),
    Update(ResourceId),
    Delete(ResourceId),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PermissionResourceAction {
    Read,
    Update,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UserResourceAction {
    List,
    Create,
    Read(ResourceId),
    Update(ResourceId),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AccountResourceAction {
    List,
    Create,
    Transfer(ResourceId),
    Read(ResourceId),
    Update(ResourceId),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SystemResourceAction {
    SystemInfo,
    Capabilities,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChangeCanisterResourceAction {
    Create,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProposalResourceAction {
    List,
    Read(ResourceId),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceId {
    Any,
    Id(UUID),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceIds {
    Any,
    Ids(Vec<UUID>),
}

impl ResourceIds {
    pub fn to_vec(&self) -> Vec<ResourceId> {
        match self {
            ResourceIds::Any => vec![],
            ResourceIds::Ids(ids) => ids.iter().map(|id| ResourceId::Id(*id)).collect(),
        }
    }
}

impl Resource {
    /// Enables Resource be used in range queries in indexes.
    /// Takes advantage of lexicographical ordering implemented by Ord.
    pub fn min() -> Self {
        Resource::Permission(PermissionResourceAction::Read)
    }
    pub fn max() -> Self {
        Resource::UserGroup(ResourceAction::Delete(ResourceId::Id([u8::MAX; 16])))
    }

    /// Returns the expanded list of resources that the resource represents.
    ///
    /// E.g. if the resource is for account_id = 1, it will also return the resource for account_id = any.
    pub fn to_expanded_list(&self) -> Vec<Resource> {
        match self {
            Resource::Account(action) => match action {
                AccountResourceAction::List => vec![Resource::Account(AccountResourceAction::List)],
                AccountResourceAction::Create => {
                    vec![Resource::Account(AccountResourceAction::Create)]
                }
                AccountResourceAction::Transfer(ResourceId::Id(id)) => {
                    vec![
                        Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(*id))),
                        Resource::Account(AccountResourceAction::Transfer(ResourceId::Any)),
                    ]
                }
                AccountResourceAction::Read(ResourceId::Id(id)) => {
                    vec![
                        Resource::Account(AccountResourceAction::Read(ResourceId::Id(*id))),
                        Resource::Account(AccountResourceAction::Read(ResourceId::Any)),
                    ]
                }
                AccountResourceAction::Update(ResourceId::Id(id)) => {
                    vec![
                        Resource::Account(AccountResourceAction::Update(ResourceId::Id(*id))),
                        Resource::Account(AccountResourceAction::Update(ResourceId::Any)),
                    ]
                }
                AccountResourceAction::Transfer(ResourceId::Any) => {
                    vec![Resource::Account(AccountResourceAction::Transfer(
                        ResourceId::Any,
                    ))]
                }
                AccountResourceAction::Read(ResourceId::Any) => {
                    vec![Resource::Account(AccountResourceAction::Read(
                        ResourceId::Any,
                    ))]
                }
                AccountResourceAction::Update(ResourceId::Any) => {
                    vec![Resource::Account(AccountResourceAction::Update(
                        ResourceId::Any,
                    ))]
                }
            },
            Resource::Permission(action) => match action {
                PermissionResourceAction::Read => {
                    vec![
                        Resource::Permission(PermissionResourceAction::Read),
                        Resource::Permission(PermissionResourceAction::Update),
                    ]
                }
                PermissionResourceAction::Update => {
                    vec![Resource::Permission(PermissionResourceAction::Update)]
                }
            },
            Resource::AddressBook(action) => match action {
                ResourceAction::Create => vec![Resource::AddressBook(ResourceAction::Create)],
                ResourceAction::Delete(ResourceId::Id(id)) => {
                    vec![
                        Resource::AddressBook(ResourceAction::Delete(ResourceId::Id(*id))),
                        Resource::AddressBook(ResourceAction::Delete(ResourceId::Any)),
                    ]
                }
                ResourceAction::List => vec![Resource::AddressBook(ResourceAction::List)],
                ResourceAction::Read(ResourceId::Id(id)) => {
                    vec![
                        Resource::AddressBook(ResourceAction::Read(ResourceId::Id(*id))),
                        Resource::AddressBook(ResourceAction::Read(ResourceId::Any)),
                    ]
                }
                ResourceAction::Update(ResourceId::Id(id)) => {
                    vec![
                        Resource::AddressBook(ResourceAction::Update(ResourceId::Id(*id))),
                        Resource::AddressBook(ResourceAction::Update(ResourceId::Any)),
                    ]
                }
                ResourceAction::Read(ResourceId::Any) => {
                    vec![Resource::AddressBook(ResourceAction::Read(ResourceId::Any))]
                }
                ResourceAction::Update(ResourceId::Any) => {
                    vec![Resource::AddressBook(ResourceAction::Update(
                        ResourceId::Any,
                    ))]
                }
                ResourceAction::Delete(ResourceId::Any) => {
                    vec![Resource::AddressBook(ResourceAction::Delete(
                        ResourceId::Any,
                    ))]
                }
            },
            Resource::ChangeCanister(action) => match action {
                ChangeCanisterResourceAction::Create => {
                    vec![Resource::ChangeCanister(
                        ChangeCanisterResourceAction::Create,
                    )]
                }
            },
            Resource::Proposal(action) => match action {
                ProposalResourceAction::List => {
                    vec![Resource::Proposal(ProposalResourceAction::List)]
                }
                ProposalResourceAction::Read(ResourceId::Id(id)) => {
                    vec![
                        Resource::Proposal(ProposalResourceAction::Read(ResourceId::Id(*id))),
                        Resource::Proposal(ProposalResourceAction::Read(ResourceId::Any)),
                    ]
                }
                ProposalResourceAction::Read(ResourceId::Any) => {
                    vec![Resource::Proposal(ProposalResourceAction::Read(
                        ResourceId::Any,
                    ))]
                }
            },
            Resource::ProposalPolicy(action) => match action {
                ResourceAction::Create => vec![Resource::ProposalPolicy(ResourceAction::Create)],
                ResourceAction::Delete(ResourceId::Id(id)) => {
                    vec![
                        Resource::ProposalPolicy(ResourceAction::Delete(ResourceId::Id(*id))),
                        Resource::ProposalPolicy(ResourceAction::Delete(ResourceId::Any)),
                    ]
                }
                ResourceAction::List => vec![Resource::ProposalPolicy(ResourceAction::List)],
                ResourceAction::Read(ResourceId::Id(id)) => {
                    vec![
                        Resource::ProposalPolicy(ResourceAction::Read(ResourceId::Id(*id))),
                        Resource::ProposalPolicy(ResourceAction::Read(ResourceId::Any)),
                    ]
                }
                ResourceAction::Update(ResourceId::Id(id)) => {
                    vec![
                        Resource::ProposalPolicy(ResourceAction::Update(ResourceId::Id(*id))),
                        Resource::ProposalPolicy(ResourceAction::Update(ResourceId::Any)),
                    ]
                }
                ResourceAction::Update(ResourceId::Any) => {
                    vec![Resource::ProposalPolicy(ResourceAction::Update(
                        ResourceId::Any,
                    ))]
                }
                ResourceAction::Read(ResourceId::Any) => {
                    vec![Resource::ProposalPolicy(ResourceAction::Read(
                        ResourceId::Any,
                    ))]
                }
                ResourceAction::Delete(ResourceId::Any) => {
                    vec![Resource::ProposalPolicy(ResourceAction::Delete(
                        ResourceId::Any,
                    ))]
                }
            },
            Resource::System(action) => match action {
                SystemResourceAction::SystemInfo => {
                    vec![Resource::System(SystemResourceAction::SystemInfo)]
                }
                SystemResourceAction::Capabilities => {
                    vec![Resource::System(SystemResourceAction::Capabilities)]
                }
            },
            Resource::User(action) => match action {
                UserResourceAction::Create => vec![Resource::User(UserResourceAction::Create)],
                UserResourceAction::List => vec![Resource::User(UserResourceAction::List)],
                UserResourceAction::Read(ResourceId::Id(id)) => {
                    vec![
                        Resource::User(UserResourceAction::Read(ResourceId::Id(*id))),
                        Resource::User(UserResourceAction::Read(ResourceId::Any)),
                    ]
                }
                UserResourceAction::Update(ResourceId::Id(id)) => {
                    vec![
                        Resource::User(UserResourceAction::Update(ResourceId::Id(*id))),
                        Resource::User(UserResourceAction::Update(ResourceId::Any)),
                    ]
                }
                UserResourceAction::Read(ResourceId::Any) => {
                    vec![Resource::User(UserResourceAction::Read(ResourceId::Any))]
                }
                UserResourceAction::Update(ResourceId::Any) => {
                    vec![Resource::User(UserResourceAction::Update(ResourceId::Any))]
                }
            },
            Resource::UserGroup(action) => match action {
                ResourceAction::Create => vec![Resource::UserGroup(ResourceAction::Create)],
                ResourceAction::Delete(ResourceId::Id(id)) => {
                    vec![
                        Resource::UserGroup(ResourceAction::Delete(ResourceId::Id(*id))),
                        Resource::UserGroup(ResourceAction::Delete(ResourceId::Any)),
                    ]
                }
                ResourceAction::List => vec![Resource::UserGroup(ResourceAction::List)],
                ResourceAction::Read(ResourceId::Id(id)) => {
                    vec![
                        Resource::UserGroup(ResourceAction::Read(ResourceId::Id(*id))),
                        Resource::UserGroup(ResourceAction::Read(ResourceId::Any)),
                    ]
                }
                ResourceAction::Update(ResourceId::Id(id)) => {
                    vec![
                        Resource::UserGroup(ResourceAction::Update(ResourceId::Id(*id))),
                        Resource::UserGroup(ResourceAction::Update(ResourceId::Any)),
                    ]
                }
                ResourceAction::Update(ResourceId::Any) => {
                    vec![Resource::UserGroup(ResourceAction::Update(ResourceId::Any))]
                }
                ResourceAction::Read(ResourceId::Any) => {
                    vec![Resource::UserGroup(ResourceAction::Read(ResourceId::Any))]
                }
                ResourceAction::Delete(ResourceId::Any) => {
                    vec![Resource::UserGroup(ResourceAction::Delete(ResourceId::Any))]
                }
            },
        }
    }
}

impl Display for Resource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Resource::Permission(action) => write!(f, "Permission({})", action),
            Resource::Account(action) => write!(f, "Account({})", action),
            Resource::AddressBook(action) => write!(f, "AddressBook({})", action),
            Resource::ChangeCanister(action) => write!(f, "ChangeCanister({})", action),
            Resource::Proposal(action) => write!(f, "Proposal({})", action),
            Resource::ProposalPolicy(action) => write!(f, "ProposalPolicy({})", action),
            Resource::System(action) => write!(f, "System({})", action),
            Resource::User(action) => write!(f, "User({})", action),
            Resource::UserGroup(action) => write!(f, "UserGroup({})", action),
        }
    }
}

impl Display for ResourceAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceAction::List => write!(f, "List"),
            ResourceAction::Create => write!(f, "Create"),
            ResourceAction::Read(id) => write!(f, "Read({})", id),
            ResourceAction::Update(id) => write!(f, "Update({})", id),
            ResourceAction::Delete(id) => write!(f, "Delete({})", id),
        }
    }
}

impl Display for PermissionResourceAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PermissionResourceAction::Read => write!(f, "Read"),
            PermissionResourceAction::Update => write!(f, "Update"),
        }
    }
}

impl Display for AccountResourceAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountResourceAction::List => write!(f, "List"),
            AccountResourceAction::Create => write!(f, "Create"),
            AccountResourceAction::Transfer(id) => write!(f, "Transfer({})", id),
            AccountResourceAction::Read(id) => write!(f, "Read({})", id),
            AccountResourceAction::Update(id) => write!(f, "Update({})", id),
        }
    }
}

impl Display for ChangeCanisterResourceAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangeCanisterResourceAction::Create => write!(f, "Create"),
        }
    }
}

impl Display for ProposalResourceAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposalResourceAction::List => write!(f, "List"),
            ProposalResourceAction::Read(id) => write!(f, "Read({})", id),
        }
    }
}

impl Display for SystemResourceAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemResourceAction::SystemInfo => write!(f, "SystemInfo"),
            SystemResourceAction::Capabilities => write!(f, "Capabilities"),
        }
    }
}

impl Display for UserResourceAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UserResourceAction::List => write!(f, "List"),
            UserResourceAction::Create => write!(f, "Create"),
            UserResourceAction::Read(id) => write!(f, "Read({})", id),
            UserResourceAction::Update(id) => write!(f, "Update({})", id),
        }
    }
}

impl Display for ResourceId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceId::Any => write!(f, "Any"),
            ResourceId::Id(id) => {
                write!(f, "Id({})", Uuid::from_bytes(*id).hyphenated())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use orbit_essentials::model::ModelValidator;

    use crate::core::validation::disable_mock_resource_validation;

    use super::{
        AccountResourceAction, ChangeCanisterResourceAction, PermissionResourceAction,
        ProposalResourceAction, Resource, ResourceAction, ResourceId, SystemResourceAction,
        UserResourceAction,
    };

    #[test]
    fn test_resource_validation() {
        disable_mock_resource_validation();

        let valid_resources = vec![
            Resource::Account(AccountResourceAction::List),
            Resource::Account(AccountResourceAction::Create),
            Resource::Account(AccountResourceAction::Transfer(ResourceId::Any)),
            Resource::Account(AccountResourceAction::Read(ResourceId::Any)),
            Resource::Account(AccountResourceAction::Update(ResourceId::Any)),
            Resource::Permission(PermissionResourceAction::Read),
            Resource::Permission(PermissionResourceAction::Update),
            Resource::AddressBook(ResourceAction::List),
            Resource::AddressBook(ResourceAction::Create),
            Resource::AddressBook(ResourceAction::Read(ResourceId::Any)),
            Resource::AddressBook(ResourceAction::Update(ResourceId::Any)),
            Resource::AddressBook(ResourceAction::Delete(ResourceId::Any)),
            Resource::ChangeCanister(ChangeCanisterResourceAction::Create),
            Resource::Proposal(ProposalResourceAction::List),
            Resource::Proposal(ProposalResourceAction::Read(ResourceId::Any)),
            Resource::ProposalPolicy(ResourceAction::List),
            Resource::ProposalPolicy(ResourceAction::Create),
            Resource::ProposalPolicy(ResourceAction::Read(ResourceId::Any)),
            Resource::ProposalPolicy(ResourceAction::Update(ResourceId::Any)),
            Resource::ProposalPolicy(ResourceAction::Delete(ResourceId::Any)),
            Resource::System(SystemResourceAction::SystemInfo),
            Resource::System(SystemResourceAction::Capabilities),
            Resource::User(UserResourceAction::List),
            Resource::User(UserResourceAction::Create),
            Resource::User(UserResourceAction::Read(ResourceId::Any)),
            Resource::User(UserResourceAction::Update(ResourceId::Any)),
            Resource::UserGroup(ResourceAction::List),
            Resource::UserGroup(ResourceAction::Create),
            Resource::UserGroup(ResourceAction::Read(ResourceId::Any)),
            Resource::UserGroup(ResourceAction::Update(ResourceId::Any)),
            Resource::UserGroup(ResourceAction::Delete(ResourceId::Any)),
        ];

        for resource in valid_resources {
            resource
                .validate()
                .expect("ResourceId::Any resource should be valid");
        }
    }

    #[test]
    fn fail_non_existent_resource_id() {
        disable_mock_resource_validation();

        let invalid_resources = vec![
            Resource::Account(AccountResourceAction::Read(ResourceId::Id([0; 16]))),
            Resource::Account(AccountResourceAction::Update(ResourceId::Id([0; 16]))),
            Resource::Account(AccountResourceAction::Transfer(ResourceId::Id([0; 16]))),
            Resource::AddressBook(ResourceAction::Read(ResourceId::Id([0; 16]))),
            Resource::AddressBook(ResourceAction::Update(ResourceId::Id([0; 16]))),
            Resource::AddressBook(ResourceAction::Delete(ResourceId::Id([0; 16]))),
            Resource::Proposal(ProposalResourceAction::Read(ResourceId::Id([0; 16]))),
            Resource::ProposalPolicy(ResourceAction::Read(ResourceId::Id([0; 16]))),
            Resource::ProposalPolicy(ResourceAction::Update(ResourceId::Id([0; 16]))),
            Resource::ProposalPolicy(ResourceAction::Delete(ResourceId::Id([0; 16]))),
            Resource::User(UserResourceAction::Read(ResourceId::Id([0; 16]))),
            Resource::User(UserResourceAction::Update(ResourceId::Id([0; 16]))),
            Resource::UserGroup(ResourceAction::Read(ResourceId::Id([0; 16]))),
            Resource::UserGroup(ResourceAction::Update(ResourceId::Id([0; 16]))),
            Resource::UserGroup(ResourceAction::Delete(ResourceId::Id([0; 16]))),
        ];

        for resource in invalid_resources {
            resource
                .validate()
                .expect_err("Non existent resource should be invalid");
        }
    }
}
