use ic_canister_core::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for proposal execution errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum ProposalExecuteError {
    /// Proposal execution failed due to {reason}.
    #[error(r#"Proposal execution failed due to `{reason}`."#)]
    Failed { reason: String },
    /// Proposal can't be executed because it was not adopted.
    #[error(r#"Proposal can't be executed because it was not adopted."#)]
    NotAdopted,
}

impl DetailableError for ProposalExecuteError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            ProposalExecuteError::Failed { reason } => {
                details.insert("reason".to_string(), reason.to_string());
                Some(details)
            }
            _ => None,
        }
    }
}
