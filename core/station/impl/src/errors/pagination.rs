use orbit_essentials::api::DetailableError;
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum PaginationError {
    /// Invalid max limit.
    #[error(r#"Invalid list limit, it cannot be more than {max}."#)]
    MaxLimitExceeded { max: u16 },
}

impl DetailableError for PaginationError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            PaginationError::MaxLimitExceeded { max } => {
                details.insert("max".to_string(), max.to_string());
                Some(details)
            }
        }
    }
}
