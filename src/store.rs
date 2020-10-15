//use merk;
use std::collections::HashMap;

pub trait Storage {
    /// Return None if there is no block matching `height`.
    fn set(&mut self, height: u64, path: Vec<u8>, value: Vec<u8>) -> Option<()>;

    fn get(&self, height: u64, path: &[u8]) -> Option<&[u8]>;

    /// Return None is there is no block matchin `height`;
    fn delete(&mut self, height: u64, path: &[u8]) -> Option<()>;
}

pub struct InMemoryStore {
    store: Vec<HashMap<Vec<u8>, Vec<u8>>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        let genesis = HashMap::new();
        InMemoryStore {
            store: vec![genesis],
        }
    }
}

impl Storage for InMemoryStore {
    fn set(&mut self, height: u64, path: Vec<u8>, value: Vec<u8>) -> Option<()> {
        let store = self.store.get_mut(height as usize)?;
        store.insert(path, value);
        Some(())
    }

    fn get(&self, height: u64, path: &[u8]) -> Option<&[u8]> {
        let store = self.store.get(height as usize)?;
        store.get(path).map(|x| &**x)
    }

    fn delete(&mut self, height: u64, path: &[u8]) -> Option<()> {
        let store = self.store.get_mut(height as usize)?;
        store.remove(path);
        Some(())
    }
}

/*
 * This merk store implementation does not yet support versionning.
 *

pub struct MerkStore {
    store: merk::Merk,
}

impl MerkStore {
    pub fn new() -> Self {
        let store = merk::Merk::open("tendermock.db").unwrap();
        MerkStore { store }
    }
}

impl Storage for MerkStore {
    fn set(&mut self, path: Vec<u8>, value: Vec<u8>) {
        self.store
            .apply(&[(path, merk::Op::Put(value))], &[])
            .unwrap();
    }

    fn get(&mut self, path: &[u8]) -> Option<Vec<u8>> {
        self.store.get(path).unwrap()
    }

    fn delete(&mut self, path: Vec<u8>) {
        self.store.apply(&[(path, merk::Op::Delete)], &[]).unwrap();
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store() {
        let store = InMemoryStore::new();
        test_with_store(store)
    }

    fn test_with_store<T: Storage>(mut store: T) {
        let data = b"hello";
        let path = b"foo/bar";
        let data = &data[..];
        let path = &path[..];

        assert_eq!(store.get(0, path), None);
        store.set(0, path.to_vec(), data.to_vec());
        assert_eq!(store.get(0, path), Some(data));
        store.delete(0, &path);
        assert_eq!(store.get(0, path), None);
    }
}
