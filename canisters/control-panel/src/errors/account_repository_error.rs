use std::collections::HashMap;

use thiserror::Error;

use crate::core::DetailableError;

/// Container for account repository errors
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum AccountRepositoryError {
    /// Multiple accounts are associated with the same id, which is not allowed.
    #[error(r#"Multiple accounts are associated with the same id"#)]
    NotAllowedMultipleAccountsWithSameId,
}

impl DetailableError for AccountRepositoryError {
    fn details(&self) -> Option<HashMap<String, String>> {
        None
    }
}
