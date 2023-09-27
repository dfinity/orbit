/// A repository is a generic interface for storing and retrieving data.
pub trait Repository<Key, Value> {
    /// Returns the record from the repository if it exists.
    fn get(&self, key: &Key) -> Option<Value>;

    /// Inserts a record into the repository.
    fn insert(&self, key: Key, value: Value) -> Option<Value>;

    /// Removes a record from the repository and returns it if it exists.
    fn remove(&self, key: &Key) -> Option<Value>;
}
