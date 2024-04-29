use orbit_essentials::api::DetailableError;
use thiserror::Error;

/// Container for user errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum UserError {
    /// The user must have at least one associated identity.
    #[error(r#"The user must have at least one associated identity."#)]
    TooLittleIdentities,
    /// The user has too many identities.
    #[error(r#"The user has too many identities, it cannot have more than {max_identities}."#)]
    TooManyIdentities {
        /// The maximum number of identities allowed.
        max_identities: u8,
    },
    /// Identity not allowed to be added to the user.
    #[error(r#"Identity not allowed to be added to the user."#)]
    IdentityNotAllowed { identity: String },
    /// The user has too many unconfirmed identities.
    #[error(r#"The user has too many unconfirmed identities, it cannot have more than {max_identities}."#)]
    TooManyUnconfirmedIdentities {
        /// The maximum number of identities allowed.
        max_identities: u8,
    },
    /// The user has too many user groups, it cannot have more than {max}.
    #[error(r#"The user has too many user groups, it cannot have more than {max}."#)]
    TooManyUserGroups {
        /// The maximum number of access roles allowed.
        max: u8,
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
    /// Removing the caller identity would lock the user.
    #[error(r#"Removing the caller identity would lock the user."#)]
    SelfLocked,
    /// Cannot remove the admin role from the caller identity.
    #[error(r#"Cannot remove own admin role."#)]
    CannotRemoveOwnAdminRole,
    /// Name is too long.
    #[error(r#"Name is too long, it cannot have more than {max_length}."#)]
    NameTooLong {
        /// The maximum length of the name.
        max_length: usize,
    },
    #[error(r#"You're not authorized to perform this action."#)]
    Unauthorized,
    /// Invalid user list limit.
    #[error(r#"Invalid user list limit, it cannot be more than {max}."#)]
    InvalidUserListLimit { max: u16 },

    // error for when non existent user group is getting added
    #[error(r#"The user group {group_id} does not exist."#)]
    UserGroupDoesNotExist { group_id: String },
}

impl DetailableError for UserError {
    fn details(&self) -> Option<std::collections::HashMap<String, String>> {
        let mut details = std::collections::HashMap::new();
        match self {
            UserError::TooManyIdentities { max_identities } => {
                details.insert("max_identities".to_string(), max_identities.to_string());
                Some(details)
            }
            UserError::TooManyUnconfirmedIdentities { max_identities } => {
                details.insert("max_identities".to_string(), max_identities.to_string());
                Some(details)
            }
            UserError::TooManyUserGroups { max } => {
                details.insert("max".to_string(), max.to_string());
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
            UserError::NameTooLong { max_length } => {
                details.insert("max_length".to_string(), max_length.to_string());
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
            _ => None,
        }
    }
}
