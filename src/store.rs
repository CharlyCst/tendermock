//! # Store
//!
//! A storage for tendermock. For now the only available storage is the `InMemoryStore`, which ,as
//! its name implies, is not persisted to the hard drive. However, possible implementation of
//! persistent storage are possible without impacting the rest of the code base as it only relies
//! on the `Storage` trait, which may be implemented for new kinds of storage in the future.
//!
//! A storage has two jobs:
//!  - persist the state of commited blocks.
//!  - updating the state of the pending block.
use crate::avl::AvlTree;
use std::sync::RwLock;

/// A concurrent, on chain storage using interior mutability.
pub trait Storage: std::fmt::Debug {
    /// Set a value in the store at the last (pending) height.
    fn set(&self, path: Vec<u8>, value: Vec<u8>);
    /// Return None if there is no block matching `height`.
    fn get(&self, height: u64, path: &[u8]) -> Option<Vec<u8>>;
    /// Freeze the pending store by adding it to the commited chain and create a new pending.
    fn grow(&self);
}

/// An in-memory store backed by a simple hashmap.
pub struct InMemoryStore {
    store: RwLock<Vec<AvlTree<Vec<u8>, Vec<u8>>>>,
    pending: RwLock<AvlTree<Vec<u8>, Vec<u8>>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        let genesis = AvlTree::new();
        let pending = genesis.clone();
        InMemoryStore {
            store: RwLock::new(vec![genesis]),
            pending: RwLock::new(pending),
        }
    }
}

impl std::fmt::Debug for InMemoryStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let store = self.store.read().unwrap();
        let keys = store.last().unwrap().get_keys();
        write!(
            f,
            "InMemoryStore {{ height: {}, keys: [{}] }}",
            store.len(),
            keys.iter()
                .map(|k| String::from_utf8_lossy(k).into_owned())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Storage for InMemoryStore {
    fn set(&self, path: Vec<u8>, value: Vec<u8>) {
        let mut store = self.pending.write().unwrap();
        store.insert(path, value);
    }

    /// Three cases:
    ///  - height = 0 -> last commited block
    ///  - height - 1 < store.len() -> the block n° (height-1)
    ///  - height - 1 == store.len() -> the pending block
    fn get(&self, height: u64, path: &[u8]) -> Option<Vec<u8>> {
        let store = self.store.read().unwrap();
        if height == 0 {
            // Access last commited block
            return store.last().unwrap().get(path).cloned();
        }
        let h = (height - 1) as usize;
        if h < store.len() {
            // Access one of the commited blocks
            let state = store.get(h).unwrap();
            state.get(path).cloned()
        } else if h == store.len() {
            // Access the pending blocks
            drop(store); // Release lock
            let pending = self.pending.read().unwrap();
            pending.get(path).cloned()
        } else {
            None
        }
    }

    fn grow(&self) {
        let mut store = self.store.write().unwrap();
        let pending = self.pending.write().unwrap();
        let pending_copy = pending.clone();
        store.push(pending_copy);
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
        store.set(path.to_vec(), data.to_vec()); // Set value on pending block (height 2 here)
        assert_eq!(store.get(0, path), None);
        assert_eq!(store.get(2, path), Some(data.to_vec()));
        store.grow(); // Commit value, will be seen as "last block" (height 0)
        assert_eq!(store.get(0, path), Some(data.to_vec()));
    }
}
