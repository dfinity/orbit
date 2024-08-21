use super::indexes::unique_index::UniqueIndexRepository;
use crate::{
    core::{utils::format_unique_string, with_memory_manager, Memory, EXTERNAL_CANISTER_MEMORY_ID},
    models::{
        indexes::unique_index::UniqueIndexKey, ExternalCanister, ExternalCanisterEntryId,
        ExternalCanisterKey, ExternalCanisterState,
    },
};
use candid::Principal;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use orbit_essentials::repository::{IndexedRepository, SortDirection};
use orbit_essentials::repository::{Repository, StableDb};
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
    unique_index: UniqueIndexRepository,
}

impl StableDb<ExternalCanisterKey, ExternalCanister, VirtualMemory<Memory>>
    for ExternalCanisterRepository
{
    fn with_db<F, R>(f: F) -> R
    where
        F: FnOnce(
            &mut StableBTreeMap<ExternalCanisterKey, ExternalCanister, VirtualMemory<Memory>>,
        ) -> R,
    {
        DB.with(|m| f(&mut m.borrow_mut()))
    }
}

impl IndexedRepository<ExternalCanisterKey, ExternalCanister, VirtualMemory<Memory>>
    for ExternalCanisterRepository
{
    fn remove_entry_indexes(&self, value: &ExternalCanister) {
        self.unique_index.refresh(&[], &value.to_unique_indexes());
    }

    fn add_entry_indexes(&self, value: &ExternalCanister) {
        self.unique_index.refresh(&value.to_unique_indexes(), &[]);
    }
}

impl Repository<ExternalCanisterKey, ExternalCanister, VirtualMemory<Memory>>
    for ExternalCanisterRepository
{
    fn insert(
        &self,
        key: ExternalCanisterKey,
        value: ExternalCanister,
    ) -> Option<ExternalCanister> {
        DB.with(|m| {
            let prev = m.borrow_mut().insert(key, value.clone());

            self.save_entry_indexes(&value, prev.as_ref());

            prev
        })
    }

    fn remove(&self, key: &ExternalCanisterKey) -> Option<ExternalCanister> {
        DB.with(|m| {
            let prev = m.borrow_mut().remove(key);

            if let Some(prev) = &prev {
                self.remove_entry_indexes(prev);
            }

            prev
        })
    }
}

impl ExternalCanisterRepository {
    /// Returns an external canister by its name if it exists.
    pub fn find_by_name(&self, name: &str) -> Option<ExternalCanisterEntryId> {
        let name = format_unique_string(name);

        self.unique_index
            .get(&UniqueIndexKey::ExternalCanisterName(name))
    }

    /// Returns an external canister by its canister id if it exists.
    pub fn find_by_canister_id(&self, canister_id: &Principal) -> Option<ExternalCanisterEntryId> {
        self.unique_index
            .get(&UniqueIndexKey::ExternalCanisterId(*canister_id))
    }

    /// Verifies that the name is unique among external canisters.
    ///
    /// If `skip_id` is provided, it will be ignored if the match would be the same.
    pub fn is_unique_name(&self, name: &str, skip_id: Option<ExternalCanisterEntryId>) -> bool {
        self.find_by_name(name)
            .map_or(true, |existing_id| skip_id == Some(existing_id))
    }

    /// Verifies that the canister id is unique among external canisters.
    ///
    /// If `skip_id` is provided, it will be ignored if the match would be the same.
    pub fn is_unique_canister_id(
        &self,
        canister_id: &Principal,
        skip_id: Option<ExternalCanisterEntryId>,
    ) -> bool {
        self.find_by_canister_id(canister_id)
            .map_or(true, |existing_id| skip_id == Some(existing_id))
    }

    /// Finds all the labels of the external canisters, which are unique.
    pub fn find_all_labels(&self) -> Vec<String> {
        self.list()
            .into_iter()
            .flat_map(|entry| entry.labels.into_iter())
            .collect::<HashSet<String>>()
            .into_iter()
            .collect()
    }

    /// Finds the names of the external canisters that start with the given prefix.
    pub fn find_names_by_prefix(
        &self,
        prefix: &str,
    ) -> Vec<(String, ExternalCanisterEntryId, Principal)> {
        self.unique_index
            .find_by_criteria(
                Some(UniqueIndexKey::ExternalCanisterName(format_unique_string(
                    prefix,
                ))),
                None,
                None,
            )
            .into_iter()
            .filter_map(|id| {
                self.get(&ExternalCanisterKey { id })
                    .map(|entry| (entry.name, entry.id, entry.canister_id))
            })
            .collect()
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
    use orbit_essentials::model::ModelKey;

    #[test]
    fn test_crud() {
        let repository = ExternalCanisterRepository::default();
        let entry = mock_external_canister();

        assert!(repository.get(&entry.key()).is_none());

        repository.insert(entry.key(), entry.clone());

        assert!(repository.get(&entry.key()).is_some());
        assert!(repository.remove(&entry.key()).is_some());
        assert!(repository.get(&entry.key()).is_none());
    }

    #[test]
    fn test_find_by_name() {
        let repository = ExternalCanisterRepository::default();
        for i in 0..10 {
            let mut entry = mock_external_canister();
            entry.name = format!("test-{}", i);

            repository.insert(entry.key(), entry);
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

            repository.insert(entry.key(), entry);
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

        repository.insert(entry.key(), entry.clone());

        assert!(!repository.is_unique_name(&entry.name, None));
        assert!(repository.is_unique_name(&entry.name, Some(entry.id)));
    }

    #[test]
    fn test_is_unique_canister_id() {
        let repository = ExternalCanisterRepository::default();
        let entry = mock_external_canister();

        assert!(repository.is_unique_canister_id(&entry.canister_id, None));

        repository.insert(entry.key(), entry.clone());

        assert!(!repository.is_unique_canister_id(&entry.canister_id, None));
        assert!(repository.is_unique_canister_id(&entry.canister_id, Some(entry.id)));
    }
}
