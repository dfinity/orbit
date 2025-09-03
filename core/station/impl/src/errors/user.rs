use orbit_essentials::api::DetailableError;
use thiserror::Error;

use super::ValidationError;

/// Container for user errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum UserError {
    /// Identity not allowed to be added to the user.
    #[error(r#"Identity not allowed to be added to the user."#)]
    IdentityNotAllowed { identity: String },
    /// The user has too many unconfirmed identities.
    #[error(r#"The user has too many unconfirmed identities, it cannot have more than {max_identities}."#)]
    TooManyUnconfirmedIdentities {
        /// The maximum number of identities allowed.
        max_identities: u8,
    },
    /// The requested user identity was not found.
    #[error(r#"The requested user identity was not found."#)]
    NotFoundUserIdentity {
        /// The requested user identity.
        identity: String,
    },
    /// The identity already has an associated user.
    #[error(r#"The identity already has an associated user."#)]
    IdentityAlreadyHasUser {
        /// The associated user of the identity.
        user: String,
    },
    // The name already has an associated user.
    #[error(r#"The name already has an associated user."#)]
    NameAlreadyHasUser {
        /// The associated user of the name.
        user: String,
    },
    /// The requested user was not found.
    #[error(r#"The requested user was not found."#)]
    NotFoundUser {
        /// The requested user.
        user: String,
    },
    /// You don't have permission to access the requested user.
    #[error(r#"You don't have permission to access the requested user."#)]
    Forbidden {
        /// The requested user.
        user: String,
    },
    /// Cannot remove the admin role from the caller identity.
    #[error(r#"Cannot remove own admin role."#)]
    CannotRemoveOwnAdminRole,
    /// Invalid user list limit.
    #[error(r#"Invalid user list limit, it cannot be more than {max}."#)]
    InvalidUserListLimit { max: u16 },
    // Error for when non existent user group is getting added
    #[error(r#"The user group {group_id} does not exist."#)]
    UserGroupDoesNotExist { group_id: String },
    // Error for when a user with the same id already exists
    #[error(r#"The user {user_id} already exists."#)]
    IdAlreadyExists { user_id: String },
    /// The user has failed validation.
    #[error(r#"The user has failed validation."#)]
    ValidationError { info: String },
}

impl DetailableError for UserError {
    fn details(&self) -> Option<std::collections::HashMap<String, String>> {
        let mut details = std::collections::HashMap::new();
        match self {
            UserError::TooManyUnconfirmedIdentities { max_identities } => {
                details.insert("max_identities".to_string(), max_identities.to_string());
                Some(details)
            }
            UserError::NotFoundUserIdentity { identity } => {
                details.insert("identity".to_string(), identity.to_string());
                Some(details)
            }
            UserError::IdentityAlreadyHasUser { user } => {
                details.insert("user".to_string(), user.to_string());
                Some(details)
            }
            UserError::NotFoundUser { user } => {
                details.insert("user".to_string(), user.to_string());
                Some(details)
            }
            UserError::Forbidden { user } => {
                details.insert("user".to_string(), user.to_string());
                Some(details)
            }
            UserError::InvalidUserListLimit { max } => {
                details.insert("max".to_string(), max.to_string());
                Some(details)
            }
            UserError::IdentityNotAllowed { identity } => {
                details.insert("identity".to_string(), identity.to_string());
                Some(details)
            }
            UserError::NameAlreadyHasUser { user } => {
                details.insert("user".to_string(), user.to_string());
                Some(details)
            }
            UserError::IdAlreadyExists { user_id } => {
                details.insert("user_id".to_string(), user_id.to_string());
                Some(details)
            }
            UserError::ValidationError { info } => {
                details.insert("info".to_string(), info.to_string());
                Some(details)
            }
            UserError::UserGroupDoesNotExist { group_id } => {
                details.insert("group_id".to_string(), group_id.to_string());
                Some(details)
            }
            UserError::CannotRemoveOwnAdminRole => Some(details),
        }
    }
}

impl From<ValidationError> for UserError {
    fn from(err: ValidationError) -> Self {
        UserError::ValidationError {
            info: err.to_string(),
        }
    }
}
