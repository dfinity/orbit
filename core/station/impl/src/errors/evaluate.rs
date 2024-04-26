use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for evaluation errors.
#[derive(Error, Debug)]
pub enum EvaluateError {
    /// Evaluation failed due to {reason}.
    #[error(r#"Evaluation failed due to `{reason}`."#)]
    Failed { reason: String },
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl DetailableError for EvaluateError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            EvaluateError::Failed { reason } => {
                details.insert("reason".to_string(), reason.to_string());
                Some(details)
            }
            _ => None,
        }
    }
}
