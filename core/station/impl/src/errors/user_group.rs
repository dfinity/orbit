use orbit_essentials::api::DetailableError;
use thiserror::Error;

use super::ValidationError;

/// Container for user group errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum UserGroupError {
    /// The user group name is not unique.
    #[error(r#"The user group name "{name}" is not unique."#)]
    NonUniqueName {
        /// The user group name.
        name: String,
    },
    /// The user group was not found.
    #[error("The user group with id {id} was not found.")]
    NotFound {
        /// The user group id.
        id: String,
    },
    #[error("Cannot delete the user group marked as the disaster recovery committee.")]
    CannotDeleteDisasterRecoveryCommittee {
        /// The user group id.
        id: String,
    },
    /// The user group with id `{id}` already exists.
    #[error(r#"The user group with id `{id}` already exists."#)]
    IdAlreadyExists { id: String },
    /// The user group has failed validation.
    #[error(r#"The user group has failed validation."#)]
    ValidationError { info: String },
}

impl DetailableError for UserGroupError {
    fn details(&self) -> Option<std::collections::HashMap<String, String>> {
        let mut details = std::collections::HashMap::new();
        match self {
            UserGroupError::NonUniqueName { name } => {
                details.insert("name".to_string(), name.to_string());
                Some(details)
            }
            UserGroupError::NotFound { id } => {
                details.insert("id".to_string(), id.to_string());
                Some(details)
            }
            UserGroupError::CannotDeleteDisasterRecoveryCommittee { id } => {
                details.insert("id".to_string(), id.to_string());
                Some(details)
            }
            UserGroupError::IdAlreadyExists { id } => {
                details.insert("id".to_string(), id.to_string());
                Some(details)
            }
            UserGroupError::ValidationError { info } => {
                details.insert("info".to_string(), info.to_string());
                Some(details)
            }
        }
    }
}

impl From<ValidationError> for UserGroupError {
    fn from(err: ValidationError) -> Self {
        UserGroupError::ValidationError {
            info: err.to_string(),
        }
    }
}
