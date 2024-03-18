use super::{User, UserGroupId, UserId};
use ic_canister_core::{model::ModelKey, types::UUID};
use ic_canister_macros::storable;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AccessPolicyCallerPrivileges {
    pub resource: Resource,
    pub can_edit: bool,
}

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

impl Resource {
    pub fn min() -> Self {
        Resource::AccessPolicy(AccessPolicyResourceAction::List)
    }

    pub fn max() -> Self {
        Resource::UserGroup(ResourceAction::Update(ResourceId::Any))
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceTypeId {
    Any,
    Resource(ResourceType),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceType {
    AccessPolicy(AccessPolicyResourceActionType),
    Account(AccountResourceActionType),
    AddressBook(ResourceActionType),
    ChangeCanister(ChangeCanisterResourceActionType),
    Proposal(ProposalResourceActionType),
    ProposalPolicy(ResourceActionType),
    Settings(SettingsResourceActionType),
    User(UserResourceActionType),
    UserGroup(ResourceActionType),
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
pub enum ResourceActionType {
    List = 1,
    Create = 2,
    Read = 3,
    Update = 4,
    Delete = 5,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AccessPolicyResourceAction {
    List,
    Read(ResourceTypeId),
    Edit(ResourceTypeId),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AccessPolicyResourceActionType {
    List = 1,
    Read = 2,
    Edit = 3,
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
pub enum UserResourceActionType {
    List = 1,
    Create = 2,
    Read = 3,
    Update = 4,
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
pub enum AccountResourceActionType {
    List = 1,
    Create = 2,
    Transfer = 3,
    Read = 4,
    Update = 5,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SettingsResourceAction {
    Read,
    ReadConfig,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SettingsResourceActionType {
    Read = 1,
    ReadConfig = 2,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChangeCanisterResourceAction {
    Create,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChangeCanisterResourceActionType {
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
pub enum ProposalResourceActionType {
    List = 1,
    Read = 2,
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
                AccountResourceAction::Transfer(ResourceId::Id(ids)) => {
                    vec![
                        Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(*ids))),
                        Resource::Account(AccountResourceAction::Transfer(ResourceId::Any)),
                    ]
                }
                AccountResourceAction::Read(ResourceId::Id(ids)) => {
                    vec![
                        Resource::Account(AccountResourceAction::Read(ResourceId::Id(*ids))),
                        Resource::Account(AccountResourceAction::Read(ResourceId::Any)),
                    ]
                }
                AccountResourceAction::Update(ResourceId::Id(ids)) => {
                    vec![
                        Resource::Account(AccountResourceAction::Update(ResourceId::Id(*ids))),
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
                AccessPolicyResourceAction::List => {
                    vec![Resource::AccessPolicy(AccessPolicyResourceAction::List)]
                }
                AccessPolicyResourceAction::Read(ResourceTypeId::Resource(rtype)) => {
                    vec![
                        Resource::AccessPolicy(AccessPolicyResourceAction::Read(
                            ResourceTypeId::Resource(rtype.clone()),
                        )),
                        Resource::AccessPolicy(AccessPolicyResourceAction::Read(
                            ResourceTypeId::Any,
                        )),
                    ]
                }
                AccessPolicyResourceAction::Edit(ResourceTypeId::Resource(rtype)) => {
                    vec![
                        Resource::AccessPolicy(AccessPolicyResourceAction::Edit(
                            ResourceTypeId::Resource(rtype.clone()),
                        )),
                        Resource::AccessPolicy(AccessPolicyResourceAction::Edit(
                            ResourceTypeId::Any,
                        )),
                    ]
                }
                AccessPolicyResourceAction::Read(ResourceTypeId::Any) => {
                    vec![Resource::AccessPolicy(AccessPolicyResourceAction::Read(
                        ResourceTypeId::Any,
                    ))]
                }
                AccessPolicyResourceAction::Edit(ResourceTypeId::Any) => {
                    vec![Resource::AccessPolicy(AccessPolicyResourceAction::Edit(
                        ResourceTypeId::Any,
                    ))]
                }
            },
            Resource::AddressBook(action) => match action {
                ResourceAction::Create => vec![Resource::AddressBook(ResourceAction::Create)],
                ResourceAction::Delete(ResourceId::Id(ids)) => {
                    vec![
                        Resource::AddressBook(ResourceAction::Delete(ResourceId::Id(*ids))),
                        Resource::AddressBook(ResourceAction::Delete(ResourceId::Any)),
                    ]
                }
                ResourceAction::List => vec![Resource::AddressBook(ResourceAction::List)],
                ResourceAction::Read(ResourceId::Id(ids)) => {
                    vec![
                        Resource::AddressBook(ResourceAction::Read(ResourceId::Id(*ids))),
                        Resource::AddressBook(ResourceAction::Read(ResourceId::Any)),
                    ]
                }
                ResourceAction::Update(ResourceId::Id(ids)) => {
                    vec![
                        Resource::AddressBook(ResourceAction::Update(ResourceId::Id(*ids))),
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
                ProposalResourceAction::Read(ResourceId::Id(ids)) => {
                    vec![
                        Resource::Proposal(ProposalResourceAction::Read(ResourceId::Id(*ids))),
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
                ResourceAction::Delete(ResourceId::Id(ids)) => {
                    vec![
                        Resource::ProposalPolicy(ResourceAction::Delete(ResourceId::Id(*ids))),
                        Resource::ProposalPolicy(ResourceAction::Delete(ResourceId::Any)),
                    ]
                }
                ResourceAction::List => vec![Resource::ProposalPolicy(ResourceAction::List)],
                ResourceAction::Read(ResourceId::Id(ids)) => {
                    vec![
                        Resource::ProposalPolicy(ResourceAction::Read(ResourceId::Id(*ids))),
                        Resource::ProposalPolicy(ResourceAction::Read(ResourceId::Any)),
                    ]
                }
                ResourceAction::Update(ResourceId::Id(ids)) => {
                    vec![
                        Resource::ProposalPolicy(ResourceAction::Update(ResourceId::Id(*ids))),
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
                SettingsResourceAction::Read => {
                    vec![Resource::Settings(SettingsResourceAction::Read)]
                }
                SettingsResourceAction::ReadConfig => {
                    vec![Resource::Settings(SettingsResourceAction::ReadConfig)]
                }
            },
            Resource::User(action) => match action {
                UserResourceAction::Create => vec![Resource::User(UserResourceAction::Create)],
                UserResourceAction::List => vec![Resource::User(UserResourceAction::List)],
                UserResourceAction::Read(ResourceId::Id(ids)) => {
                    vec![
                        Resource::User(UserResourceAction::Read(ResourceId::Id(*ids))),
                        Resource::User(UserResourceAction::Read(ResourceId::Any)),
                    ]
                }
                UserResourceAction::Update(ResourceId::Id(ids)) => {
                    vec![
                        Resource::User(UserResourceAction::Update(ResourceId::Id(*ids))),
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
                ResourceAction::Delete(ResourceId::Id(ids)) => {
                    vec![
                        Resource::UserGroup(ResourceAction::Delete(ResourceId::Id(*ids))),
                        Resource::UserGroup(ResourceAction::Delete(ResourceId::Any)),
                    ]
                }
                ResourceAction::List => vec![Resource::UserGroup(ResourceAction::List)],
                ResourceAction::Read(ResourceId::Id(ids)) => {
                    vec![
                        Resource::UserGroup(ResourceAction::Read(ResourceId::Id(*ids))),
                        Resource::UserGroup(ResourceAction::Read(ResourceId::Any)),
                    ]
                }
                ResourceAction::Update(ResourceId::Id(ids)) => {
                    vec![
                        Resource::UserGroup(ResourceAction::Update(ResourceId::Id(*ids))),
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

    pub fn to_type(&self) -> ResourceType {
        match self.to_owned() {
            Resource::AddressBook(action) => ResourceType::AddressBook(action.into()),
            Resource::ProposalPolicy(action) => ResourceType::ProposalPolicy(action.into()),
            Resource::UserGroup(action) => ResourceType::UserGroup(action.into()),
            Resource::Account(action) => ResourceType::Account(action.into()),
            Resource::ChangeCanister(action) => ResourceType::ChangeCanister(action.into()),
            Resource::Proposal(action) => ResourceType::Proposal(action.into()),
            Resource::Settings(action) => ResourceType::Settings(action.into()),
            Resource::User(action) => ResourceType::User(action.into()),
            Resource::AccessPolicy(action) => ResourceType::AccessPolicy(action.into()),
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
            AccessPolicyResourceAction::List => write!(f, "List"),
            AccessPolicyResourceAction::Read(id) => write!(f, "Read({})", id),
            AccessPolicyResourceAction::Edit(id) => write!(f, "Edit({})", id),
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
            SettingsResourceAction::Read => write!(f, "Read"),
            SettingsResourceAction::ReadConfig => write!(f, "ReadConfig"),
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

impl Display for ResourceTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceTypeId::Any => write!(f, "Any"),
            ResourceTypeId::Resource(rtype) => write!(f, "Resource({})", rtype),
        }
    }
}

impl Display for ResourceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceType::AccessPolicy(action) => write!(f, "AccessPolicy({})", action),
            ResourceType::Account(action) => write!(f, "Account({})", action),
            ResourceType::AddressBook(action) => write!(f, "AddressBook({})", action),
            ResourceType::ChangeCanister(action) => write!(f, "ChangeCanister({})", action),
            ResourceType::Proposal(action) => write!(f, "Proposal({})", action),
            ResourceType::ProposalPolicy(action) => write!(f, "ProposalPolicy({})", action),
            ResourceType::Settings(action) => write!(f, "Settings({})", action),
            ResourceType::User(action) => write!(f, "User({})", action),
            ResourceType::UserGroup(action) => write!(f, "UserGroup({})", action),
        }
    }
}

impl Display for AccessPolicyResourceActionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AccessPolicyResourceActionType::List => write!(f, "List"),
            AccessPolicyResourceActionType::Read => write!(f, "Read"),
            AccessPolicyResourceActionType::Edit => write!(f, "Edit"),
        }
    }
}

impl Display for AccountResourceActionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountResourceActionType::List => write!(f, "List"),
            AccountResourceActionType::Create => write!(f, "Create"),
            AccountResourceActionType::Transfer => write!(f, "Transfer"),
            AccountResourceActionType::Read => write!(f, "Read"),
            AccountResourceActionType::Update => write!(f, "Update"),
        }
    }
}

impl Display for ResourceActionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceActionType::List => write!(f, "List"),
            ResourceActionType::Create => write!(f, "Create"),
            ResourceActionType::Read => write!(f, "Read"),
            ResourceActionType::Update => write!(f, "Update"),
            ResourceActionType::Delete => write!(f, "Delete"),
        }
    }
}

impl Display for ChangeCanisterResourceActionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangeCanisterResourceActionType::Create => write!(f, "Create"),
        }
    }
}

impl Display for ProposalResourceActionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposalResourceActionType::List => write!(f, "List"),
            ProposalResourceActionType::Read => write!(f, "Read"),
        }
    }
}

impl Display for SettingsResourceActionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsResourceActionType::Read => write!(f, "Read"),
            SettingsResourceActionType::ReadConfig => write!(f, "ReadConfig"),
        }
    }
}

impl Display for UserResourceActionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UserResourceActionType::List => write!(f, "List"),
            UserResourceActionType::Create => write!(f, "Create"),
            UserResourceActionType::Read => write!(f, "Read"),
            UserResourceActionType::Update => write!(f, "Update"),
        }
    }
}

#[cfg(any(test, feature = "canbench"))]
pub mod access_policy_test_utils {
    use super::*;
    use std::cell::RefCell;

    thread_local! {
        static RANDOM_MOCKED_POLICY: RefCell<u8> = RefCell::new(0);
    }

    /// Generates a random access policy for testing purposes.
    pub fn mock_access_policy() -> AccessPolicy {
        let policy = match RANDOM_MOCKED_POLICY.with(|num| *num.borrow()) {
            0 => AccessPolicy::new(
                Allow::public(),
                Resource::Account(AccountResourceAction::Create),
            ),
            1 => AccessPolicy::new(
                Allow::authenticated(),
                Resource::Account(AccountResourceAction::List),
            ),
            2 => AccessPolicy::new(
                Allow::authenticated(),
                Resource::Account(AccountResourceAction::Read(ResourceId::Any)),
            ),
            3 => AccessPolicy::new(
                Allow::authenticated(),
                Resource::Account(AccountResourceAction::Transfer(ResourceId::Any)),
            ),
            4 => AccessPolicy::new(
                Allow::authenticated(),
                Resource::Account(AccountResourceAction::Update(ResourceId::Any)),
            ),
            5 => AccessPolicy::new(
                Allow::authenticated(),
                Resource::AccessPolicy(AccessPolicyResourceAction::List),
            ),
            6 => AccessPolicy::new(
                Allow::authenticated(),
                Resource::AccessPolicy(AccessPolicyResourceAction::Edit(ResourceTypeId::Any)),
            ),
            7 => AccessPolicy::new(
                Allow::authenticated(),
                Resource::AccessPolicy(AccessPolicyResourceAction::Read(ResourceTypeId::Any)),
            ),
            8 => AccessPolicy::new(
                Allow::authenticated(),
                Resource::AddressBook(ResourceAction::Create),
            ),
            9 => AccessPolicy::new(
                Allow::authenticated(),
                Resource::AddressBook(ResourceAction::Delete(ResourceId::Any)),
            ),
            10 => AccessPolicy::new(
                Allow::authenticated(),
                Resource::AddressBook(ResourceAction::List),
            ),
            11 => AccessPolicy::new(
                Allow::authenticated(),
                Resource::AddressBook(ResourceAction::Read(ResourceId::Any)),
            ),
            12 => AccessPolicy::new(
                Allow::authenticated(),
                Resource::AddressBook(ResourceAction::Update(ResourceId::Any)),
            ),
            13 => AccessPolicy::new(
                Allow::authenticated(),
                Resource::User(UserResourceAction::Create),
            ),
            14 => AccessPolicy::new(
                Allow::authenticated(),
                Resource::User(UserResourceAction::List),
            ),
            15 => AccessPolicy::new(
                Allow::authenticated(),
                Resource::User(UserResourceAction::Read(ResourceId::Any)),
            ),
            16 => AccessPolicy::new(
                Allow::authenticated(),
                Resource::User(UserResourceAction::Update(ResourceId::Any)),
            ),
            17 => AccessPolicy::new(
                Allow::authenticated(),
                Resource::UserGroup(ResourceAction::Create),
            ),
            18 => AccessPolicy::new(
                Allow::authenticated(),
                Resource::UserGroup(ResourceAction::Delete(ResourceId::Any)),
            ),
            19 => AccessPolicy::new(
                Allow::authenticated(),
                Resource::UserGroup(ResourceAction::List),
            ),
            20 => AccessPolicy::new(
                Allow::authenticated(),
                Resource::UserGroup(ResourceAction::Read(ResourceId::Any)),
            ),
            21 => AccessPolicy::new(
                Allow::authenticated(),
                Resource::UserGroup(ResourceAction::Update(ResourceId::Any)),
            ),
            22 => AccessPolicy::new(
                Allow::authenticated(),
                Resource::Proposal(ProposalResourceAction::List),
            ),
            23 => AccessPolicy::new(
                Allow::authenticated(),
                Resource::Proposal(ProposalResourceAction::Read(ResourceId::Any)),
            ),
            _ => panic!("Invalid random mocked policy"),
        };

        RANDOM_MOCKED_POLICY.with(|num| {
            *num.borrow_mut() += 1;
            if *num.borrow() > 23 {
                *num.borrow_mut() = 0;
            }
        });

        policy
    }
}
