use crate::{
    core::{with_memory_manager, Memory, PROPOSAL_SORT_INDEX_MEMORY_ID},
    models::indexes::proposal_sort_index::{
        ProposalSortIndex, ProposalSortIndexCriteria, ProposalSortIndexKey, ProposalSortIndexValue,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::repository::IndexRepository;
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<ProposalSortIndexKey, ProposalSortIndexValue, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(PROPOSAL_SORT_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables finding proposals based on the voter in stable memory.
#[derive(Default, Debug)]
pub struct ProposalSortIndexRepository {}

impl IndexRepository<ProposalSortIndex, ProposalSortIndexValue> for ProposalSortIndexRepository {
    type FindByCriteria = ProposalSortIndexCriteria;

    fn exists(&self, index: &ProposalSortIndex) -> bool {
        DB.with(|m| m.borrow().contains_key(&index.key))
    }

    fn insert(&self, index: ProposalSortIndex) {
        DB.with(|m| m.borrow_mut().insert(index.key, index.value));
    }

    fn remove(&self, index: &ProposalSortIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(&index.key).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<ProposalSortIndexValue> {
        let value = self.get(&ProposalSortIndexKey {
            proposal_id: criteria.proposal_id,
        });

        match value {
            Some(value) => {
                let mut set = HashSet::new();
                set.insert(value);
                set
            }
            None => HashSet::new(),
        }
    }
}

impl ProposalSortIndexRepository {
    pub fn get(&self, key: &ProposalSortIndexKey) -> Option<ProposalSortIndexValue> {
        DB.with(|m| m.borrow().get(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_crud() {
        let repository = ProposalSortIndexRepository::default();
        let index = ProposalSortIndex {
            key: ProposalSortIndexKey {
                proposal_id: [0; 16],
            },
            value: ProposalSortIndexValue {
                creation_timestamp: 1,
                modification_timestamp: 2,
                expiration_timestamp: 3,
            },
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_criteria() {
        let repository = ProposalSortIndexRepository::default();
        let index = ProposalSortIndex {
            key: ProposalSortIndexKey {
                proposal_id: [0; 16],
            },
            value: ProposalSortIndexValue {
                creation_timestamp: 1,
                modification_timestamp: 2,
                expiration_timestamp: 3,
            },
        };

        repository.insert(index.clone());

        let criteria = ProposalSortIndexCriteria {
            proposal_id: [0; 16],
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);

        let value = result.into_iter().next().unwrap();

        assert_eq!(value.creation_timestamp, index.value.creation_timestamp);
        assert_eq!(
            value.modification_timestamp,
            index.value.modification_timestamp
        );
        assert_eq!(value.expiration_timestamp, index.value.expiration_timestamp);
    }
}
