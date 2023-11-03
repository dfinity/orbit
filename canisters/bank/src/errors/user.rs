use ic_canister_core::api::DetailableError;
use thiserror::Error;

/// Container for wallet errors.
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
    /// The user has too many unconfirmed identities.
    #[error(r#"The user has too many unconfirmed identities, it cannot have more than {max_identities}."#)]
    TooManyUnconfirmedIdentities {
        /// The maximum number of identities allowed.
        max_identities: u8,
    },
    /// The user has too little access roles associated.
    #[error(r#"The user has too little access roles associated."#)]
    TooLittleAccessRoles,
    /// The user has too many access roles.
    #[error(r#"The user has too many access roles, it cannot have more than {max_access_roles}."#)]
    TooManyAccessRoles {
        /// The maximum number of access roles allowed.
        max_access_roles: u8,
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
            UserError::TooManyAccessRoles { max_access_roles } => {
                details.insert("max_access_roles".to_string(), max_access_roles.to_string());
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
            _ => None,
        }
    }
}
