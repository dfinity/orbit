use super::{User, UserGroupId, UserId};
use ic_canister_core::{model::ModelKey, types::UUID};
use ic_canister_macros::storable;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AuthScope {
    Public = 1,
    Authenticated = 2,
    Restricted = 3,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Allow {
    pub auth_scope: AuthScope,
    pub users: Vec<UserId>,
    pub user_groups: Vec<UserGroupId>,
}

impl Default for AuthScope {
    fn default() -> Self {
        Self::Restricted
    }
}

/// Represents an access policy within the system.
///
/// An access policy is a rule that defines who can access a resource and what they can do with it.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd)]
pub struct AccessPolicy {
    /// The resource that the user can access.
    pub resource: Resource,
    /// The users who can access the resource.
    ///
    /// It can be a list of specific users, user groups, or any user.
    pub allow: Allow,
}

impl AccessPolicy {
    pub fn allowed_public(&self) -> bool {
        AuthScope::Public == self.allow.auth_scope
    }

    pub fn allowed_authenticated(&self) -> bool {
        AuthScope::Authenticated == self.allow.auth_scope
    }

    pub fn allowed_users(&self) -> Vec<UserId> {
        self.allow.users.clone()
    }

    pub fn allowed_user_groups(&self) -> Vec<UserGroupId> {
        self.allow.user_groups.clone()
    }

    /// Checks if the user is allowed to access the resource according to the policy.
    pub fn is_allowed(&self, user: &User) -> bool {
        if self.allowed_public() {
            return true;
        }

        if !user.is_active() {
            return false;
        }

        if self.allowed_authenticated() {
            return true;
        }

        if self.allowed_users().contains(&user.id) {
            return true;
        }

        self.allowed_user_groups()
            .iter()
            .any(|group| user.groups.contains(group))
    }
}

impl Allow {
    pub fn public() -> Self {
        Self {
            auth_scope: AuthScope::Public,
            ..Default::default()
        }
    }

    pub fn authenticated() -> Self {
        Self {
            auth_scope: AuthScope::Authenticated,
            ..Default::default()
        }
    }

    pub fn restricted() -> Self {
        Self {
            auth_scope: AuthScope::Restricted,
            ..Default::default()
        }
    }

    pub fn users(users: Vec<UserId>) -> Self {
        Self {
            auth_scope: AuthScope::Restricted,
            users,
            ..Default::default()
        }
    }

    pub fn user_groups(user_groups: Vec<UserGroupId>) -> Self {
        Self {
            auth_scope: AuthScope::Restricted,
            user_groups,
            ..Default::default()
        }
    }
}

/// The unique identifier of an access policy.
pub type AccessPolicyKey = Resource;

impl ModelKey<AccessPolicyKey> for AccessPolicy {
    fn key(&self) -> Resource {
        self.resource.clone()
    }
}

impl AccessPolicy {
    /// Creates a new access policy with the given allow and resource.
    ///
    /// The id is generated automatically.
    pub fn new(allow: Allow, resource: Resource) -> Self {
        Self { allow, resource }
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Resource {
    AccessPolicy(AccessPolicyResourceAction),
    Account(AccountResourceAction),
    AddressBook(ResourceAction),
    ChangeCanister(ChangeCanisterResourceAction),
    Proposal(ProposalResourceAction),
    ProposalPolicy(ResourceAction),
    Settings(SettingsResourceAction),
    User(UserResourceAction),
    UserGroup(ResourceAction),
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
pub enum AccessPolicyResourceAction {
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
pub enum SettingsResourceAction {
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

impl Resource {
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
            Resource::AccessPolicy(action) => match action {
                AccessPolicyResourceAction::Read => {
                    vec![
                        Resource::AccessPolicy(AccessPolicyResourceAction::Read),
                        Resource::AccessPolicy(AccessPolicyResourceAction::Update),
                    ]
                }
                AccessPolicyResourceAction::Update => {
                    vec![Resource::AccessPolicy(AccessPolicyResourceAction::Update)]
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
            Resource::Settings(action) => match action {
                SettingsResourceAction::SystemInfo => {
                    vec![Resource::Settings(SettingsResourceAction::SystemInfo)]
                }
                SettingsResourceAction::Capabilities => {
                    vec![Resource::Settings(SettingsResourceAction::Capabilities)]
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
            Resource::AccessPolicy(action) => write!(f, "AccessPolicy({})", action),
            Resource::Account(action) => write!(f, "Account({})", action),
            Resource::AddressBook(action) => write!(f, "AddressBook({})", action),
            Resource::ChangeCanister(action) => write!(f, "ChangeCanister({})", action),
            Resource::Proposal(action) => write!(f, "Proposal({})", action),
            Resource::ProposalPolicy(action) => write!(f, "ProposalPolicy({})", action),
            Resource::Settings(action) => write!(f, "Settings({})", action),
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

impl Display for AccessPolicyResourceAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AccessPolicyResourceAction::Read => write!(f, "Read"),
            AccessPolicyResourceAction::Update => write!(f, "Update"),
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

impl Display for SettingsResourceAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsResourceAction::SystemInfo => write!(f, "SystemInfo"),
            SettingsResourceAction::Capabilities => write!(f, "Capabilities"),
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

#[cfg(any(test, feature = "canbench"))]
pub mod access_policy_test_utils {
    use super::*;
    use std::cell::RefCell;

    thread_local! {
        static RANDOM_MOCKED_POLICY: RefCell<u8> = RefCell::new(0);
        static AVAILABLE_POLICIES: RefCell<Vec<AccessPolicy>> = RefCell::new(vec![
            AccessPolicy::new(
                Allow::public(),
                Resource::Account(AccountResourceAction::Create),
            ),
            AccessPolicy::new(
                Allow::authenticated(),
                Resource::Account(AccountResourceAction::List),
            ),
            AccessPolicy::new(
                Allow::authenticated(),
                Resource::Account(AccountResourceAction::Read(ResourceId::Any)),
            ),
            AccessPolicy::new(
                Allow::authenticated(),
                Resource::Account(AccountResourceAction::Transfer(ResourceId::Any)),
            ),
            AccessPolicy::new(
                Allow::authenticated(),
                Resource::Account(AccountResourceAction::Update(ResourceId::Any)),
            ),
            AccessPolicy::new(
                Allow::authenticated(),
                Resource::AccessPolicy(AccessPolicyResourceAction::Read),
            ),
            AccessPolicy::new(
                Allow::authenticated(),
                Resource::AccessPolicy(AccessPolicyResourceAction::Update),
            ),
            AccessPolicy::new(
                Allow::authenticated(),
                Resource::AddressBook(ResourceAction::Create),
            ),
            AccessPolicy::new(
                Allow::authenticated(),
                Resource::AddressBook(ResourceAction::Delete(ResourceId::Any)),
            ),
            AccessPolicy::new(
                Allow::authenticated(),
                Resource::AddressBook(ResourceAction::List),
            ),
            AccessPolicy::new(
                Allow::authenticated(),
                Resource::AddressBook(ResourceAction::Read(ResourceId::Any)),
            ),
            AccessPolicy::new(
                Allow::authenticated(),
                Resource::AddressBook(ResourceAction::Update(ResourceId::Any)),
            ),
            AccessPolicy::new(
                Allow::authenticated(),
                Resource::User(UserResourceAction::Create),
            ),
            AccessPolicy::new(
                Allow::authenticated(),
                Resource::User(UserResourceAction::List),
            ),
            AccessPolicy::new(
                Allow::authenticated(),
                Resource::User(UserResourceAction::Read(ResourceId::Any)),
            ),
            AccessPolicy::new(
                Allow::authenticated(),
                Resource::User(UserResourceAction::Update(ResourceId::Any)),
            ),
            AccessPolicy::new(
                Allow::authenticated(),
                Resource::UserGroup(ResourceAction::Create),
            ),
            AccessPolicy::new(
                Allow::authenticated(),
                Resource::UserGroup(ResourceAction::Delete(ResourceId::Any)),
            ),
            AccessPolicy::new(
                Allow::authenticated(),
                Resource::UserGroup(ResourceAction::List),
            ),
            AccessPolicy::new(
                Allow::authenticated(),
                Resource::UserGroup(ResourceAction::Read(ResourceId::Any)),
            ),
            AccessPolicy::new(
                Allow::authenticated(),
                Resource::UserGroup(ResourceAction::Update(ResourceId::Any)),
            ),
            AccessPolicy::new(
                Allow::authenticated(),
                Resource::Proposal(ProposalResourceAction::List),
            ),
            AccessPolicy::new(
                Allow::authenticated(),
                Resource::Proposal(ProposalResourceAction::Read(ResourceId::Any)),
            ),
        ]);

    }

    /// Generates a random access policy for testing purposes.
    pub fn mock_access_policy() -> AccessPolicy {
        let policy = RANDOM_MOCKED_POLICY.with(|num| {
            let num = *num.borrow();
            AVAILABLE_POLICIES.with(|policies| {
                let policies = &mut *policies.borrow_mut();
                policies[num as usize].clone()
            })
        });

        RANDOM_MOCKED_POLICY.with(|num| {
            *num.borrow_mut() += 1;
            if *num.borrow() > 23 {
                *num.borrow_mut() = 0;
            }
        });

        policy
    }
}
