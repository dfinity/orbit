use ic_canister_core::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for proposal execution errors.
#[derive(Error, Debug)]
pub enum ProposalEvaluateError {
    /// Proposal evaluation failed due to {reason}.
    #[error(r#"Proposal evaluation failed due to `{reason}`."#)]
    Failed { reason: String },
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl DetailableError for ProposalEvaluateError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            ProposalEvaluateError::Failed { reason } => {
                details.insert("reason".to_string(), reason.to_string());
                Some(details)
            }
            _ => None,
        }
    }
}
