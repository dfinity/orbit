use crate::core::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for account registration errors
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum AccountRegistrationError {
    /// The identity is already associated with another account.
    #[error(r#"Your identity is already linked to another account."#)]
    IdentityAssociatedWithAnotherAccount {
        /// The account_id of the account that the identity is associated with.
        account_id: String,
    },

    /// The generated account id already exists.
    #[error(r#"The generated account id already exists."#)]
    AccountIdAlreadyExists,
}

impl DetailableError for AccountRegistrationError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();

        match self {
            AccountRegistrationError::IdentityAssociatedWithAnotherAccount { account_id } => {
                details.insert("account_id".to_string(), account_id.to_string());

                return Some(details);
            }
            _ => {}
        }

        None
    }
}
