use orbit_essentials::api::DetailableError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RecordValidationError {
    #[error(r#"The {model_name} {id} does not exist."#)]
    NotFound { model_name: String, id: String },
}

impl DetailableError for RecordValidationError {
    fn details(&self) -> Option<std::collections::HashMap<String, String>> {
        let mut details = std::collections::HashMap::new();

        match self {
            RecordValidationError::NotFound { model_name, id } => {
                details.insert("model_name".to_string(), model_name.to_string());
                details.insert("id".to_string(), id.to_string());
                Some(details)
            }
        }
    }
}
