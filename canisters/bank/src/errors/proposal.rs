use ic_canister_core::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for system proposal errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum ProposalError {
    /// The requested system proposal was not found.
    #[error(r#"The requested system proposal was not found."#)]
    NotFound { proposal_id: String },
    /// You don't have access to the requested resource.
    #[error(r#"You don't have access to the requested resource."#)]
    Forbidden { proposal_id: String },
    /// Proposals that have already been completed cannot be modified.
    #[error(r#"This proposal was already completed, it cannot be modified."#)]
    NotAllowedModification { proposal_id: String },
    /// The reason for the proposal vote status is too long.
    #[error(r#"The reason for the proposal status is too long."#)]
    VoteReasonTooLong { max_len: u8 },
    /// The proposal has failed validation.
    #[error(r#"The proposal has failed validation."#)]
    ValidationError { info: String },
}

impl DetailableError for ProposalError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            ProposalError::NotFound { proposal_id } => {
                details.insert("proposal_id".to_string(), proposal_id.to_string());
                Some(details)
            }
            ProposalError::Forbidden { proposal_id } => {
                details.insert("proposal_id".to_string(), proposal_id.to_string());
                Some(details)
            }
            ProposalError::NotAllowedModification { proposal_id } => {
                details.insert("proposal_id".to_string(), proposal_id.to_string());
                Some(details)
            }
            ProposalError::VoteReasonTooLong { max_len } => {
                details.insert("max_len".to_string(), max_len.to_string());
                Some(details)
            }
            ProposalError::ValidationError { info } => {
                details.insert("info".to_string(), info.to_string());
                Some(details)
            }
        }
    }
}
