use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

use crate::cache::Cache;
use crate::error::Error;

pub struct InMemCache<K, V> {
    data: RefCell<HashMap<K, V>>,
}

impl<K, V> InMemCache<K, V> {
    pub fn new() -> InMemCache<K, V> {
        InMemCache {
            data: RefCell::new(HashMap::new()),
        }
    }

    pub fn len(&self) -> usize {
        self.data.borrow().len()
    }
}

impl<K: Hash + Eq, V: Clone> Cache<K, V> for InMemCache<K, V> {
    fn get(&self, k: &K) -> Option<V> {
        let cache = self.data.borrow();
        cache.get(k).cloned()
    }

    fn set(&self, k: K, v: V) -> Result<(), Error> {
        let mut cache = self.data.borrow_mut();
        cache.insert(k, v);
        Ok(())
    }

    fn delete(&self, k: &K) -> Result<(), Error> {
        let mut cache = self.data.borrow_mut();
        cache.remove(k);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize() {
        let c: InMemCache<u8, u8> = InMemCache::new();
        assert_eq!(c.len(), 0);
    }

    #[test]
    fn get_set_delete() {
        let c = InMemCache::new();
        let k = "key".to_owned();
        assert!(c.get(&k).is_none());
        assert!(c.set(k.clone(), 123).is_ok());
        assert_eq!(c.get(&k).unwrap(), 123);
        assert!(c.delete(&k).is_ok());
        assert!(c.get(&k).is_none());
    }
}
