use super::UserStatus;
use crate::{
    core::validation::{EnsureIdExists, EnsureUserGroup},
    errors::{RecordValidationError, UserError},
};
use candid::{CandidType, Deserialize, Principal};
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use ic_canister_macros::storable;

/// The user id, which is a UUID.
pub type UserId = UUID;

/// Represents a user within the system.
///
/// A user can be associated with one or more identity.
#[storable]
#[derive(CandidType, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct User {
    /// The user id, which is a UUID.
    pub id: UserId,
    /// The name of the user (if any).
    pub name: Option<String>,
    /// The user status within the system (e.g. active, inactive, etc.)
    pub status: UserStatus,
    /// The identities associated with the user.
    pub identities: Vec<Principal>,
    /// The groups the user is a member of (e.g. Finance Team, Admin, etc.)
    pub groups: Vec<UUID>,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UserKey {
    pub id: UserId,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct UserCallerPrivileges {
    pub id: UUID,
    pub can_edit: bool,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct DisplayUser {
    pub id: UUID,
    pub name: Option<String>,
}

impl User {
    pub const IDENTITIES_RANGE: (u8, u8) = (1, 10);
    pub const MAX_USER_GROUPS: u8 = 25;
    pub const MAX_NAME_LENGTH: u8 = 50;

    /// Creates a new user key from the given key components.
    pub fn key(id: UserId) -> UserKey {
        UserKey { id }
    }

    pub fn to_key(&self) -> UserKey {
        User::key(self.id)
    }

    pub fn is_active(&self) -> bool {
        self.status == UserStatus::Active
    }
}

fn validate_identities(identities: &[Principal]) -> ModelValidatorResult<UserError> {
    if identities.len() < User::IDENTITIES_RANGE.0 as usize {
        return Err(UserError::TooLittleIdentities);
    }

    if identities.len() > User::IDENTITIES_RANGE.1 as usize {
        return Err(UserError::TooManyIdentities {
            max_identities: User::IDENTITIES_RANGE.1,
        });
    }

    for identity in identities {
        if Principal::anonymous() == *identity {
            return Err(UserError::IdentityNotAllowed {
                identity: identity.to_text(),
            });
        }
    }

    Ok(())
}

fn validate_groups(group_ids: &[UUID]) -> ModelValidatorResult<UserError> {
    if group_ids.len() > User::MAX_USER_GROUPS as usize {
        return Err(UserError::TooManyUserGroups {
            max: User::MAX_USER_GROUPS,
        });
    }

    EnsureUserGroup::id_list_exists(group_ids).map_err(|err| match err {
        RecordValidationError::NotFound { id, .. } => {
            UserError::UserGroupDoesNotExist { group_id: id }
        }
    })?;

    Ok(())
}

fn validate_name(name: &Option<String>) -> ModelValidatorResult<UserError> {
    if let Some(name) = name {
        if name.len() > User::MAX_NAME_LENGTH as usize {
            return Err(UserError::NameTooLong {
                max_length: User::MAX_NAME_LENGTH as usize,
            });
        }
    }

    Ok(())
}

impl ModelValidator<UserError> for User {
    fn validate(&self) -> ModelValidatorResult<UserError> {
        validate_identities(&self.identities)?;
        validate_groups(&self.groups)?;
        validate_name(&self.name)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::core::validation::disable_mock_resource_validation;
    use crate::models::UserGroup;
    use crate::repositories::USER_GROUP_REPOSITORY;
    use ic_canister_core::repository::Repository;

    use super::user_test_utils::mock_user;
    use super::*;

    #[test]
    fn fail_user_too_little_identities() {
        let mut user = mock_user();
        user.identities = vec![];

        let result = validate_identities(&user.identities);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), UserError::TooLittleIdentities);
    }

    #[test]
    fn fail_user_too_many_identities() {
        let mut user = mock_user();
        user.identities =
            vec![Principal::from_slice(&[1; 29]); User::IDENTITIES_RANGE.1 as usize + 1];

        let result = validate_identities(&user.identities);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            UserError::TooManyIdentities {
                max_identities: User::IDENTITIES_RANGE.1
            }
        );
    }

    #[test]
    fn test_user_identities_validation() {
        let mut user = mock_user();
        user.identities = vec![Principal::from_slice(&[1; 29]); 5];

        let result = validate_identities(&user.identities);

        assert!(result.is_ok());
    }

    #[test]
    fn test_identity_not_allowed() {
        let mut user = mock_user();
        user.identities = vec![Principal::anonymous()];

        let result = validate_identities(&user.identities);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            UserError::IdentityNotAllowed {
                identity: Principal::anonymous().to_text()
            }
        );
    }

    #[test]
    fn test_user_access_roles_validation() {
        let mut user = mock_user();
        user.groups = vec![];

        let result = validate_groups(&user.groups);

        assert!(result.is_ok());
    }

    #[test]
    fn fail_user_groups_too_many() {
        let mut user = mock_user();

        let groups = (0..=User::MAX_USER_GROUPS)
            .map(|i| {
                let id = [i + 1; 16];
                USER_GROUP_REPOSITORY.insert(
                    id,
                    UserGroup {
                        id,
                        last_modification_timestamp: 0,
                        name: format!("group_{}", i),
                    },
                );
                id
            })
            .collect::<Vec<_>>();

        user.groups = groups;

        let result = validate_groups(&user.groups);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            UserError::TooManyUserGroups {
                max: User::MAX_USER_GROUPS
            }
        );
    }

    #[test]
    fn fail_non_existent_user_group() {
        disable_mock_resource_validation();

        let mut user = mock_user();
        user.groups = vec![[1; 16]];

        let result = validate_groups(&user.groups);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            UserError::UserGroupDoesNotExist {
                group_id: Uuid::from_bytes(user.groups[0]).hyphenated().to_string()
            }
        );
    }

    #[test]
    fn fail_user_name_too_long() {
        let mut user = mock_user();
        user.name = Some("a".repeat(User::MAX_NAME_LENGTH as usize + 1));

        let result = validate_name(&user.name);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            UserError::NameTooLong {
                max_length: User::MAX_NAME_LENGTH as usize
            }
        );
    }

    #[test]
    fn test_user_name_validation() {
        let mut user = mock_user();
        user.name = Some("a".repeat(User::MAX_NAME_LENGTH as usize));

        let result = validate_name(&user.name);

        assert!(result.is_ok());
    }
}

#[cfg(any(test, feature = "canbench"))]
pub mod user_test_utils {
    use super::*;
    use crate::repositories::USER_REPOSITORY;
    use ic_canister_core::repository::Repository;
    use uuid::Uuid;

    pub fn mock_user() -> User {
        User {
            id: *Uuid::new_v4().as_bytes(),
            identities: vec![Principal::from_slice(&[24; 29])],
            groups: vec![],
            name: None,
            status: UserStatus::Active,
            last_modification_timestamp: 0,
        }
    }

    pub fn add_user(id: &UUID) -> User {
        let mut user = mock_user();
        user.id = id.to_owned();
        user.status = UserStatus::Active;
        USER_REPOSITORY.insert(user.to_key(), user.to_owned());

        user
    }

    pub fn add_inactive_user(id: &UUID) -> User {
        let mut user = mock_user();
        user.id = id.to_owned();
        user.status = UserStatus::Inactive;
        USER_REPOSITORY.insert(user.to_key(), user.to_owned());

        user
    }
}
