use crate::{core::ic_cdk::next_time, errors::ArtifactError, repositories::ARTIFACT_REPOSITORY};
use orbit_essentials::{
    model::{ModelKey, ModelValidator, ModelValidatorResult},
    storable,
    types::{Timestamp, UUID},
    utils::sha256_hash,
};
use uuid::Uuid;

/// The artifact id, which is a UUID.
pub type ArtifactId = UUID;

/// The artifact is a record that is stored in the artifact repository.
#[storable]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Artifact {
    /// The UUID identifies an artifact.
    id: ArtifactId,
    /// The artifact sha256 hash.
    hash: Vec<u8>,
    /// The artifact itself.
    artifact: Vec<u8>,
    /// Stores the references to the artifact. This is used as a reference counter since there can
    /// be multiple references to the same artifact if the hash is the same.
    ///
    /// The reference counter is incremented when a new reference is created and decremented when a
    /// reference is removed. When the reference counter reaches zero, the artifact is removed.
    rc: u32,
    /// The date when the artifact was created.
    created_at: Timestamp,
}

impl ModelKey<ArtifactId> for Artifact {
    fn key(&self) -> ArtifactId {
        self.id
    }
}

#[derive(Clone, Debug)]
pub struct ArtifactCreateOpts {
    pub id: ArtifactId,
    pub created_at: Timestamp,
}

impl Default for ArtifactCreateOpts {
    fn default() -> Self {
        Self {
            id: *Uuid::new_v4().as_bytes(),
            created_at: next_time(),
        }
    }
}

impl Artifact {
    /// Creates a new artifact.
    pub fn new(artifact: Vec<u8>) -> Self {
        Self::new_with_opts(artifact, ArtifactCreateOpts::default())
    }

    /// Creates a new artifact with the given options.
    pub fn new_with_opts(artifact: Vec<u8>, opts: ArtifactCreateOpts) -> Self {
        let hash = sha256_hash(&artifact);

        Self {
            id: opts.id,
            rc: 1,
            artifact,
            hash,
            created_at: opts.created_at,
        }
    }

    /// Returns the artifact id.
    pub fn id(&self) -> &ArtifactId {
        &self.id
    }

    /// Returns the artifact hash.
    pub fn hash(&self) -> &[u8] {
        &self.hash
    }

    /// Returns the artifact.
    pub fn artifact(&self) -> &[u8] {
        &self.artifact
    }

    /// Returns the creation timestamp.
    pub fn created_at(&self) -> Timestamp {
        self.created_at
    }

    /// Returns the reference counter.
    pub fn rc(&self) -> u32 {
        self.rc
    }

    /// Increments the reference counter.
    pub fn increment_rc(&mut self) {
        self.rc = self.rc.saturating_add(1);
    }

    /// Decrements the reference counter.
    pub fn decrement_rc(&mut self) {
        self.rc = self.rc.saturating_sub(1);
    }
}

fn validate_unique_hash(self_id: &ArtifactId, hash: &[u8]) -> ModelValidatorResult<ArtifactError> {
    ARTIFACT_REPOSITORY
        .find_by_hash(hash)
        .map_or(Ok(()), |existing_id| {
            if existing_id != *self_id {
                return Err(ArtifactError::ValidationError {
                    info: "Artifact with the same hash already exists".to_string(),
                });
            }

            Ok(())
        })
}

impl ModelValidator<ArtifactError> for Artifact {
    fn validate(&self) -> ModelValidatorResult<ArtifactError> {
        validate_unique_hash(self.id(), &self.hash)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbit_essentials::repository::Repository;

    #[test]
    fn test_empty_artifact_is_accepted() {
        let artifact = Artifact::new(Vec::new());

        assert_eq!(artifact.artifact(), b"");
        assert_eq!(
            artifact.hash(),
            vec![
                227, 176, 196, 66, 152, 252, 28, 20, 154, 251, 244, 200, 153, 111, 185, 36, 39,
                174, 65, 228, 100, 155, 147, 76, 164, 149, 153, 27, 120, 82, 184, 85
            ]
        );
    }

    #[test]
    fn test_artifact_creation() {
        let artifact = Artifact::new(b"hello world".to_vec());

        assert_eq!(
            artifact.hash(),
            vec![
                185, 77, 39, 185, 147, 77, 62, 8, 165, 46, 82, 215, 218, 125, 171, 250, 196, 132,
                239, 227, 122, 83, 128, 238, 144, 136, 247, 172, 226, 239, 205, 233,
            ]
        );
        assert_eq!(artifact.artifact(), b"hello world");
    }

    #[test]
    fn artifact_with_same_hash_is_invalid() {
        let artifact1 = Artifact::new(b"hello world".to_vec());

        ARTIFACT_REPOSITORY.insert(*artifact1.id(), artifact1.clone());
        let artifact2 = Artifact::new(b"hello world".to_vec());

        assert!(artifact1.validate().is_ok());
        assert!(artifact2.validate().is_err());
    }

    #[test]
    fn artifact_with_different_hash_is_valid() {
        let artifact1 = Artifact::new(b"hello world".to_vec());

        ARTIFACT_REPOSITORY.insert(*artifact1.id(), artifact1.clone());
        let artifact2 = Artifact::new(b"hello world!".to_vec());

        assert!(artifact1.validate().is_ok());
        assert!(artifact2.validate().is_ok());
    }
}
