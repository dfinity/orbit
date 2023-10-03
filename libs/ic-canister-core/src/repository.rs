/// A repository is a generic interface for storing and retrieving data.
pub trait Repository<Key, Value> {
    /// Returns the record from the repository if it exists.
    fn get(&self, key: &Key) -> Option<Value>;

    /// Inserts a record into the repository.
    fn insert(&self, key: Key, value: Value) -> Option<Value>;

    /// Removes a record from the repository and returns it if it exists.
    fn remove(&self, key: &Key) -> Option<Value>;
}

/// An index repository is a generic interface for storing and retrieving data based on an index.
pub trait IndexRepository<Index, Record> {
    type FindByCriteria;

    /// Checks if an index exists.
    fn exists(&self, index: &Index) -> bool;

    /// Inserts a new index entry into the repository.
    fn insert(&self, index: Index);

    /// Removes the index entry from the repository and returns true if it existed.
    fn remove(&self, index: &Index) -> bool;

    /// Returns all records from the repository that match a set criteria.
    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> Vec<Record>;
}
