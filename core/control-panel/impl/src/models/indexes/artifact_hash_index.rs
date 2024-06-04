use crate::models::{Artifact, ArtifactId};
use orbit_essentials::storable;

/// The index for artifacts by hash.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ArtifactHashIndex {
    /// The hash of the artifact.
    pub hash: Vec<u8>,
    /// The user id, which is a UUID.
    pub artifact_id: ArtifactId,
}

#[derive(Clone, Debug)]
pub struct ArtifactHashIndexCriteria {
    pub hash: Vec<u8>,
}

impl Artifact {
    pub fn to_index_by_hash(&self) -> ArtifactHashIndex {
        ArtifactHashIndex {
            hash: self.hash().to_vec(),
            artifact_id: *self.id(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let model = ArtifactHashIndex {
            hash: vec![1, 2, 3],
            artifact_id: [u8::MAX; 16],
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = ArtifactHashIndex::from_bytes(serialized_model);

        assert_eq!(model.hash, deserialized_model.hash);
        assert_eq!(model.artifact_id, deserialized_model.artifact_id);
    }

    #[test]
    fn valid_artifact_to_index() {
        let artifact = Artifact::new(b"hello world".to_vec());
        let index = artifact.to_index_by_hash();

        assert_eq!(index.hash, artifact.hash().to_vec());
        assert_eq!(index.artifact_id, *artifact.id());
    }
}
