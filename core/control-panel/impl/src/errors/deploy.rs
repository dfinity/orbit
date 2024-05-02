use orbit_essentials::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for deployment errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum DeployError {
    /// The deployment of the station canister failed.
    #[error(r#"The deployment of the station canister failed due to `{reason}`"#)]
    Failed { reason: String },
}

impl DetailableError for DeployError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        let DeployError::Failed { reason } = self;
        details.insert("reason".to_string(), reason.to_string());
        Some(details)
    }
}
