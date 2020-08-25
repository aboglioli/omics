use std::collections::HashMap;
use std::hash::Hash;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::cache::Cache;

use crate::result::Result;

#[derive(Default)]
pub struct InMemCache<K, V> {
    data: Mutex<HashMap<K, V>>,
}

impl<K, V: Clone> InMemCache<K, V> {
    pub fn new() -> InMemCache<K, V> {
        InMemCache {
            data: Mutex::new(HashMap::new()),
        }
    }

    pub async fn find<P>(&self, predicate: P) -> Option<V>
    where
        P: FnMut(&(&K, &V)) -> bool,
    {
        self.data
            .lock()
            .await
            .iter()
            .find(predicate)
            .map(|(_, v)| v.clone())
    }

    pub async fn filter<P>(&self, predicate: P) -> Vec<V>
    where
        P: FnMut(&(&K, &V)) -> bool,
    {
        self.data
            .lock()
            .await
            .iter()
            .filter(predicate)
            .map(|(_, v)| v.clone())
            .collect()
    }

    pub async fn all(&self) -> Vec<V> {
        self.filter(|_| true).await
    }

    pub async fn len(&self) -> usize {
        self.data.lock().await.len()
    }

    pub async fn is_empty(&self) -> bool {
        self.data.lock().await.is_empty()
    }
}

#[async_trait]
impl<K: Hash + Eq + Send + Sync, V: Clone + Send + Sync> Cache<K, V> for InMemCache<K, V> {
    async fn get(&self, k: &K) -> Option<V> {
        let cache = self.data.lock().await;
        cache.get(k).cloned()
    }

    async fn set(&self, k: K, v: V) -> Result<()> {
        let mut cache = self.data.lock().await;
        cache.insert(k, v);
        Ok(())
    }

    async fn delete(&self, k: &K) -> Result<()> {
        let mut cache = self.data.lock().await;
        cache.remove(k);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn initialize() {
        let c: InMemCache<u8, u8> = InMemCache::new();
        assert_eq!(c.data.lock().await.len(), 0);
    }

    #[tokio::test]
    async fn get_set_delete() {
        let c = InMemCache::new();
        let k = "key".to_owned();
        assert!(c.get(&k).await.is_none());
        assert!(c.set(k.clone(), 123).await.is_ok());
        assert_eq!(c.get(&k).await.unwrap(), 123);
        assert!(c.delete(&k).await.is_ok());
        assert!(c.get(&k).await.is_none());
    }

    #[tokio::test]
    async fn find() {
        let c = InMemCache::new();
        c.set("one", 1u8).await.unwrap();
        c.set("two", 2).await.unwrap();
        c.set("three", 3).await.unwrap();
        c.set("four", 4).await.unwrap();
        c.set("five", 5).await.unwrap();

        let res = c.find(|&(k, _)| *k == "four").await.unwrap();
        assert_eq!(res, 4);

        let res = c.find(|&(_, v)| *v > 4).await.unwrap();
        assert!(res > 4);

        assert!(c.find(|&(_, v)| *v > 10).await.is_none());
    }

    #[tokio::test]
    async fn filter() {
        let c = InMemCache::new();
        c.set("one", 1u8).await.unwrap();
        c.set("two", 2).await.unwrap();
        c.set("three", 3).await.unwrap();
        c.set("four", 4).await.unwrap();
        c.set("five", 5).await.unwrap();

        let res = c.filter(|&(k, _)| *k == "four").await;
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], 4);

        let res = c.filter(|&(_, v)| *v > 2).await;
        assert_eq!(res.len(), 3);
    }
}
