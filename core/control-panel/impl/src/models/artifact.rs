use crate::core::ic_cdk::next_time;
use orbit_essentials::{
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
    /// The artifact byte size.
    size: u64,
    /// The artifact sha256 hash.
    hash: Vec<u8>,
    /// The artifact itself.
    artifact: Vec<u8>,
    /// The date when the artifact was created.
    created_at: Timestamp,
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
        let size = artifact.len() as u64;

        Self {
            id: opts.id,
            artifact,
            hash,
            size,
            created_at: opts.created_at,
        }
    }

    /// Returns the artifact id.
    pub fn id(&self) -> &ArtifactId {
        &self.id
    }

    /// Returns the artifact size.
    pub fn size(&self) -> u64 {
        self.size
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artifact_creation() {
        let artifact = Artifact::new(b"hello world".to_vec());

        assert_eq!(artifact.size(), 11);
        assert_eq!(
            artifact.hash(),
            vec![
                185, 77, 39, 185, 147, 77, 62, 8, 165, 46, 82, 215, 218, 125, 171, 250, 196, 132,
                239, 227, 122, 83, 128, 238, 144, 136, 247, 172, 226, 239, 205, 233,
            ]
        );
        assert_eq!(artifact.artifact(), b"hello world");
    }
}
