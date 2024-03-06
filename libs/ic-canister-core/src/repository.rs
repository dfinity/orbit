use std::collections::HashSet;

use crate::types::UUID;

/// A repository is a generic interface for storing and retrieving data.
pub trait Repository<Key, Value> {
    /// Returns the list of records from the repository.
    fn list(&self) -> Vec<Value>;

    /// Returns the record from the repository if it exists.
    fn get(&self, key: &Key) -> Option<Value>;

    /// Inserts a record into the repository.
    fn insert(&self, key: Key, value: Value) -> Option<Value>;

    /// Removes a record from the repository and returns it if it exists.
    fn remove(&self, key: &Key) -> Option<Value>;

    /// Returns the number of records stored in the repository.
    fn len(&self) -> usize;

    /// Returns whether the repository is empty or not.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn refresh_indexes(&self, _current: Value, _previous: Option<Value>) {
        // no-op
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

    fn refresh_index_on_modification(&self, mode: RefreshIndexMode<Index>)
    where
        Index: Eq + std::hash::Hash + Clone,
    {
        match mode {
            RefreshIndexMode::Value { previous, current } => match (previous, current) {
                (Some(prev), Some(curr)) => {
                    if prev != curr {
                        self.remove(&prev);
                        self.insert(curr);
                    }
                }
                (Some(prev), None) => {
                    self.remove(&prev);
                }
                (None, Some(curr)) => {
                    self.insert(curr);
                }
                _ => {}
            },
            RefreshIndexMode::List { previous, current } => {
                let set_prev: HashSet<Index> = previous.into_iter().collect();
                let set_curr: HashSet<Index> = current.into_iter().collect();

                for prev in set_prev.difference(&set_curr) {
                    self.remove(prev);
                }

                for curr in set_curr.difference(&set_prev) {
                    self.insert(curr.clone());
                }
            }
            RefreshIndexMode::CleanupValue { current } => {
                if let Some(curr) = current {
                    self.remove(&curr);
                }
            }
            RefreshIndexMode::CleanupList { current } => {
                for curr in current {
                    self.remove(&curr);
                }
            }
        }
    }
}

pub enum RefreshIndexMode<Index> {
    Value {
        previous: Option<Index>,
        current: Option<Index>,
    },
    List {
        previous: Vec<Index>,
        current: Vec<Index>,
    },
    CleanupValue {
        current: Option<Index>,
    },
    CleanupList {
        current: Vec<Index>,
    },
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
                let mut new_ids = self.select();
                new_ids.retain(|id| ids.contains(id));

                new_ids
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

            if found_ids.is_some() && found_ids.as_ref().unwrap().is_empty() {
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
