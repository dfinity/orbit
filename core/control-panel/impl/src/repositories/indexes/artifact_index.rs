use crate::{
    core::{with_memory_manager, Memory, ARTIFACT_INDEX_MEMORY_ID},
    models::{
        indexes::artifact_index::{ArtifactIndex, ArtifactIndexCriteria},
        ArtifactId,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::repository::IndexRepository;
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<ArtifactIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(ARTIFACT_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that holds the artifact indexes, which is used to find artifacts efficiently.
#[derive(Default, Debug)]
pub struct ArtifactIndexRepository {}

impl ArtifactIndexRepository {
    /// Clears all entries in the index repository.
    pub fn clear(&self) {
        DB.with(|m| m.borrow_mut().clear_new());
    }
}

impl IndexRepository<ArtifactIndex, ArtifactId> for ArtifactIndexRepository {
    type FindByCriteria = ArtifactIndexCriteria;

    fn exists(&self, index: &ArtifactIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: ArtifactIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &ArtifactIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<ArtifactId> {
        DB.with(|db| {
            let start_key = ArtifactIndex {
                index: criteria.from,
                artifact_id: [u8::MIN; 16],
            };
            let end_key = ArtifactIndex {
                index: criteria.to,
                artifact_id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.artifact_id)
                .collect::<HashSet<ArtifactId>>()
        })
    }
}

impl ArtifactIndexRepository {
    pub fn len(&self) -> u64 {
        DB.with(|m| m.borrow().len())
    }

    pub fn is_empty(&self) -> bool {
        DB.with(|m| m.borrow().is_empty())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        models::indexes::artifact_index::{
            ArtifactIndex, ArtifactIndexCriteria, ArtifactIndexKind,
        },
        repositories::indexes::artifact_index::ArtifactIndexRepository,
    };
    use orbit_essentials::repository::IndexRepository;

    #[test]
    fn test_index_repository() {
        let repository = ArtifactIndexRepository::default();
        let index = ArtifactIndex {
            index: ArtifactIndexKind::Hash(vec![1, 2, 3]),
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
        let repository = ArtifactIndexRepository::default();
        for i in 0..10 {
            repository.insert(ArtifactIndex {
                index: ArtifactIndexKind::Hash(vec![i]),
                artifact_id: [i; 16],
            });
        }

        let result = repository.find_by_criteria(ArtifactIndexCriteria {
            from: ArtifactIndexKind::Hash(vec![5]),
            to: ArtifactIndexKind::Hash(vec![5]),
        });

        assert!(!result.is_empty());
        assert_eq!(result.len(), 1);
        assert!(result.contains(&[5; 16]));
    }
}
