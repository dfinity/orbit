use super::UserStatus;
use crate::errors::UserError;
use candid::{CandidType, Deserialize, Principal};
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use ic_canister_macros::stable_object;

/// The user id, which is a UUID.
pub type UserId = UUID;

/// Represents a user within the system.
///
/// A user can be associated with one or more identity.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UserKey {
    pub id: UserId,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct UserCallerPrivileges {
    pub id: UUID,
    pub can_edit: bool,
    pub can_delete: bool,
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

    Ok(())
}

fn validate_groups(access_roles: &[UUID]) -> ModelValidatorResult<UserError> {
    if access_roles.len() > User::MAX_USER_GROUPS as usize {
        return Err(UserError::TooManyUserGroups {
            max: User::MAX_USER_GROUPS,
        });
    }

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
        user.identities = vec![Principal::anonymous(); User::IDENTITIES_RANGE.1 as usize + 1];

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
        user.identities = vec![Principal::anonymous(); 5];

        let result = validate_identities(&user.identities);

        assert!(result.is_ok());
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
        user.groups = vec![[0; 16]; User::MAX_USER_GROUPS as usize + 1];

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

#[cfg(test)]
pub mod user_test_utils {
    use super::*;
    use crate::repositories::USER_REPOSITORY;
    use ic_canister_core::repository::Repository;

    pub fn mock_user() -> User {
        User {
            id: [0; 16],
            identities: vec![Principal::anonymous()],
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
