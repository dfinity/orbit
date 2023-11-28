use crate::errors::UserGroupError;
use candid::{CandidType, Deserialize};
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use ic_canister_macros::stable_object;

/// Represents a user group within the system.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UserGroup {
    /// The user group id, which is a UUID.
    pub id: UUID,
    /// The name of the user group (e.g. "Finance").
    pub name: String,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UserGroupKey {
    pub id: UUID,
}

pub struct UserGroupValidator<'group> {
    group: &'group UserGroup,
}

impl<'group> UserGroupValidator<'group> {
    pub const NAME_RANGE: (u8, u8) = (1, 50);

    pub fn new(group: &'group UserGroup) -> UserGroupValidator {
        UserGroupValidator { group }
    }

    pub fn validate_name(&self) -> ModelValidatorResult<UserGroupError> {
        if self.group.name.len() < Self::NAME_RANGE.0 as usize {
            return Err(UserGroupError::NameTooShort {
                min_length: Self::NAME_RANGE.0,
            });
        }

        if self.group.name.len() > Self::NAME_RANGE.1 as usize {
            return Err(UserGroupError::NameTooLong {
                max_length: Self::NAME_RANGE.1,
            });
        }

        Ok(())
    }

    pub fn validate(&self) -> ModelValidatorResult<UserGroupError> {
        self.validate_name()?;

        Ok(())
    }
}

impl ModelValidator<UserGroupError> for UserGroup {
    fn validate(&self) -> ModelValidatorResult<UserGroupError> {
        UserGroupValidator::new(self).validate()
    }
}

impl UserGroup {
    pub fn key(id: UUID) -> UserGroupKey {
        UserGroupKey { id }
    }

    pub fn to_key(&self) -> UserGroupKey {
        UserGroup::key(self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::user_group_test_utils::mock_user_group;
    use super::*;

    #[test]
    fn fail_user_group_name_too_short() {
        let mut group = mock_user_group();
        group.name = String::new();

        let result = UserGroupValidator::new(&group).validate_name();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            UserGroupError::NameTooShort {
                min_length: UserGroupValidator::NAME_RANGE.0
            }
        );
    }

    #[test]
    fn fail_user_group_name_too_long() {
        let mut group: UserGroup = mock_user_group();
        group.name = "a".repeat(UserGroupValidator::NAME_RANGE.1 as usize + 1);

        let result = UserGroupValidator::new(&group).validate_name();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            UserGroupError::NameTooLong {
                max_length: UserGroupValidator::NAME_RANGE.1
            }
        );
    }

    #[test]
    fn test_user_group_name_validation() {
        let mut user = mock_user_group();
        user.name = "finance".to_string();

        let result = UserGroupValidator::new(&user).validate_name();

        assert!(result.is_ok());
    }
}

#[cfg(test)]
pub mod user_group_test_utils {
    use super::*;

    pub fn mock_user_group() -> UserGroup {
        UserGroup {
            id: [0; 16],
            name: "test".to_string(),
            last_modification_timestamp: 0,
        }
    }
}
