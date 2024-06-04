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
#[query(name = "get_user")]
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
