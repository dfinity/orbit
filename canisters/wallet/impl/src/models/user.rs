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

impl User {
    pub const IDENTITIES_RANGE: (u8, u8) = (1, 10);
    pub const ACCESS_ROLES_RANGE: (u8, u8) = (1, 10);
    pub const MAX_UNCONFIRMED_IDENTITIES: u8 = 9;

    /// Creates a new user key from the given key components.
    pub fn key(id: UserId) -> UserKey {
        UserKey { id }
    }

    pub fn to_key(&self) -> UserKey {
        User::key(self.id)
    }
}

fn validate_identities(identities: &Vec<Principal>) -> ModelValidatorResult<UserError> {
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

fn validate_access_roles(access_roles: &Vec<AccessRole>) -> ModelValidatorResult<UserError> {
    if access_roles.len() < User::ACCESS_ROLES_RANGE.0 as usize {
        return Err(UserError::TooLittleAccessRoles);
    }

    if access_roles.len() > User::ACCESS_ROLES_RANGE.1 as usize {
        return Err(UserError::TooManyAccessRoles {
            max_access_roles: User::ACCESS_ROLES_RANGE.1,
        });
    }

    Ok(())
}

fn validate_unconfirmed_identities(
    unconfirmed_identities: &Vec<Principal>,
) -> ModelValidatorResult<UserError> {
    if unconfirmed_identities.len() > User::MAX_UNCONFIRMED_IDENTITIES as usize {
        return Err(UserError::TooManyUnconfirmedIdentities {
            max_identities: User::MAX_UNCONFIRMED_IDENTITIES,
        });
    }

    Ok(())
}

impl ModelValidator<UserError> for User {
    fn validate(&self) -> ModelValidatorResult<UserError> {
        validate_identities(&self.identities)?;
        validate_unconfirmed_identities(&self.unconfirmed_identities)?;
        validate_access_roles(&self.access_roles)?;

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
    fn fail_user_too_many_unconfirmed_identities() {
        let mut user = mock_user();
        user.unconfirmed_identities =
            vec![Principal::anonymous(); User::MAX_UNCONFIRMED_IDENTITIES as usize + 1];

        let result = validate_unconfirmed_identities(&user.unconfirmed_identities);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            UserError::TooManyUnconfirmedIdentities {
                max_identities: User::MAX_UNCONFIRMED_IDENTITIES
            }
        );
    }

    #[test]
    fn test_user_unconfirmed_identities_validation() {
        let mut user = mock_user();
        user.unconfirmed_identities =
            vec![Principal::anonymous(); User::MAX_UNCONFIRMED_IDENTITIES as usize - 1];

        let result = validate_unconfirmed_identities(&user.unconfirmed_identities);

        assert!(result.is_ok());
    }

    #[test]
    fn test_user_access_roles_validation() {
        let mut user = mock_user();
        user.access_roles = vec![AccessRole::User];

        let result = validate_access_roles(&user.access_roles);

        assert!(result.is_ok());
    }

    #[test]
    fn fail_user_access_roles_too_little() {
        let mut user = mock_user();
        user.access_roles = vec![];

        let result = validate_access_roles(&user.access_roles);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), UserError::TooLittleAccessRoles);
    }

    #[test]
    fn fail_user_access_roles_too_many() {
        let mut user = mock_user();
        user.access_roles = vec![AccessRole::User; User::ACCESS_ROLES_RANGE.1 as usize + 1];

        let result = validate_access_roles(&user.access_roles);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            UserError::TooManyAccessRoles {
                max_access_roles: User::ACCESS_ROLES_RANGE.1
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
