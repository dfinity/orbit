use crate::models::Artifact;
use orbit_essentials::utils::timestamp_to_rfc3339;
use uuid::Uuid;

impl From<Artifact> for control_panel_api::ArtifactDTO {
    fn from(model: Artifact) -> Self {
        control_panel_api::ArtifactDTO {
            id: Uuid::from_bytes(*model.id()).to_string(),
            size: model.artifact().len() as u64,
            hash: hex::encode(model.hash()),
            artifact: model.artifact().to_vec(),
            created_at: timestamp_to_rfc3339(&model.created_at()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_model_conversion() {
        let model = Artifact::new(b"hello world".to_vec());
        let dto = control_panel_api::ArtifactDTO::from(model.clone());

        assert_eq!(dto.id, Uuid::from_bytes(*model.id()).to_string());
        assert_eq!(dto.size, model.artifact().len() as u64);
        assert_eq!(
            dto.hash,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9".to_string()
        );
        assert_eq!(dto.artifact, model.artifact());
        assert_eq!(dto.created_at, timestamp_to_rfc3339(&model.created_at()));
    }
}
