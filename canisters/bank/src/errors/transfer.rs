use ic_canister_core::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for transfer errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum TransferError {
    #[error(r#"The requested transfer was not found."#)]
    TransferNotFound { transfer_id: String },
}

impl DetailableError for TransferError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            TransferError::TransferNotFound { transfer_id } => {
                details.insert("transfer_id".to_string(), transfer_id.to_string());
                Some(details)
            }
        }
    }
}
