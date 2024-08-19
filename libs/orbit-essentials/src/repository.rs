use crate::{model::ModelKey, types::UUID};
use ic_stable_structures::{Memory, StableBTreeMap, Storable};
use std::collections::HashSet;

pub trait StableDb<Key, Value, Mem>
where
    Key: Eq + std::hash::Hash + Clone + Ord + Storable,
    Value: Clone + Storable,
    Mem: Memory,
{
    fn with_db<F, R>(f: F) -> R
    where
        F: FnOnce(&mut StableBTreeMap<Key, Value, Mem>) -> R;
}

pub trait IndexedRepository<Key, Value, Mem>: Repository<Key, Value, Mem>
where
    Key: Eq + std::hash::Hash + Clone + Ord + Storable,
    Value: Clone + Storable + ModelKey<Key>,
    Mem: Memory,
{
    /// Removes the indexes for the current value.
    fn remove_entry_indexes(&self, value: &Value);

    /// Adds the indexes for the current value.
    fn add_entry_indexes(&self, value: &Value);

    /// Saves the indexes for the current value and removes the old indexes if
    /// the value has changed.
    fn save_entry_indexes(&self, value: &Value, previous: Option<&Value>) {
        if let Some(prev) = previous {
            self.remove_entry_indexes(prev);
        }

        self.add_entry_indexes(value);
    }
}

pub trait RebuildRepository<Key, Value, Mem>:
    Repository<Key, Value, Mem> + IndexedRepository<Key, Value, Mem>
where
    Key: Eq + std::hash::Hash + Clone + Ord + Storable,
    Value: Clone + Storable + ModelKey<Key>,
    Mem: Memory,
{
    /// This method goes over all the entries and rebuilds the indexes.
    ///
    /// WARNING: Please only use during upgrades to ensure enough intructions are available.
    fn rebuild(&self) {
        Self::with_db(|db| {
            let keys = db.iter().map(|(k, _)| k.clone()).collect::<Vec<Key>>();

            for key in keys {
                if let Some(value) = db.get(&key) {
                    // First make sure there is no dangling index for the entry.
                    self.remove_entry_indexes(&value);
                    // Then add the updated indexes.
                    self.add_entry_indexes(&value);
                    // Finally, update the entry in the database.
                    db.insert(key, value);
                }
            }
        });
    }
}

/// A repository is a generic interface for storing and retrieving data.
pub trait Repository<Key, Value, Mem>: StableDb<Key, Value, Mem>
where
    Key: Eq + std::hash::Hash + Clone + Ord + Storable,
    Value: Clone + Storable,
    Mem: Memory,
{
    /// Returns the list of records from the repository.
    fn list(&self) -> Vec<Value> {
        Self::with_db(|db| db.iter().map(|(_, v)| v.clone()).collect())
    }

    /// Returns whether a record exists in the repository.
    fn exists(&self, key: &Key) -> bool {
        Self::with_db(|db| db.contains_key(key))
    }

    /// Returns the record from the repository if it exists.
    fn get(&self, key: &Key) -> Option<Value> {
        Self::with_db(|db| db.get(key))
    }

    /// Inserts a record into the repository.
    fn insert(&self, key: Key, value: Value) -> Option<Value> {
        Self::with_db(|db| db.insert(key, value))
    }

    /// Removes a record from the repository and returns it if it exists.
    fn remove(&self, key: &Key) -> Option<Value> {
        Self::with_db(|db| db.remove(key))
    }

    /// Returns the number of records stored in the repository.
    fn len(&self) -> usize {
        Self::with_db(|db| db.len() as usize)
    }

    /// Returns whether the repository is empty or not.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn find_with_filters<'a>(
        &self,
        filters: Vec<Box<dyn SelectionFilter<'a, IdType = UUID> + 'a>>,
    ) -> HashSet<UUID> {
        let mut found_ids = None;

        for filter in filters {
            found_ids = Some(filter.apply(found_ids.as_ref()));
        }

        found_ids.unwrap_or_default()
    }
}

/// An index repository is a generic interface for storing and retrieving data based on an index.
pub trait IndexRepository<Index, Value> {
    type FindByCriteria;

    /// Checks if an index exists.
    fn exists(&self, index: &Index) -> bool;

    /// Inserts a new index entry into the repository.
    fn insert(&self, index: Index);

    /// Removes the index entry from the repository and returns true if it existed.
    fn remove(&self, index: &Index) -> bool;

    /// Returns all records from the repository that match a set criteria.
    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<Value>;
}

/// A filter that can be applied to a set of ids to select or filter them down based on some criteria.
///
/// By default, the filter is meant to filter down the set of IDs and not select them, unless
/// the `is_selective` method is overridden.
pub trait SelectionFilter<'a>
where
    Self::IdType: Clone + Eq + std::hash::Hash,
{
    /// The type of the IDs that the filter operates on
    type IdType;

    /// Applies the filter to the existing set of IDs and returns the new set of IDs
    fn apply(&self, existing_ids: Option<&HashSet<Self::IdType>>) -> HashSet<Self::IdType> {
        match (existing_ids, self.is_selective()) {
            (Some(ids), true) => {
                let new_ids = self.select();
                new_ids.intersection(ids).cloned().collect()
            }
            (Some(ids), false) => ids.iter().filter(|id| self.matches(id)).cloned().collect(),
            // If the existing set of IDs is None, then we are meant to select all IDs that match the filter
            (None, true) | (None, false) => self.select(),
        }
    }

    /// Returns true if the item matches the filter criteria
    ///
    /// By default it is true for all items, which means that the filter is a no-op.
    ///
    /// A no-op match is useful for logical operations (e.g. AND and OR), where the filter is meant to
    /// rely on other filters to do the actual filtering.
    fn matches(&self, _item_id: &Self::IdType) -> bool {
        true
    }

    /// Returns true if the filter is meant to select rather than filter down
    ///
    /// By default it is false
    fn is_selective(&self) -> bool {
        false
    }

    /// Returns the initial set of IDs for the filter
    ///
    /// By default it is an empty set
    fn select(&self) -> HashSet<Self::IdType> {
        HashSet::new()
    }
}

/// A filter that combines multiple filters using a logical AND operation.
pub struct AndSelectionFilter<'a> {
    pub filters: Vec<Box<dyn SelectionFilter<'a, IdType = UUID> + 'a>>,
}

impl<'a> SelectionFilter<'a> for AndSelectionFilter<'a> {
    type IdType = UUID;

    fn apply(&self, existing_ids: Option<&HashSet<Self::IdType>>) -> HashSet<Self::IdType> {
        let mut found_ids: Option<HashSet<Self::IdType>> = None;

        for filter in &self.filters {
            found_ids = Some(filter.apply(found_ids.as_ref()));

            if found_ids.as_ref().map_or(false, |ids| ids.is_empty()) {
                break;
            }
        }

        let newly_found_ids = found_ids.unwrap_or_default();

        match existing_ids {
            Some(ids) => {
                let mut new_ids = newly_found_ids;
                new_ids.retain(|id| ids.contains(id));

                new_ids
            }
            None => newly_found_ids,
        }
    }
}

/// A filter that combines multiple filters using a logical OR operation.
pub struct OrSelectionFilter<'a> {
    pub filters: Vec<Box<dyn SelectionFilter<'a, IdType = UUID> + 'a>>,
}

impl<'a> SelectionFilter<'a> for OrSelectionFilter<'a> {
    type IdType = UUID;

    fn apply(&self, existing_ids: Option<&HashSet<Self::IdType>>) -> HashSet<Self::IdType> {
        let mut found_ids = HashSet::new();

        for filter in &self.filters {
            let new_ids = filter.apply(existing_ids);

            found_ids.extend(new_ids);
        }

        match existing_ids {
            Some(ids) => {
                let mut new_ids = found_ids;
                new_ids.retain(|id| ids.contains(id));

                new_ids
            }
            None => found_ids,
        }
    }
}

/// A filter that negates the result of another filter.
pub struct NotSelectionFilter<'a> {
    pub input: Box<dyn SelectionFilter<'a, IdType = UUID> + 'a>,
}

impl<'a> SelectionFilter<'a> for NotSelectionFilter<'a> {
    type IdType = UUID;

    fn apply(&self, existing_ids: Option<&HashSet<Self::IdType>>) -> HashSet<Self::IdType> {
        match existing_ids {
            None => Default::default(),
            Some(ids) => {
                let matching_ids = self.input.apply(Some(ids));
                ids.iter()
                    .filter(|id| !matching_ids.contains(*id))
                    .cloned()
                    .collect()
            }
        }
    }
}

/// A filter that selects a pre-determined set of IDs.
#[derive(Debug, Clone)]
pub struct IdentitySelectionFilter {
    pub ids: HashSet<UUID>,
}

impl<'a> SelectionFilter<'a> for IdentitySelectionFilter {
    type IdType = UUID;

    fn matches(&self, id: &Self::IdType) -> bool {
        self.ids.contains(id)
    }

    fn select(&self) -> HashSet<Self::IdType> {
        self.ids.clone()
    }
}

/// The sorting direction for a list of items in a repository.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

/// A strategy for sorting a list of loaded items.
pub trait SortingStrategy<'a> {
    type IdType;

    fn sort(&self, ids: &mut [Self::IdType]);
}

/// The default sorting strategy that sorts items based on their natural ordering.
///
/// When the sorting direction is not specified, it defaults to ascending.
pub struct DefaultSortingStrategy {
    pub direction: Option<SortDirection>,
}

impl<'a> SortingStrategy<'a> for DefaultSortingStrategy {
    type IdType = UUID;

    fn sort(&self, ids: &mut [Self::IdType]) {
        let direction = self.direction.unwrap_or(SortDirection::Ascending);

        ids.sort_by(|a, b| match direction {
            SortDirection::Ascending => a.cmp(b),
            SortDirection::Descending => b.cmp(a),
        });
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::repository::{IdentitySelectionFilter, NotSelectionFilter, SelectionFilter};

    #[test]
    fn test_not_selection_filter() {
        let filter: Box<dyn SelectionFilter<IdType = [u8; 16]> + '_> =
            Box::new(NotSelectionFilter {
                input: Box::new(IdentitySelectionFilter {
                    ids: vec![[0u8; 16], [2u8; 16]]
                        .into_iter()
                        .collect::<HashSet<[u8; 16]>>(),
                }),
            });

        let empty_result = filter.apply(None);
        assert_eq!(empty_result, Default::default());

        let non_empty_result = filter.apply(Some(
            &vec![[0u8; 16], [1u8; 16], [2u8; 16]]
                .into_iter()
                .collect::<HashSet<[u8; 16]>>(),
        ));
        assert_eq!(
            non_empty_result,
            vec![[1u8; 16]].into_iter().collect::<HashSet<[u8; 16]>>()
        );
    }
}
