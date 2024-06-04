use crate::{
    core::{with_memory_manager, Memory, ARTIFACT_HASH_INDEX_MEMORY_ID},
    models::{
        indexes::artifact_hash_index::{ArtifactHashIndex, ArtifactHashIndexCriteria},
        ArtifactId,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::repository::IndexRepository;
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<ArtifactHashIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(ARTIFACT_HASH_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that holds the artifact hash index, which is used to find artifacts by hash.
#[derive(Default, Debug)]
pub struct ArtifactHashIndexRepository {}

impl IndexRepository<ArtifactHashIndex, ArtifactId> for ArtifactHashIndexRepository {
    type FindByCriteria = ArtifactHashIndexCriteria;

    fn exists(&self, index: &ArtifactHashIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: ArtifactHashIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &ArtifactHashIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<ArtifactId> {
        DB.with(|db| {
            let start_key = ArtifactHashIndex {
                hash: criteria.hash.clone(),
                artifact_id: [u8::MIN; 16],
            };
            let end_key = ArtifactHashIndex {
                hash: criteria.hash,
                artifact_id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.artifact_id)
                .collect::<HashSet<ArtifactId>>()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        models::indexes::artifact_hash_index::{ArtifactHashIndex, ArtifactHashIndexCriteria},
        repositories::indexes::artifact_hash_index::ArtifactHashIndexRepository,
    };
    use orbit_essentials::repository::IndexRepository;

    #[test]
    fn test_index_repository() {
        let repository = ArtifactHashIndexRepository::default();
        let index = ArtifactHashIndex {
            hash: vec![1, 2, 3],
            artifact_id: [1; 16],
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_hash() {
        let repository = ArtifactHashIndexRepository::default();
        for i in 0..10 {
            repository.insert(ArtifactHashIndex {
                hash: vec![i],
                artifact_id: [i; 16],
            });
        }

        let result = repository.find_by_criteria(ArtifactHashIndexCriteria { hash: vec![5] });

        assert!(!result.is_empty());
        assert_eq!(result.len(), 1);
        assert!(result.contains(&[5; 16]));
    }
}
