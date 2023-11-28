use super::AccessRole;
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
    /// The identities associated with the user.
    pub identities: Vec<Principal>,
    /// The unconfirmed identities associated with the user.
    pub unconfirmed_identities: Vec<Principal>,
    /// The access roles associated with the user.
    pub access_roles: Vec<AccessRole>,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UserKey {
    pub id: UserId,
}

pub struct UserValidator<'user> {
    user: &'user User,
}

impl<'user> UserValidator<'user> {
    pub const IDENTITIES_RANGE: (u8, u8) = (1, 10);
    pub const ACCESS_ROLES_RANGE: (u8, u8) = (1, 10);
    pub const MAX_UNCONFIRMED_IDENTITIES: u8 = 9;

    pub fn new(user: &'user User) -> UserValidator {
        UserValidator { user }
    }

    pub fn validate_identities(&self) -> ModelValidatorResult<UserError> {
        if self.user.identities.len() < Self::IDENTITIES_RANGE.0 as usize {
            return Err(UserError::TooLittleIdentities);
        }

        if self.user.identities.len() > Self::IDENTITIES_RANGE.1 as usize {
            return Err(UserError::TooManyIdentities {
                max_identities: Self::IDENTITIES_RANGE.1,
            });
        }

        Ok(())
    }

    pub fn validate_access_roles(&self) -> ModelValidatorResult<UserError> {
        if self.user.access_roles.len() < Self::ACCESS_ROLES_RANGE.0 as usize {
            return Err(UserError::TooLittleAccessRoles);
        }

        if self.user.access_roles.len() > Self::ACCESS_ROLES_RANGE.1 as usize {
            return Err(UserError::TooManyAccessRoles {
                max_access_roles: Self::ACCESS_ROLES_RANGE.1,
            });
        }

        Ok(())
    }

    pub fn validate_unconfirmed_identities(&self) -> ModelValidatorResult<UserError> {
        if self.user.unconfirmed_identities.len() > Self::MAX_UNCONFIRMED_IDENTITIES as usize {
            return Err(UserError::TooManyUnconfirmedIdentities {
                max_identities: Self::MAX_UNCONFIRMED_IDENTITIES,
            });
        }

        Ok(())
    }

    pub fn validate(&self) -> ModelValidatorResult<UserError> {
        self.validate_identities()?;
        self.validate_unconfirmed_identities()?;
        self.validate_access_roles()?;

        Ok(())
    }
}

impl ModelValidator<UserError> for User {
    fn validate(&self) -> ModelValidatorResult<UserError> {
        UserValidator::new(self).validate()
    }
}

impl User {
    /// Creates a new user key from the given key components.
    pub fn key(id: UserId) -> UserKey {
        UserKey { id }
    }

    pub fn to_key(&self) -> UserKey {
        User::key(self.id)
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

        let result = UserValidator::new(&user).validate_identities();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), UserError::TooLittleIdentities);
    }

    #[test]
    fn fail_user_too_many_identities() {
        let mut user = mock_user();
        user.identities =
            vec![Principal::anonymous(); UserValidator::IDENTITIES_RANGE.1 as usize + 1];

        let result = UserValidator::new(&user).validate_identities();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            UserError::TooManyIdentities {
                max_identities: UserValidator::IDENTITIES_RANGE.1
            }
        );
    }

    #[test]
    fn test_user_identities_validation() {
        let mut user = mock_user();
        user.identities = vec![Principal::anonymous(); 5];

        let result = UserValidator::new(&user).validate_identities();

        assert!(result.is_ok());
    }

    #[test]
    fn fail_user_too_many_unconfirmed_identities() {
        let mut user = mock_user();
        user.unconfirmed_identities =
            vec![Principal::anonymous(); UserValidator::MAX_UNCONFIRMED_IDENTITIES as usize + 1];

        let result = UserValidator::new(&user).validate_unconfirmed_identities();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            UserError::TooManyUnconfirmedIdentities {
                max_identities: UserValidator::MAX_UNCONFIRMED_IDENTITIES
            }
        );
    }

    #[test]
    fn test_user_unconfirmed_identities_validation() {
        let mut user = mock_user();
        user.unconfirmed_identities =
            vec![Principal::anonymous(); UserValidator::MAX_UNCONFIRMED_IDENTITIES as usize - 1];

        let result = UserValidator::new(&user).validate_unconfirmed_identities();

        assert!(result.is_ok());
    }

    #[test]
    fn test_user_access_roles_validation() {
        let mut user = mock_user();
        user.access_roles = vec![AccessRole::User];

        let result = UserValidator::new(&user).validate_access_roles();

        assert!(result.is_ok());
    }

    #[test]
    fn fail_user_access_roles_too_little() {
        let mut user = mock_user();
        user.access_roles = vec![];

        let result = UserValidator::new(&user).validate_access_roles();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), UserError::TooLittleAccessRoles);
    }

    #[test]
    fn fail_user_access_roles_too_many() {
        let mut user = mock_user();
        user.access_roles =
            vec![AccessRole::User; UserValidator::ACCESS_ROLES_RANGE.1 as usize + 1];

        let result = UserValidator::new(&user).validate_access_roles();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            UserError::TooManyAccessRoles {
                max_access_roles: UserValidator::ACCESS_ROLES_RANGE.1
            }
        );
    }
}

#[cfg(test)]
pub mod user_test_utils {
    use super::*;

    pub fn mock_user() -> User {
        User {
            id: [0; 16],
            identities: vec![Principal::anonymous()],
            unconfirmed_identities: vec![],
            access_roles: vec![AccessRole::User],
            last_modification_timestamp: 0,
        }
    }
}
