use crate::{
    mappers::HelperMapper,
    services::{ArtifactService, ARTIFACT_SERVICE},
};
use control_panel_api::{GetArtifactInput, GetArtifactResponse};
use ic_cdk_macros::query;
use lazy_static::lazy_static;
use orbit_essentials::api::ApiResult;
use std::sync::Arc;

// Canister entrypoints for the controller.
#[query(name = "get_artifact")]
async fn get_artifact(input: GetArtifactInput) -> ApiResult<GetArtifactResponse> {
    CONTROLLER.get_artifact(input).await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: ArtifactController =
        ArtifactController::new(Arc::clone(&ARTIFACT_SERVICE));
}

#[derive(Debug)]
pub struct ArtifactController {
    artifact_service: Arc<ArtifactService>,
}

impl ArtifactController {
    pub fn new(artifact_service: Arc<ArtifactService>) -> Self {
        Self { artifact_service }
    }

    /// Returns the artifact with the given id if it exists.
    pub async fn get_artifact(&self, input: GetArtifactInput) -> ApiResult<GetArtifactResponse> {
        let artifact = self.artifact_service.find_by_id(
            HelperMapper::to_uuid(input.artifact_id)
                .expect("Invalid artifact id")
                .as_bytes(),
        )?;

        Ok(GetArtifactResponse {
            artifact: artifact.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use control_panel_api::GetArtifactInput;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_artifact() {
        let artifact_data = b"hello world";
        let artifact_id = ARTIFACT_SERVICE.create(artifact_data.to_vec()).unwrap();

        let response = CONTROLLER
            .get_artifact(GetArtifactInput {
                artifact_id: Uuid::from_bytes(artifact_id).to_string(),
            })
            .await
            .unwrap();

        assert_eq!(
            response.artifact.id,
            Uuid::from_bytes(artifact_id).to_string()
        );
        assert_eq!(response.artifact.size, artifact_data.len() as u64);
        assert_eq!(response.artifact.artifact, artifact_data);
    }

    #[tokio::test]
    async fn test_get_artifact_invalid_id() {
        let response = CONTROLLER
            .get_artifact(GetArtifactInput {
                artifact_id: Uuid::new_v4().to_string(),
            })
            .await;

        assert!(response.is_err());
    }
}
