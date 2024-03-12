use super::{User, UserGroupId, UserId};
use candid::CandidType;
use ic_canister_core::{model::ModelKey, types::UUID};
use ic_canister_macros::storable;
use serde::Deserialize;

/// The user gorup id, which is a UUID.
#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AccessPolicyCallerPrivileges {
    pub can_edit: bool,
}

/// Represents an access policy within the system.
///
/// An access policy is a rule that defines who can access a resource and what they can do with it.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd)]
pub struct AccessPolicy {
    /// The users who can access the resource.
    ///
    /// It can be a list of specific users, user groups, or any user.
    pub allow: Allow,
    /// The resource that the user can access.
    pub resource: Resource,
}

/// The unique identifier of an access policy.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AccessPolicyKey {
    /// The resource that the user can access.
    pub resource: Resource,
    /// The user level who can access the resource.
    pub allow_level: AllowLevel,
}

impl Ord for AccessPolicyKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.resource
            .cmp(&other.resource)
            .then(self.allow_level.cmp(&other.allow_level))
    }
}

impl PartialOrd for AccessPolicyKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl ModelKey<AccessPolicyKey> for AccessPolicy {
    fn key(&self) -> AccessPolicyKey {
        AccessPolicyKey {
            allow_level: self.allow.clone().into(),
            resource: self.resource.clone(),
        }
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
pub enum Allow {
    Any,
    Authenticated,
    Users(Vec<UserId>),
    UserGroups(Vec<UserGroupId>),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AllowLevel {
    Any = 1,
    Authenticated = 2,
    Users = 3,
    UserGroups = 4,
}

impl From<Allow> for AllowLevel {
    fn from(allow: Allow) -> Self {
        match allow {
            Allow::Any => AllowLevel::Any,
            Allow::Authenticated => AllowLevel::Authenticated,
            Allow::Users(_) => AllowLevel::Users,
            Allow::UserGroups(_) => AllowLevel::UserGroups,
        }
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
pub enum ResourceTypeId {
    Any,
    Resource(ResourceType),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceType {
    // Account(AccountResourceAction),
    AddressBook(ResourceActionType),
    // ChangeCanister(ChangeCanisterResourceAction),
    // Proposal(ProposalResourceAction),
    ProposalPolicy(ResourceActionType),
    // Settings(SettingsResourceAction),
    // User(UserResourceAction),
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
    Read,
    ReadConfig,
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
            _ => unimplemented!(),
        }
    }
}

impl AccessPolicy {
    /// Checks if the user is allowed to access the resource according to the policy.
    pub fn is_allowed(&self, user: &User) -> bool {
        match &self.allow {
            Allow::Any => true,
            Allow::Authenticated => user.is_active(),
            Allow::Users(ids) => ids.contains(&user.id),
            Allow::UserGroups(ids) => user.groups.iter().any(|group| ids.contains(group)),
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
            0 => AccessPolicy::new(Allow::Any, Resource::Account(AccountResourceAction::Create)),
            1 => AccessPolicy::new(Allow::Any, Resource::Account(AccountResourceAction::List)),
            2 => AccessPolicy::new(
                Allow::Any,
                Resource::Account(AccountResourceAction::Read(ResourceId::Any)),
            ),
            3 => AccessPolicy::new(
                Allow::Any,
                Resource::Account(AccountResourceAction::Transfer(ResourceId::Any)),
            ),
            4 => AccessPolicy::new(
                Allow::Any,
                Resource::Account(AccountResourceAction::Update(ResourceId::Any)),
            ),
            5 => AccessPolicy::new(
                Allow::Any,
                Resource::AccessPolicy(AccessPolicyResourceAction::List),
            ),
            6 => AccessPolicy::new(
                Allow::Any,
                Resource::AccessPolicy(AccessPolicyResourceAction::Edit(ResourceTypeId::Any)),
            ),
            7 => AccessPolicy::new(
                Allow::Any,
                Resource::AccessPolicy(AccessPolicyResourceAction::Read(ResourceTypeId::Any)),
            ),
            8 => AccessPolicy::new(Allow::Any, Resource::AddressBook(ResourceAction::Create)),
            9 => AccessPolicy::new(
                Allow::Any,
                Resource::AddressBook(ResourceAction::Delete(ResourceId::Any)),
            ),
            10 => AccessPolicy::new(Allow::Any, Resource::AddressBook(ResourceAction::List)),
            11 => AccessPolicy::new(
                Allow::Any,
                Resource::AddressBook(ResourceAction::Read(ResourceId::Any)),
            ),
            12 => AccessPolicy::new(
                Allow::Any,
                Resource::AddressBook(ResourceAction::Update(ResourceId::Any)),
            ),
            13 => AccessPolicy::new(Allow::Any, Resource::User(UserResourceAction::Create)),
            14 => AccessPolicy::new(Allow::Any, Resource::User(UserResourceAction::List)),
            15 => AccessPolicy::new(
                Allow::Any,
                Resource::User(UserResourceAction::Read(ResourceId::Any)),
            ),
            16 => AccessPolicy::new(
                Allow::Any,
                Resource::User(UserResourceAction::Update(ResourceId::Any)),
            ),
            17 => AccessPolicy::new(Allow::Any, Resource::UserGroup(ResourceAction::Create)),
            18 => AccessPolicy::new(
                Allow::Any,
                Resource::UserGroup(ResourceAction::Delete(ResourceId::Any)),
            ),
            19 => AccessPolicy::new(Allow::Any, Resource::UserGroup(ResourceAction::List)),
            20 => AccessPolicy::new(
                Allow::Any,
                Resource::UserGroup(ResourceAction::Read(ResourceId::Any)),
            ),
            21 => AccessPolicy::new(
                Allow::Any,
                Resource::UserGroup(ResourceAction::Update(ResourceId::Any)),
            ),
            22 => AccessPolicy::new(Allow::Any, Resource::Proposal(ProposalResourceAction::List)),
            23 => AccessPolicy::new(
                Allow::Any,
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
