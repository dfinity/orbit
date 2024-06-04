use super::indexes::artifact_hash_index::ArtifactHashIndexRepository;
use crate::{
    core::{with_memory_manager, Memory, ARTIFACT_MEMORY_ID},
    models::{indexes::artifact_hash_index::ArtifactHashIndexCriteria, Artifact, ArtifactId},
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use orbit_essentials::repository::RefreshIndexMode;
use orbit_essentials::repository::{IndexRepository, Repository};
use std::{cell::RefCell, sync::Arc};

thread_local! {
  static DB: RefCell<StableBTreeMap<ArtifactId, Artifact, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(ARTIFACT_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref ARTIFACT_REPOSITORY: Arc<ArtifactRepository> =
        Arc::new(ArtifactRepository::default());
}

/// A repository that enables managing artifacts in stable memory.
#[derive(Default, Debug)]
pub struct ArtifactRepository {
    hash_index: ArtifactHashIndexRepository,
}

impl Repository<ArtifactId, Artifact> for ArtifactRepository {
    fn list(&self) -> Vec<Artifact> {
        DB.with(|m| m.borrow().iter().map(|(_, v)| v).collect())
    }

    fn get(&self, key: &ArtifactId) -> Option<Artifact> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: ArtifactId, value: Artifact) -> Option<Artifact> {
        DB.with(|m| {
            let prev = m.borrow_mut().insert(key, value.clone());

            self.hash_index
                .refresh_index_on_modification(RefreshIndexMode::List {
                    previous: prev
                        .clone()
                        .map_or(Vec::new(), |prev| vec![prev.to_index_by_hash()]),
                    current: vec![value.to_index_by_hash()],
                });

            prev
        })
    }

    fn remove(&self, key: &ArtifactId) -> Option<Artifact> {
        DB.with(|m| {
            let prev = m.borrow_mut().remove(key);

            self.hash_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupList {
                    current: prev
                        .clone()
                        .map_or(Vec::new(), |prev| vec![prev.to_index_by_hash()]),
                });

            prev
        })
    }

    fn len(&self) -> usize {
        DB.with(|m| m.borrow().len()) as usize
    }
}

impl ArtifactRepository {
    /// Finds an artifact by the given hash.
    pub fn find_by_hash(&self, hash: &[u8]) -> Option<ArtifactId> {
        let artifacts = self.hash_index.find_by_criteria(ArtifactHashIndexCriteria {
            hash: hash.to_vec(),
        });

        artifacts.into_iter().next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_crud_operations() {
        let repository = ArtifactRepository::default();
        let artifact = Artifact::new(b"hello world".to_vec());

        assert!(repository.get(artifact.id()).is_none());

        repository.insert(*artifact.id(), artifact.clone());
        assert_eq!(repository.len(), 1);
        assert_eq!(repository.get(artifact.id()), Some(artifact.clone()));

        repository.remove(artifact.id());

        assert!(repository.get(artifact.id()).is_none());

        assert_eq!(repository.len(), 0);
    }

    #[test]
    fn test_find_by_hash() {
        let repository = ArtifactRepository::default();
        let artifact = Artifact::new(b"hello world".to_vec());

        repository.insert(*artifact.id(), artifact.clone());

        assert_eq!(
            repository.find_by_hash(artifact.hash()),
            Some(*artifact.id())
        );
    }
}
