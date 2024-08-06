use crate::{
    core::{with_memory_manager, Memory, EXTERNAL_CANISTER_INDEX_MEMORY_ID},
    models::{
        indexes::external_canister_index::{ExternalCanisterIndex, ExternalCanisterIndexCriteria},
        ExternalCanisterId,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::repository::IndexRepository;
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<ExternalCanisterIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(EXTERNAL_CANISTER_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that holds the external canisters indexes, which is used to find external canisters efficiently.
#[derive(Default, Debug)]
pub struct ExternalCanisterIndexRepository {}

impl IndexRepository<ExternalCanisterIndex, ExternalCanisterId>
    for ExternalCanisterIndexRepository
{
    type FindByCriteria = ExternalCanisterIndexCriteria;

    fn exists(&self, index: &ExternalCanisterIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: ExternalCanisterIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &ExternalCanisterIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<ExternalCanisterId> {
        DB.with(|db| {
            let start_key = ExternalCanisterIndex {
                index: criteria.from,
                external_canister_id: [u8::MIN; 16],
            };
            let end_key = ExternalCanisterIndex {
                index: criteria.to,
                external_canister_id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.external_canister_id)
                .collect::<HashSet<ExternalCanisterId>>()
        })
    }
}

impl ExternalCanisterIndexRepository {
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
        models::indexes::external_canister_index::{
            ExternalCanisterIndex, ExternalCanisterIndexCriteria, ExternalCanisterIndexKind,
        },
        repositories::indexes::external_canister_index::ExternalCanisterIndexRepository,
    };
    use candid::Principal;
    use orbit_essentials::repository::IndexRepository;

    #[test]
    fn test_index_repository() {
        let repository = ExternalCanisterIndexRepository::default();
        let index = ExternalCanisterIndex {
            index: ExternalCanisterIndexKind::Name("test".to_string()),
            external_canister_id: [1; 16],
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_name() {
        let repository = ExternalCanisterIndexRepository::default();
        for i in 0..10 {
            repository.insert(ExternalCanisterIndex {
                index: ExternalCanisterIndexKind::Name(format!("test-{}", i)),
                external_canister_id: [i; 16],
            });
        }

        let result = repository.find_by_criteria(ExternalCanisterIndexCriteria {
            from: ExternalCanisterIndexKind::Name("test-5".to_string()),
            to: ExternalCanisterIndexKind::Name("test-5".to_string()),
        });

        assert!(!result.is_empty());
        assert_eq!(result.len(), 1);
        assert!(result.contains(&[5; 16]));
    }

    #[test]
    fn test_find_by_canister_id() {
        let repository = ExternalCanisterIndexRepository::default();
        for i in 0..10 {
            repository.insert(ExternalCanisterIndex {
                index: ExternalCanisterIndexKind::CanisterId(Principal::from_slice(&[i; 29])),
                external_canister_id: [i; 16],
            });
        }

        let result = repository.find_by_criteria(ExternalCanisterIndexCriteria {
            from: ExternalCanisterIndexKind::CanisterId(Principal::from_slice(&[5; 29])),
            to: ExternalCanisterIndexKind::CanisterId(Principal::from_slice(&[5; 29])),
        });

        assert!(!result.is_empty());
        assert_eq!(result.len(), 1);
        assert!(result.contains(&[5; 16]));
    }

    #[test]
    fn test_find_by_labels() {
        let repository = ExternalCanisterIndexRepository::default();
        for i in 0..10 {
            repository.insert(ExternalCanisterIndex {
                index: ExternalCanisterIndexKind::Label(format!("label-{}", i)),
                external_canister_id: [i; 16],
            });
        }

        let result = repository.find_by_criteria(ExternalCanisterIndexCriteria {
            from: ExternalCanisterIndexKind::Label("label-5".to_string()),
            to: ExternalCanisterIndexKind::Label("label-6".to_string()),
        });

        assert!(!result.is_empty());
        assert_eq!(result.len(), 2);
        assert!(result.contains(&[5; 16]));
        assert!(result.contains(&[6; 16]));
    }
}
