use crate::models::{Artifact, ArtifactId};
use orbit_essentials::storable;

/// The main index for artifacts.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ArtifactIndex {
    /// An indexed value of the artifact.
    pub index: ArtifactIndexKind,
    /// The user id, which is a UUID.
    pub artifact_id: ArtifactId,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ArtifactIndexKind {
    Hash(Vec<u8>),
    Size(u64),
}

#[derive(Clone, Debug)]
pub struct ArtifactIndexCriteria {
    pub from: ArtifactIndexKind,
    pub to: ArtifactIndexKind,
}

impl Artifact {
    pub fn to_index_by_hash(&self) -> ArtifactIndex {
        ArtifactIndex {
            index: ArtifactIndexKind::Hash(self.hash().to_vec()),
            artifact_id: *self.id(),
        }
    }

    pub fn to_index_by_size(&self) -> ArtifactIndex {
        ArtifactIndex {
            index: ArtifactIndexKind::Size(self.artifact().len() as u64),
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
        let model = ArtifactIndex {
            index: ArtifactIndexKind::Hash(vec![1, 2, 3]),
            artifact_id: [u8::MAX; 16],
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = ArtifactIndex::from_bytes(serialized_model);

        assert_eq!(model.index, deserialized_model.index);
        assert_eq!(model.artifact_id, deserialized_model.artifact_id);
    }

    #[test]
    fn valid_artifact_to_hash_index() {
        let artifact = Artifact::new(b"hello world".to_vec());
        let index = artifact.to_index_by_hash();

        assert_eq!(
            index.index,
            ArtifactIndexKind::Hash(artifact.hash().to_vec())
        );
        assert_eq!(index.artifact_id, *artifact.id());
    }

    #[test]
    fn valid_artifact_to_size_index() {
        let artifact = Artifact::new(b"hello world".to_vec());
        let index = artifact.to_index_by_size();

        assert_eq!(index.index, ArtifactIndexKind::Size(11));
        assert_eq!(index.artifact_id, *artifact.id());
    }
}
