use crate::{
    errors::ArtifactError,
    models::{Artifact, ArtifactId},
    repositories::{ArtifactRepository, ARTIFACT_REPOSITORY},
};
use lazy_static::lazy_static;
use orbit_essentials::repository::Repository;
use orbit_essentials::{api::ServiceResult, model::ModelValidator};
use std::sync::Arc;
use uuid::Uuid;

lazy_static! {
    pub static ref ARTIFACT_SERVICE: Arc<ArtifactService> =
        Arc::new(ArtifactService::new(Arc::clone(&ARTIFACT_REPOSITORY)));
}

/// The artifact service provides methods to manage artifacts.
#[derive(Default, Debug)]
pub struct ArtifactService {
    artifact_repository: Arc<ArtifactRepository>,
}

impl ArtifactService {
    pub fn new(artifact_repository: Arc<ArtifactRepository>) -> Self {
        Self {
            artifact_repository,
        }
    }

    /// Creates a new artifact with the given artifact bytes.
    ///
    /// If the artifact already exists, the existing artifact id is returned.
    pub fn create(&self, artifact: Vec<u8>) -> ServiceResult<ArtifactId> {
        let artifact = Artifact::new(artifact);

        match self.artifact_repository.find_by_hash(artifact.hash()) {
            Some(artifact_id) => {
                let mut artifact = self.find_by_id(&artifact_id)?;

                artifact.increment_rc();

                self.artifact_repository
                    .insert(*artifact.id(), artifact.clone());

                Ok(artifact_id)
            }
            None => {
                let artifact_id = artifact.id();

                artifact.validate()?;

                self.artifact_repository
                    .insert(*artifact_id, artifact.clone());

                Ok(*artifact_id)
            }
        }
    }

    /// Finds an artifact by its id.
    ///
    /// If the artifact does not exist, an error is returned.
    pub fn find_by_id(&self, artifact_id: &ArtifactId) -> ServiceResult<Artifact> {
        let artifact =
            self.artifact_repository
                .get(artifact_id)
                .ok_or(ArtifactError::NotFound {
                    id: Uuid::from_bytes(*artifact_id).to_string(),
                })?;

        Ok(artifact)
    }

    /// Removes an artifact by its id.
    ///
    /// The artifact is only removed if all references to it are removed.
    pub fn remove_by_id(&self, artifact_id: &ArtifactId) -> ServiceResult<()> {
        let mut artifact = self.find_by_id(artifact_id)?;

        artifact.decrement_rc();

        if artifact.rc() > 0 {
            self.artifact_repository
                .insert(*artifact_id, artifact.clone());

            return Ok(());
        }

        self.artifact_repository.remove(artifact_id);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbit_essentials::api::ApiError;

    #[test]
    fn test_create() {
        let artifact = vec![1, 2, 3];
        let artifact_id = ARTIFACT_SERVICE.create(artifact.clone()).unwrap();
        let found_artifact = ARTIFACT_SERVICE.find_by_id(&artifact_id).unwrap();

        assert_eq!(found_artifact.id(), &artifact_id);
        assert_eq!(found_artifact.artifact(), &artifact);
    }

    #[test]
    fn test_create_same_resolves_to_same_id() {
        let artifact = vec![1, 2, 3];
        let artifact_id = ARTIFACT_SERVICE.create(artifact.clone()).unwrap();
        let same_artifact_id = ARTIFACT_SERVICE.create(artifact.clone()).unwrap();

        assert_eq!(artifact_id, same_artifact_id);
    }

    #[test]
    fn test_create_same_increment_rc() {
        let artifact = vec![1, 2, 3];
        let artifact_id = ARTIFACT_SERVICE.create(artifact.clone()).unwrap();
        let _ = ARTIFACT_SERVICE.create(artifact.clone()).unwrap();
        let found_artifact = ARTIFACT_SERVICE.find_by_id(&artifact_id).unwrap();

        assert_eq!(found_artifact.rc(), 2);
    }

    #[test]
    fn test_remove_by_id() {
        let artifact = vec![1, 2, 3];
        let artifact_id = ARTIFACT_SERVICE.create(artifact.clone()).unwrap();

        ARTIFACT_SERVICE.remove_by_id(&artifact_id).unwrap();

        let result = ARTIFACT_SERVICE.find_by_id(&artifact_id);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ApiError::from(ArtifactError::NotFound {
                id: Uuid::from_bytes(artifact_id).to_string()
            })
        );
    }

    #[test]
    fn test_remove_by_id_with_references() {
        let artifact = vec![1, 2, 3];
        let artifact_id = ARTIFACT_SERVICE.create(artifact.clone()).unwrap();
        let _ = ARTIFACT_SERVICE.create(artifact.clone()).unwrap();

        ARTIFACT_SERVICE.remove_by_id(&artifact_id).unwrap();

        let found_artifact = ARTIFACT_SERVICE.find_by_id(&artifact_id).unwrap();

        assert_eq!(found_artifact.rc(), 1);

        ARTIFACT_SERVICE.remove_by_id(&artifact_id).unwrap();

        let result = ARTIFACT_SERVICE.find_by_id(&artifact_id);

        assert!(result.is_err());
    }
}
