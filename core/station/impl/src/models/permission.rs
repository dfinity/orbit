use crate::{
    core::validation::{EnsureIdExists, EnsureUser, EnsureUserGroup},
    errors::RecordValidationError,
};

use super::{resource::Resource, User, UserGroupId, UserId};
use orbit_essentials::model::{ModelKey, ModelValidator, ModelValidatorResult};
use orbit_essentials::storable;

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

impl ModelValidator<RecordValidationError> for Allow {
    fn validate(&self) -> ModelValidatorResult<RecordValidationError> {
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

/// Represents a permission within the system.
///
/// A permission is a rule that defines who can access a resource and what they can do with it.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd)]
pub struct Permission {
    /// The resource that the user can access.
    pub resource: Resource,
    /// The users who can access the resource.
    ///
    /// It can be a list of specific users, user groups, or any user.
    pub allow: Allow,
}

impl Permission {
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

/// The unique identifier of a permission.
pub type PermissionKey = Resource;

impl ModelKey<PermissionKey> for Permission {
    fn key(&self) -> Resource {
        self.resource.clone()
    }
}

impl Permission {
    /// Creates a new permission with the given allow and resource.
    ///
    /// The id is generated automatically.
    pub fn new(allow: Allow, resource: Resource) -> Self {
        Self { allow, resource }
    }
}

#[cfg(any(test, feature = "canbench"))]
pub mod permission_test_utils {
    use crate::models::resource::{
        AccountResourceAction, PermissionResourceAction, ProposalResourceAction, ResourceAction,
        ResourceId, UserResourceAction,
    };

    use super::*;
    use std::cell::RefCell;

    thread_local! {
        static RANDOM_MOCKED_POLICY: RefCell<u8> = const { RefCell::new(0) };
        static AVAILABLE_POLICIES: RefCell<Vec<Permission>> = RefCell::new(vec![
            Permission::new(
                Allow::public(),
                Resource::Account(AccountResourceAction::Create),
            ),
            Permission::new(
                Allow::authenticated(),
                Resource::Account(AccountResourceAction::List),
            ),
            Permission::new(
                Allow::authenticated(),
                Resource::Account(AccountResourceAction::Read(ResourceId::Any)),
            ),
            Permission::new(
                Allow::authenticated(),
                Resource::Account(AccountResourceAction::Transfer(ResourceId::Any)),
            ),
            Permission::new(
                Allow::authenticated(),
                Resource::Account(AccountResourceAction::Update(ResourceId::Any)),
            ),
            Permission::new(
                Allow::authenticated(),
                Resource::Permission(PermissionResourceAction::Read),
            ),
            Permission::new(
                Allow::authenticated(),
                Resource::Permission(PermissionResourceAction::Update),
            ),
            Permission::new(
                Allow::authenticated(),
                Resource::AddressBook(ResourceAction::Create),
            ),
            Permission::new(
                Allow::authenticated(),
                Resource::AddressBook(ResourceAction::Delete(ResourceId::Any)),
            ),
            Permission::new(
                Allow::authenticated(),
                Resource::AddressBook(ResourceAction::List),
            ),
            Permission::new(
                Allow::authenticated(),
                Resource::AddressBook(ResourceAction::Read(ResourceId::Any)),
            ),
            Permission::new(
                Allow::authenticated(),
                Resource::AddressBook(ResourceAction::Update(ResourceId::Any)),
            ),
            Permission::new(
                Allow::authenticated(),
                Resource::User(UserResourceAction::Create),
            ),
            Permission::new(
                Allow::authenticated(),
                Resource::User(UserResourceAction::List),
            ),
            Permission::new(
                Allow::authenticated(),
                Resource::User(UserResourceAction::Read(ResourceId::Any)),
            ),
            Permission::new(
                Allow::authenticated(),
                Resource::User(UserResourceAction::Update(ResourceId::Any)),
            ),
            Permission::new(
                Allow::authenticated(),
                Resource::UserGroup(ResourceAction::Create),
            ),
            Permission::new(
                Allow::authenticated(),
                Resource::UserGroup(ResourceAction::Delete(ResourceId::Any)),
            ),
            Permission::new(
                Allow::authenticated(),
                Resource::UserGroup(ResourceAction::List),
            ),
            Permission::new(
                Allow::authenticated(),
                Resource::UserGroup(ResourceAction::Read(ResourceId::Any)),
            ),
            Permission::new(
                Allow::authenticated(),
                Resource::UserGroup(ResourceAction::Update(ResourceId::Any)),
            ),
            Permission::new(
                Allow::authenticated(),
                Resource::Proposal(ProposalResourceAction::List),
            ),
            Permission::new(
                Allow::authenticated(),
                Resource::Proposal(ProposalResourceAction::Read(ResourceId::Any)),
            ),
        ]);

    }

    /// Generates a random permission for testing purposes.
    pub fn mock_permission() -> Permission {
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
    use orbit_essentials::model::ModelValidator;

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
