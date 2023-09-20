use crate::core::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for account identity repository errors
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum AccountIdentityRepositoryError {
    /// Multiple accounts are associated with the same identity, which is not allowed.
    #[error(r#"Multiple accounts are associated with the same identity"#)]
    NotAllowedMultipleAccountsWithIdentity,
}

impl DetailableError for AccountIdentityRepositoryError {
    fn details(&self) -> Option<HashMap<String, String>> {
        None
    }
}
