use crate::{errors::UserGroupError, repositories::USER_GROUP_REPOSITORY};
use candid::{CandidType, Deserialize};
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use ic_canister_macros::stable_object;
use uuid::Uuid;

pub const ADMIN_GROUP_ID: &UUID = Uuid::from_u128(302240678275694148452352).as_bytes();

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

impl UserGroup {
    const NAME_RANGE: (u8, u8) = (1, 50);
}

fn validate_name(name: &String) -> ModelValidatorResult<UserGroupError> {
    if name.len() < UserGroup::NAME_RANGE.0 as usize {
        return Err(UserGroupError::NameTooShort {
            min_length: UserGroup::NAME_RANGE.0,
        });
    }

    if name.len() > UserGroup::NAME_RANGE.1 as usize {
        return Err(UserGroupError::NameTooLong {
            max_length: UserGroup::NAME_RANGE.1,
        });
    }

    Ok(())
}

fn validate_unique_name(
    user_group_id: &UUID,
    name: &String,
) -> ModelValidatorResult<UserGroupError> {
    let current_user_group = USER_GROUP_REPOSITORY.find_by_name(name);
    if let Some(current_user_group) = current_user_group {
        if current_user_group.id != *user_group_id {
            return Err(UserGroupError::NonUniqueName {
                name: name.to_string(),
            });
        }
    }

    Ok(())
}

impl ModelValidator<UserGroupError> for UserGroup {
    fn validate(&self) -> ModelValidatorResult<UserGroupError> {
        validate_name(&self.name)?;
        validate_unique_name(&self.id, &self.name)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::user_group_test_utils::mock_user_group;
    use super::*;
    use ic_canister_core::repository::Repository;

    #[test]
    fn fail_user_group_name_too_short() {
        let mut group = mock_user_group();
        group.name = String::new();

        let result = validate_name(&group.name);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            UserGroupError::NameTooShort {
                min_length: UserGroup::NAME_RANGE.0
            }
        );
    }

    #[test]
    fn fail_user_group_name_too_long() {
        let mut group: UserGroup = mock_user_group();
        group.name = "a".repeat(UserGroup::NAME_RANGE.1 as usize + 1);

        let result = validate_name(&group.name);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            UserGroupError::NameTooLong {
                max_length: UserGroup::NAME_RANGE.1
            }
        );
    }

    #[test]
    fn test_user_group_name_validation() {
        let mut group = mock_user_group();
        group.name = "finance".to_string();

        let result = validate_name(&group.name);

        assert!(result.is_ok());
    }

    #[test]
    fn fail_user_group_non_unique_name() {
        let mut group = mock_user_group();
        let mut group1 = mock_user_group();
        group.id = [0; 16];
        group.name = "finance".to_string();
        group1.id = [1; 16];
        group1.name = "finance".to_string();

        USER_GROUP_REPOSITORY.insert(group.id, group.clone());

        let result = validate_unique_name(&group1.id, &group1.name);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            UserGroupError::NonUniqueName {
                name: group.name.to_string()
            }
        );
    }

    #[test]
    fn validation_of_unique_name_change_succeeds() {
        let mut group = mock_user_group();
        group.id = [0; 16];
        group.name = "finance".to_string();

        USER_GROUP_REPOSITORY.insert(group.id, group.clone());

        let result = validate_unique_name(&group.id, &group.name);

        assert!(result.is_ok());
    }
}

#[cfg(test)]
pub mod user_group_test_utils {
    use super::*;
    use ic_canister_core::repository::Repository;

    pub fn mock_user_group() -> UserGroup {
        UserGroup {
            id: [0; 16],
            name: "test".to_string(),
            last_modification_timestamp: 0,
        }
    }

    pub fn add_group(name: &str) -> UserGroup {
        let mut group = mock_user_group();
        group.id = *Uuid::new_v4().as_bytes();
        group.name = name.to_string();

        group.validate().unwrap();

        USER_GROUP_REPOSITORY.insert(group.id, group.clone());

        group
    }
}
