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
