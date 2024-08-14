use crate::{
    core::{
        utils::{MAX_PRINCIPAL, MIN_PRINCIPAL},
        with_memory_manager, Memory, EXTERNAL_CANISTER_INDEX_MEMORY_ID,
    },
    models::{
        indexes::external_canister_index::{
            ExternalCanisterIndex, ExternalCanisterIndexCriteria, ExternalCanisterIndexKind,
        },
        ExternalCanisterEntryId,
    },
};
use candid::Principal;
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

impl IndexRepository<ExternalCanisterIndex, ExternalCanisterEntryId>
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

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<ExternalCanisterEntryId> {
        DB.with(|db| {
            let start_key = ExternalCanisterIndex {
                index: criteria.from,
                external_canister_entry_id: [u8::MIN; 16],
                canister_id: MIN_PRINCIPAL,
            };
            let end_key = ExternalCanisterIndex {
                index: criteria.to,
                external_canister_entry_id: [u8::MAX; 16],
                canister_id: MAX_PRINCIPAL,
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.external_canister_entry_id)
                .collect::<HashSet<ExternalCanisterEntryId>>()
        })
    }
}

impl ExternalCanisterIndexRepository {
    pub const MAX_NAME_PREFIX_LIST_LIMIT: usize = 250;

    pub fn len(&self) -> u64 {
        DB.with(|m| m.borrow().len())
    }

    pub fn is_empty(&self) -> bool {
        DB.with(|m| m.borrow().is_empty())
    }

    /// Finds the names of the external canisters that start with the given prefix.
    ///
    /// Returns a list of names, external canister ids, and their canister ids.
    pub fn find_names_by_prefix(
        &self,
        prefix: &str,
    ) -> Vec<(String, ExternalCanisterEntryId, Principal)> {
        DB.with(|db| {
            db.borrow()
                .range((ExternalCanisterIndex {
                    index: ExternalCanisterIndexKind::Name(prefix.to_string()),
                    external_canister_entry_id: [u8::MIN; 16],
                    canister_id: MIN_PRINCIPAL,
                })..)
                .take_while(|(index, _)| matches!(&index.index, ExternalCanisterIndexKind::Name(name) if name.starts_with(prefix)))
                .filter_map(|(index, _)| match &index.index {
                    ExternalCanisterIndexKind::Name(name) => Some((name.clone(), index.external_canister_entry_id, index.canister_id)),
                    _ => None,
                })
                .collect()
        })
    }

    /// Finds the external canister that matches the given name.
    pub fn find_by_name(&self, search_name: &str) -> Option<ExternalCanisterEntryId> {
        DB.with(|db| {
            db.borrow()
                .range(
                    (ExternalCanisterIndex {
                        index: ExternalCanisterIndexKind::Name(search_name.to_string()),
                        external_canister_entry_id: [u8::MIN; 16],
                        canister_id: MIN_PRINCIPAL,
                    })..=(ExternalCanisterIndex {
                        index: ExternalCanisterIndexKind::Name(search_name.to_string()),
                        external_canister_entry_id: [u8::MAX; 16],
                        canister_id: MAX_PRINCIPAL,
                    }),
                )
                .filter_map(|(index, _)| match &index.index {
                    ExternalCanisterIndexKind::Name(name) => match name == search_name {
                        true => Some(index.external_canister_entry_id),
                        false => None,
                    },
                    _ => None,
                })
                .collect::<Vec<ExternalCanisterEntryId>>()
                .first()
                .cloned()
        })
    }

    /// Finds all the labels of the external canisters, which are unique.
    pub fn find_all_labels(&self) -> Vec<String> {
        DB.with(|db| {
            db.borrow()
                .range(
                    (ExternalCanisterIndex {
                        index: ExternalCanisterIndexKind::Label(String::new()),
                        external_canister_entry_id: [u8::MIN; 16],
                        canister_id: MIN_PRINCIPAL,
                    })..,
                )
                .take_while(|(index, _)| {
                    matches!(&index.index, ExternalCanisterIndexKind::Label(_))
                })
                .filter_map(|(index, _)| match &index.index {
                    ExternalCanisterIndexKind::Label(label) => Some(label.clone()),
                    _ => None,
                })
                .collect::<HashSet<String>>()
                .into_iter()
                .collect()
        })
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
            external_canister_entry_id: [1; 16],
            canister_id: Principal::from_slice(&[1; 29]),
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
                external_canister_entry_id: [i; 16],
                canister_id: Principal::from_slice(&[i; 29]),
            });
        }

        for i in 0..10 {
            let result = repository.find_by_name(&format!("test-{}", i));

            assert!(result.is_some());
            assert_eq!(result.unwrap(), [i; 16]);
        }
    }

    #[test]
    fn test_find_by_canister_id() {
        let repository = ExternalCanisterIndexRepository::default();
        for i in 0..10 {
            repository.insert(ExternalCanisterIndex {
                index: ExternalCanisterIndexKind::CanisterId(Principal::from_slice(&[i; 29])),
                external_canister_entry_id: [i; 16],
                canister_id: Principal::from_slice(&[i; 29]),
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
    fn test_find_by_name_prefix() {
        let repository = ExternalCanisterIndexRepository::default();
        let mut expected_results = Vec::new();
        let search_prefix = "test2";
        for i in 0..201 {
            let index_name = format!("test{}", i);
            repository.insert(ExternalCanisterIndex {
                index: ExternalCanisterIndexKind::Name(index_name.clone()),
                external_canister_entry_id: [i; 16],
                canister_id: Principal::from_slice(&[i; 29]),
            });

            if index_name.starts_with(search_prefix) {
                expected_results.push(index_name);
            }
        }

        let result = repository
            .find_names_by_prefix("test2")
            .into_iter()
            .map(|(name, _, _)| name)
            .collect::<Vec<String>>();

        assert!(!result.is_empty());
        assert_eq!(result.len(), expected_results.len());
        for name in expected_results {
            assert!(result.contains(&name));
        }
    }

    #[test]
    fn test_find_all_labels() {
        let repository = ExternalCanisterIndexRepository::default();
        for i in 0..10 {
            repository.insert(ExternalCanisterIndex {
                index: ExternalCanisterIndexKind::Label(format!("label-{}", i)),
                external_canister_entry_id: [i; 16],
                canister_id: Principal::from_slice(&[i; 29]),
            });
        }

        for i in 0..10 {
            repository.insert(ExternalCanisterIndex {
                index: ExternalCanisterIndexKind::Label(format!("label-{}", i)),
                external_canister_entry_id: [i + 20; 16],
                canister_id: Principal::from_slice(&[i + 20; 29]),
            });
        }

        let result = repository.find_all_labels();

        assert_eq!(repository.len(), 20);
        assert_eq!(result.len(), 10);
        for i in 0..10 {
            assert!(result.contains(&format!("label-{}", i)));
        }
    }
}
