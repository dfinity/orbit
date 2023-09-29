use ic_canister_core::api::DetailableError;
use std::collections::HashMap;
use thiserror::Error;

/// Container for factory errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum RepositoryError {
    /// The requested entity was not found in the repository.
    #[error(r#"The requested entity was not found in the repository."#)]
    EntityNotFound { entity: String, entity_id: String },
    /// The requested entity has too many associations.
    #[error(r#"The requested entity has too many associations."#)]
    NotAllowedMultipleAssociation { entity: String, entity_id: String },
}

impl DetailableError for RepositoryError {
    fn details(&self) -> Option<HashMap<String, String>> {
        match self {
            RepositoryError::EntityNotFound { entity, entity_id } => {
                let mut details = HashMap::new();
                details.insert("entity".to_string(), entity.to_string());
                details.insert("entity_id".to_string(), entity_id.to_string());
                Some(details)
            }
            RepositoryError::NotAllowedMultipleAssociation { entity, entity_id } => {
                let mut details = HashMap::new();
                details.insert("entity".to_string(), entity.to_string());
                details.insert("entity_id".to_string(), entity_id.to_string());
                Some(details)
            }
        }
    }
}
