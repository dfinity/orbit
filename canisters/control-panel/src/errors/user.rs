use ic_canister_core::api::DetailableError;
use thiserror::Error;

/// Container for wallet errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum UserError {
    /// The identity already has an associated user.
    #[error(r#"The identity already has an associated user."#)]
    IdentityAlreadyHasUser {
        /// The associated user of the identity.
        user: String,
    },
    /// An user associated with the identity was not found.
    #[error(r#"A user associated with the identity was not found."#)]
    AssociatedUserIdentityNotFound {
        /// The given identity.
        identity: String,
    },
    /// The requested user was not found.
    #[error(r#"The requested user was not found."#)]
    NotFound {
        /// The requested user.
        user: String,
    },
    /// You don't have permission to access the requested user.
    #[error(r#"You don't have permission to access the requested user."#)]
    Forbidden {
        /// The requested user.
        user: String,
    },
    /// The user has failed validation.
    #[error(r#"The user has failed validation."#)]
    ValidationError { info: String },
    /// Removing the caller identity would lock the user.
    #[error(r#"Removing the caller identity would lock the user."#)]
    SelfLocked,
    /// The main bank associated with the user was not found.
    #[error(r#"The main bank associated with the user was not found."#)]
    MainBankNotFound,
}

impl DetailableError for UserError {
    fn details(&self) -> Option<std::collections::HashMap<String, String>> {
        let mut details = std::collections::HashMap::new();
        match self {
            UserError::IdentityAlreadyHasUser { user } => {
                details.insert("user".to_string(), user.to_string());
                Some(details)
            }
            UserError::NotFound { user } => {
                details.insert("user".to_string(), user.to_string());
                Some(details)
            }
            UserError::Forbidden { user } => {
                details.insert("user".to_string(), user.to_string());
                Some(details)
            }
            UserError::ValidationError { info } => {
                details.insert("info".to_string(), info.to_string());
                Some(details)
            }
            UserError::AssociatedUserIdentityNotFound { identity } => {
                details.insert("identity".to_string(), identity.to_string());
                Some(details)
            }
            _ => None,
        }
    }
}
