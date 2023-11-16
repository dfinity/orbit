use std::collections::HashSet;

/// A repository is a generic interface for storing and retrieving data.
pub trait Repository<Key, Value> {
    /// Returns the record from the repository if it exists.
    fn get(&self, key: &Key) -> Option<Value>;

    /// Inserts a record into the repository.
    fn insert(&self, key: Key, value: Value) -> Option<Value>;

    /// Removes a record from the repository and returns it if it exists.
    fn remove(&self, key: &Key) -> Option<Value>;

    fn refresh_indexes(&self, _current: Value, _previous: Option<Value>) {
        // no-op
    }
}

/// An index repository is a generic interface for storing and retrieving data based on an index.
pub trait IndexRepository<Index, RecordId> {
    type FindByCriteria;

    /// Checks if an index exists.
    fn exists(&self, index: &Index) -> bool;

    /// Inserts a new index entry into the repository.
    fn insert(&self, index: Index);

    /// Removes the index entry from the repository and returns true if it existed.
    fn remove(&self, index: &Index) -> bool;

    /// Returns all records from the repository that match a set criteria.
    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<RecordId>;

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
