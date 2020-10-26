use std::collections::HashMap;
use std::sync::RwLock;

/// A concurrent, on chain storage using interior mutability.
pub trait Storage {
    /// Return None if there is no block matching `height`.
    fn set(&self, height: u64, path: Vec<u8>, value: Vec<u8>) -> Option<()>;
    /// Return None if there is no block matching `height`.
    fn delete(&self, height: u64, path: &[u8]) -> Option<()>;
    /// Return None if there is no block matching `height`.
    fn get(&self, height: u64, path: &[u8]) -> Option<Vec<u8>>;
}

/// An in-memory store backed by a simple hashmap.
pub struct InMemoryStore {
    store: RwLock<Vec<HashMap<Vec<u8>, Vec<u8>>>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        let genesis = HashMap::new();
        InMemoryStore {
            store: RwLock::new(vec![genesis]),
        }
    }

    /// Returns the store at a given height, where 0 means latest.
    fn get_store_at_height(
        height: u64,
        store: &Vec<HashMap<Vec<u8>, Vec<u8>>>,
    ) -> Option<&HashMap<Vec<u8>, Vec<u8>>> {
        if height == 0 {
            store.last()
        } else {
            store.get((height - 1) as usize)
        }
    }

    /// Returns the store at a given height, where 0 means latest.
    fn get_store_at_height_mut(
        height: u64,
        store: &mut Vec<HashMap<Vec<u8>, Vec<u8>>>,
    ) -> Option<&mut HashMap<Vec<u8>, Vec<u8>>> {
        if height == 0 {
            store.last_mut()
        } else {
            store.get_mut((height - 1) as usize)
        }
    }
}

impl Storage for InMemoryStore {
    fn set(&self, height: u64, path: Vec<u8>, value: Vec<u8>) -> Option<()> {
        let mut store = self.store.write().unwrap();
        let store = InMemoryStore::get_store_at_height_mut(height, &mut store)?;
        store.insert(path, value);
        Some(())
    }

    fn get(&self, height: u64, path: &[u8]) -> Option<Vec<u8>> {
        let store = self.store.read().unwrap();
        let store = InMemoryStore::get_store_at_height(height, &store)?;
        store.get(path).cloned()
    }

    fn delete(&self, height: u64, path: &[u8]) -> Option<()> {
        let mut store = self.store.write().unwrap();
        let store = InMemoryStore::get_store_at_height_mut(height, &mut store)?;
        store.remove(path);
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store() {
        let store = InMemoryStore::new();
        test_with_store(store)
    }

    fn test_with_store<T: Storage>(store: T) {
        let data = b"hello";
        let path = b"foo/bar";
        let data = &data[..];
        let path = &path[..];

        assert_eq!(store.get(0, path), None);
        store.set(0, path.to_vec(), data.to_vec());
        assert_eq!(store.get(0, path), Some(data.to_vec()));
        store.delete(0, &path);
        assert_eq!(store.get(0, path), None);
    }
}
