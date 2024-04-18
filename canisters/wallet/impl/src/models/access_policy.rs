use crate::core::validation::{EnsureIdExists, EnsureUser, EnsureUserGroup, RecordNotFoundError};

use super::{resource::Resource, User, UserGroupId, UserId};
use ic_canister_core::model::{ModelKey, ModelValidator, ModelValidatorResult};
use ic_canister_macros::storable;
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

impl ModelValidator<RecordNotFoundError> for Allow {
    fn validate(&self) -> ModelValidatorResult<RecordNotFoundError> {
        for user_id in &self.users {
            EnsureUser::id_exists(user_id)?;
        }
        for group_id in &self.user_groups {
            EnsureUserGroup::id_exists(group_id)?;
        }
        Ok(())
    }
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

#[cfg(any(test, feature = "canbench"))]
pub mod access_policy_test_utils {
    use crate::models::resource::{
        AccessPolicyResourceAction, AccountResourceAction, ProposalResourceAction, ResourceAction,
        ResourceId, UserResourceAction,
    };

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

#[cfg(test)]
mod test {
    use ic_canister_core::model::ModelValidator;

    use crate::core::validation::disable_mock_resource_validation;

    use super::{Allow, AuthScope};

    #[test]
    fn test_validate_default_allow() {
        disable_mock_resource_validation();

        let allow = Allow::default();
        allow.validate().expect("Default allow should be valid");
    }

    #[test]
    fn fail_allow_with_non_existent_user() {
        disable_mock_resource_validation();

        let allow = Allow {
            auth_scope: AuthScope::Restricted,
            users: vec![[1; 16]],
            user_groups: vec![],
        };

        allow
            .validate()
            .expect_err("Allow with non-existent user should fail");
    }

    #[test]
    fn fail_allow_with_non_existent_user_group() {
        disable_mock_resource_validation();

        let allow = Allow {
            auth_scope: AuthScope::Restricted,
            users: vec![],
            user_groups: vec![[1; 16]],
        };

        allow
            .validate()
            .expect_err("Allow with non-existent user group should fail");
    }
}
