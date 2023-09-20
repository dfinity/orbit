use crate::core::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for account identity repository errors
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum AccountBankRepositoryError {}

impl DetailableError for AccountBankRepositoryError {
    fn details(&self) -> Option<HashMap<String, String>> {
        None
    }
}
