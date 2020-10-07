use merk;

pub struct Store {
    store: merk::Merk,
}

impl Store {
    pub fn new() -> Self {
        let store = merk::Merk::open("tendermock.db").unwrap();
        Store { store }
    }

    pub fn set(&mut self, path: Vec<u8>, value: Vec<u8>) {
        self.store
            .apply(&[(path, merk::Op::Put(value))], &[])
            .unwrap();
    }

    pub fn get(&mut self, path: &[u8]) -> Option<Vec<u8>> {
        self.store.get(path).unwrap()
    }

    pub fn delete(&mut self, path: Vec<u8>) {
        self.store.apply(&[(path, merk::Op::Delete)], &[]).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store() {
        let mut store = Store::new();
        let data = b"hello".to_vec();
        let path = b"foo/bar".to_vec();

        assert_eq!(store.get(&path), None);
        store.set(path.clone(), data.clone());
        assert_eq!(store.get(&path), Some(data.clone()));
        store.delete(path.clone());
        assert_eq!(store.get(&path), None);
    }
}
