use super::indexes::external_canister_index::ExternalCanisterIndexRepository;
use crate::{
    core::{utils::format_unique_string, with_memory_manager, Memory, EXTERNAL_CANISTER_MEMORY_ID},
    models::{
        indexes::external_canister_index::{
            ExternalCanisterIndexCriteria, ExternalCanisterIndexKind,
        },
        ExternalCanister, ExternalCanisterId, ExternalCanisterKey, ExternalCanisterState,
    },
};
use candid::Principal;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use orbit_essentials::repository::{IndexRepository, SortDirection};
use orbit_essentials::repository::{RefreshIndexMode, Repository};
use std::{cell::RefCell, collections::HashSet, sync::Arc};

thread_local! {
  /// The memory reference to the external canister repository.
  static DB: RefCell<StableBTreeMap<ExternalCanisterKey, ExternalCanister, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(EXTERNAL_CANISTER_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref EXTERNAL_CANISTER_REPOSITORY: Arc<ExternalCanisterRepository> =
        Arc::new(ExternalCanisterRepository::default());
}

/// A repository that enables managing external canisters in stable memory.
#[derive(Debug, Default)]
pub struct ExternalCanisterRepository {
    indexes: ExternalCanisterIndexRepository,
}

impl Repository<ExternalCanisterKey, ExternalCanister> for ExternalCanisterRepository {
    fn list(&self) -> Vec<ExternalCanister> {
        DB.with(|m| m.borrow().iter().map(|(_, v)| v).collect())
    }

    fn get(&self, key: &ExternalCanisterKey) -> Option<ExternalCanister> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(
        &self,
        key: ExternalCanisterKey,
        value: ExternalCanister,
    ) -> Option<ExternalCanister> {
        DB.with(|m| {
            let prev = m.borrow_mut().insert(key, value.clone());

            self.indexes
                .refresh_index_on_modification(RefreshIndexMode::List {
                    previous: prev
                        .clone()
                        .map_or(Vec::new(), |prev: ExternalCanister| prev.indexes()),
                    current: value.indexes(),
                });

            prev
        })
    }

    fn remove(&self, key: &ExternalCanisterKey) -> Option<ExternalCanister> {
        DB.with(|m| m.borrow_mut().remove(key))
    }

    fn len(&self) -> usize {
        DB.with(|m| m.borrow().len()) as usize
    }
}

impl ExternalCanisterRepository {
    /// Returns an external canister by its name if it exists.
    pub fn find_by_name(&self, name: &str) -> Option<ExternalCanisterId> {
        let name = format_unique_string(name);

        self.indexes.find_by_name(&name)
    }

    /// Returns an external canister by its canister id if it exists.
    pub fn find_by_canister_id(&self, canister_id: &Principal) -> Option<ExternalCanisterId> {
        let found = self
            .indexes
            .find_by_criteria(ExternalCanisterIndexCriteria {
                from: ExternalCanisterIndexKind::CanisterId(*canister_id),
                to: ExternalCanisterIndexKind::CanisterId(*canister_id),
            });

        found.into_iter().next()
    }

    /// Verifies that the name is unique among external canisters.
    ///
    /// If `skip_id` is provided, it will be ignored if the match would be the same.
    pub fn is_unique_name(&self, name: &str, skip_id: Option<ExternalCanisterId>) -> bool {
        self.find_by_name(name)
            .map_or(true, |existing_id| skip_id == Some(existing_id))
    }

    /// Verifies that the canister id is unique among external canisters.
    ///
    /// If `skip_id` is provided, it will be ignored if the match would be the same.
    pub fn is_unique_canister_id(
        &self,
        canister_id: &Principal,
        skip_id: Option<ExternalCanisterId>,
    ) -> bool {
        self.find_by_canister_id(canister_id)
            .map_or(true, |existing_id| skip_id == Some(existing_id))
    }

    /// Finds all the labels of the external canisters, which are unique.
    pub fn find_all_labels(&self) -> Vec<String> {
        self.indexes.find_all_labels()
    }

    /// Finds the names of the external canisters that start with the given prefix.
    pub fn find_names_by_prefix(
        &self,
        prefix: &str,
    ) -> Vec<(String, ExternalCanisterId, Principal)> {
        self.indexes.find_names_by_prefix(prefix)
    }

    /// Finds external canisters based on the provided where clause.
    pub fn find_canister_ids_where(
        &self,
        where_clause: ExternalCanisterWhereClause,
    ) -> Vec<Principal> {
        let filter_by_labels: HashSet<String> = where_clause.labels.into_iter().collect();
        let filter_by_canister_ids: HashSet<Principal> =
            where_clause.canister_ids.into_iter().collect();
        let filter_by_states: HashSet<ExternalCanisterState> =
            where_clause.states.into_iter().collect();

        let mut found_ids = self
            .list()
            .into_iter()
            .filter_map(|entry| {
                if !filter_by_labels.is_empty()
                    && !entry
                        .labels
                        .iter()
                        .any(|label| filter_by_labels.contains(label))
                {
                    return None;
                }

                if !filter_by_canister_ids.is_empty()
                    && !filter_by_canister_ids.contains(&entry.canister_id)
                {
                    return None;
                }

                if !filter_by_states.is_empty() && !filter_by_states.contains(&entry.state) {
                    return None;
                }

                Some((entry.name, entry.canister_id))
            })
            .collect::<Vec<(String, Principal)>>();

        let sort_by = match where_clause.sort_by {
            Some(sort_by) => sort_by,
            None => ExternalCanisterWhereClauseSort::Name(SortDirection::Ascending),
        };

        match sort_by {
            ExternalCanisterWhereClauseSort::Name(direction) => {
                found_ids.sort_by(|(name_a, _), (name_b, _)| match direction {
                    SortDirection::Ascending => name_a.cmp(name_b),
                    SortDirection::Descending => name_b.cmp(name_a),
                });
            }
        }

        found_ids
            .into_iter()
            .map(|(_, canister_id)| canister_id)
            .collect()
    }
}

#[derive(Debug, Clone)]
pub enum ExternalCanisterWhereClauseSort {
    Name(SortDirection),
}

#[derive(Debug, Clone)]
pub struct ExternalCanisterWhereClause {
    pub canister_ids: Vec<Principal>,
    pub labels: Vec<String>,
    pub states: Vec<ExternalCanisterState>,
    pub sort_by: Option<ExternalCanisterWhereClauseSort>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::external_canister_test_utils::mock_external_canister;

    #[test]
    fn test_crud() {
        let repository = ExternalCanisterRepository::default();
        let entry = mock_external_canister();

        assert!(repository.get(&entry.to_key()).is_none());

        repository.insert(entry.to_key(), entry.clone());

        assert!(repository.get(&entry.to_key()).is_some());
        assert!(repository.remove(&entry.to_key()).is_some());
        assert!(repository.get(&entry.to_key()).is_none());
    }

    #[test]
    fn test_find_by_name() {
        let repository = ExternalCanisterRepository::default();
        for i in 0..10 {
            let mut entry = mock_external_canister();
            entry.name = format!("test-{}", i);

            repository.insert(entry.to_key(), entry);
        }

        let result = repository.find_by_name("test-5");

        assert!(result.is_some());
    }

    #[test]
    fn test_find_by_canister_id() {
        let repository = ExternalCanisterRepository::default();
        for i in 0..10 {
            let mut entry = mock_external_canister();
            entry.canister_id = Principal::from_slice(&[i; 29]);

            repository.insert(entry.to_key(), entry);
        }

        assert!(repository
            .find_by_canister_id(&Principal::from_slice(&[8; 29]))
            .is_some());

        assert!(repository
            .find_by_canister_id(&Principal::from_slice(&[10; 29]))
            .is_none());
    }

    #[test]
    fn test_is_unique_name() {
        let repository = ExternalCanisterRepository::default();
        let entry = mock_external_canister();

        assert!(repository.is_unique_name(&entry.name, None));

        repository.insert(entry.to_key(), entry.clone());

        assert!(!repository.is_unique_name(&entry.name, None));
        assert!(repository.is_unique_name(&entry.name, Some(entry.id)));
    }

    #[test]
    fn test_is_unique_canister_id() {
        let repository = ExternalCanisterRepository::default();
        let entry = mock_external_canister();

        assert!(repository.is_unique_canister_id(&entry.canister_id, None));

        repository.insert(entry.to_key(), entry.clone());

        assert!(!repository.is_unique_canister_id(&entry.canister_id, None));
        assert!(repository.is_unique_canister_id(&entry.canister_id, Some(entry.id)));
    }
}
