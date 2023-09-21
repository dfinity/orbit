use crate::core::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for account management errors
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum AccountManagementError {
    /// There's no account associated with the caller's identity.
    #[error(r#"There's no account associated with your identity"#)]
    NoAccountAssociatedWithCallerIdentity,
    /// The identity is already associated with another account.
    #[error(r#"Your identity is already linked to another account."#)]
    IdentityAssociatedWithAnotherAccount {
        /// The account_id of the account that the identity is associated with.
        account_id: String,
    },
    /// The generated account id already exists.
    #[error(r#"The generated account id already exists."#)]
    DuplicatedAccountId,
    /// The provided identity is associated with an account, but its not active, please activate it.
    #[error(r#"The provided identity is associated with an account, but its not active, please activate it."#)]
    IdentityPendingActivationWithinAccount {
        /// The account_id of the account that the identity is associated with.
        account_id: String,
    },
    /// The provided account is missing details.
    #[error(r#"The account requested is missing details."#)]
    MissingAccountDetails {
        /// The account_id requested.
        account_id: String,
    },
    /// The identity is already associated and active with one account.
    #[error(r#"The identity is already associated with one account."#)]
    IdentityAlreadyActivatedWithAccount {
        /// The account_id of the account that the identity is associated with.
        account_id: String,
    },
    /// The provided account id is malformed.
    #[error(r#"The provided account id is malformed."#)]
    MalformedAccountId {
        /// The account_id that was provided.
        account_id: String,
    },
    /// Some identities were not associated with the account.
    #[error(r#"Some identities were not associated with the account."#)]
    IdentityNotAssociatedWithAccount,
    /// There are too many banks associated with the account.
    #[error(r#"There are too many banks associated with the account."#)]
    TooManyBanksForAccount { max_banks: u32 },
    /// The account needs to have at least one associated identity.
    #[error(r#"The account needs to have at least one associated identity."#)]
    TooLittleAccountIdentities,
}

impl DetailableError for AccountManagementError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();

        match self {
            AccountManagementError::IdentityAssociatedWithAnotherAccount { account_id } => {
                details.insert("account_id".to_string(), account_id.to_string());

                return Some(details);
            }
            AccountManagementError::IdentityPendingActivationWithinAccount { account_id } => {
                details.insert("account_id".to_string(), account_id.to_string());

                return Some(details);
            }
            AccountManagementError::MalformedAccountId { account_id } => {
                details.insert("account_id".to_string(), account_id.to_string());

                return Some(details);
            }
            AccountManagementError::IdentityAlreadyActivatedWithAccount { account_id } => {
                details.insert("account_id".to_string(), account_id.to_string());

                return Some(details);
            }
            AccountManagementError::MissingAccountDetails { account_id } => {
                details.insert("account_id".to_string(), account_id.to_string());

                return Some(details);
            }
            AccountManagementError::TooManyBanksForAccount { max_banks } => {
                details.insert("max_banks".to_string(), max_banks.to_string());

                return Some(details);
            }
            _ => {}
        }

        None
    }
}
