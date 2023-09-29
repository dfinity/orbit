use ic_canister_core::api::DetailableError;
use thiserror::Error;

/// Container for wallet errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum AccountError {
    /// The account must have at least one associated identity.
    #[error(r#"The account must have at least one associated identity."#)]
    TooLittleIdentities,
    /// The account has too many identities.
    #[error(r#"The account has too many identities, it cannot have more than {max_identities}."#)]
    TooManyIdentities {
        /// The maximum number of identities allowed.
        max_identities: u8,
    },
    /// The account has too many unconfirmed identities.
    #[error(r#"The account has too many unconfirmed identities, it cannot have more than {max_identities}."#)]
    TooManyUnconfirmedIdentities {
        /// The maximum number of identities allowed.
        max_identities: u8,
    },
    /// The account has too little access roles associated.
    #[error(r#"The account has too little access roles associated."#)]
    TooLittleAccessRoles,
    /// The account has too many access roles.
    #[error(
        r#"The account has too many access roles, it cannot have more than {max_access_roles}."#
    )]
    TooManyAccessRoles {
        /// The maximum number of access roles allowed.
        max_access_roles: u8,
    },
    /// The requested account identity was not found.
    #[error(r#"The requested account identity was not found."#)]
    NotFoundAccountIdentity {
        /// The requested account identity.
        identity: String,
    },
    /// The identity already has an associated account.
    #[error(r#"The identity already has an associated account."#)]
    IdentityAlreadyHasAccount {
        /// The associated account of the identity.
        account: String,
    },
    /// The requested account was not found.
    #[error(r#"The requested account was not found."#)]
    NotFoundAccount {
        /// The requested account.
        account: String,
    },
}

impl DetailableError for AccountError {
    fn details(&self) -> Option<std::collections::HashMap<String, String>> {
        let mut details = std::collections::HashMap::new();
        match self {
            AccountError::TooManyIdentities { max_identities } => {
                details.insert("max_identities".to_string(), max_identities.to_string());
                Some(details)
            }
            AccountError::TooManyUnconfirmedIdentities { max_identities } => {
                details.insert("max_identities".to_string(), max_identities.to_string());
                Some(details)
            }
            AccountError::TooManyAccessRoles { max_access_roles } => {
                details.insert("max_access_roles".to_string(), max_access_roles.to_string());
                Some(details)
            }
            AccountError::NotFoundAccountIdentity { identity } => {
                details.insert("identity".to_string(), identity.to_string());
                Some(details)
            }
            AccountError::IdentityAlreadyHasAccount { account } => {
                details.insert("account".to_string(), account.to_string());
                Some(details)
            }
            AccountError::NotFoundAccount { account } => {
                details.insert("account".to_string(), account.to_string());
                Some(details)
            }
            _ => None,
        }
    }
}
