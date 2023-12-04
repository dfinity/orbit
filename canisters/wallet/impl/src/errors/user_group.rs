use ic_canister_core::api::DetailableError;
use thiserror::Error;

/// Container for user group errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum UserGroupError {
    /// The user group name is too long, it cannot have more than {max_length}.
    #[error(r#"The user group name is too long, it cannot have more than {max_length}."#)]
    NameTooLong {
        /// The maximum length allowed.
        max_length: u8,
    },
    /// The user group name is too short, it cannot have more than {max_length}.
    #[error(r#"The user group name is too short, it cannot be less than {min_length}."#)]
    NameTooShort {
        /// The minimum length allowed.
        min_length: u8,
    },
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
}

impl DetailableError for UserGroupError {
    fn details(&self) -> Option<std::collections::HashMap<String, String>> {
        let mut details = std::collections::HashMap::new();
        match self {
            UserGroupError::NameTooLong { max_length } => {
                details.insert("max_length".to_string(), max_length.to_string());
                Some(details)
            }
            UserGroupError::NameTooShort { min_length } => {
                details.insert("min_length".to_string(), min_length.to_string());
                Some(details)
            }
            UserGroupError::NonUniqueName { name } => {
                details.insert("name".to_string(), name.to_string());
                Some(details)
            }
            UserGroupError::NotFound { id } => {
                details.insert("id".to_string(), id.to_string());
                Some(details)
            }
        }
    }
}
