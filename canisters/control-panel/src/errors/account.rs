use ic_canister_core::api::DetailableError;
use thiserror::Error;

/// Container for wallet errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum AccountError {
    /// The identity already has an associated account.
    #[error(r#"The identity already has an associated account."#)]
    IdentityAlreadyHasAccount {
        /// The associated account of the identity.
        account: String,
    },
    /// An account associated with the identity was not found.
    #[error(r#"An account associated with the identity was not found."#)]
    AssociatedAccountIdentityNotFound {
        /// The given identity.
        identity: String,
    },
    /// The requested account was not found.
    #[error(r#"The requested account was not found."#)]
    NotFound {
        /// The requested account.
        account: String,
    },
    /// You don't have permission to access the requested account.
    #[error(r#"You don't have permission to access the requested account."#)]
    Forbidden {
        /// The requested account.
        account: String,
    },
    /// The account has failed validation.
    #[error(r#"The account has failed validation."#)]
    ValidationError { info: String },
    /// Removing the caller identity would lock the account.
    #[error(r#"Removing the caller identity would lock the account."#)]
    SelfLocked,
    /// The main bank associated with the account was not found.
    #[error(r#"The main bank associated with the account was not found."#)]
    MainBankNotFound,
}

impl DetailableError for AccountError {
    fn details(&self) -> Option<std::collections::HashMap<String, String>> {
        let mut details = std::collections::HashMap::new();
        match self {
            AccountError::IdentityAlreadyHasAccount { account } => {
                details.insert("account".to_string(), account.to_string());
                Some(details)
            }
            AccountError::NotFound { account } => {
                details.insert("account".to_string(), account.to_string());
                Some(details)
            }
            AccountError::Forbidden { account } => {
                details.insert("account".to_string(), account.to_string());
                Some(details)
            }
            AccountError::ValidationError { info } => {
                details.insert("info".to_string(), info.to_string());
                Some(details)
            }
            AccountError::AssociatedAccountIdentityNotFound { identity } => {
                details.insert("identity".to_string(), identity.to_string());
                Some(details)
            }
            _ => None,
        }
    }
}
