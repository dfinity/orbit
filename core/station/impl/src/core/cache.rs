use std::collections::HashMap;

/// The default cache size when default is used.
const DEFAULT_CACHE_SIZE: usize = 10_000;

/// A simple cache implementation that removes the first element when the cache is full.
///
/// The cache is implemented using a HashMap.
pub struct Cache<Key, Value> {
    max_size: usize,
    map: HashMap<Key, Value>,
}

impl<Key, Value> Cache<Key, Value>
where
    Key: std::cmp::Eq + std::hash::Hash + Clone,
{
    /// Creates a new cache with a maximum size.
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            map: HashMap::new(),
        }
    }

    /// Returns the number of elements in the cache.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns true if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Inserts a key-value pair into the cache.
    pub fn insert(&mut self, key: Key, value: Value) {
        // Remove an element if the cache is full.
        let key_to_remove = if self.map.len() >= self.max_size {
            self.map.keys().next().cloned()
        } else {
            None
        };

        if let Some(key) = key_to_remove {
            self.map.remove(&key);
        }

        self.map.insert(key, value);
    }

    /// Returns a reference to the value at the key if it exists.
    pub fn get(&self, key: &Key) -> Option<&Value> {
        self.map.get(key)
    }

    /// Removes a key from the cache and returns the value at the key if it exists.
    pub fn remove(&mut self, key: &Key) -> Option<Value> {
        self.map.remove(key)
    }

    /// Contains the key.
    pub fn contains_key(&self, key: &Key) -> bool {
        self.map.contains_key(key)
    }

    /// Clears the cache.
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Returns an iterator over the cache's values.
    pub fn values(&self) -> std::collections::hash_map::Values<Key, Value> {
        self.map.values()
    }

    /// Returns an iterator over the cache's key-value pairs.
    pub fn iter(&self) -> std::collections::hash_map::Iter<Key, Value> {
        self.map.iter()
    }
}

impl<Key, Value> Default for Cache<Key, Value>
where
    Key: std::cmp::Eq + std::hash::Hash + Clone,
{
    fn default() -> Self {
        Self::new(DEFAULT_CACHE_SIZE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_limited_to_max_size() {
        let mut cache = Cache::new(2);

        cache.insert(1, "one");
        cache.insert(2, "two");

        assert_eq!(cache.get(&1), Some(&"one"));
        assert_eq!(cache.get(&2), Some(&"two"));
        assert_eq!(cache.len(), 2);

        cache.insert(3, "three");

        assert_eq!(cache.len(), 2);
    }
}
