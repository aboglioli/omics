use async_trait::async_trait;

use crate::error::Error;

#[async_trait]
pub trait Cache<K, V> {
    async fn get(&self, k: &K) -> Option<V>;
    async fn set(&self, k: K, v: V) -> Result<(), Error>;
    async fn delete(&self, k: &K) -> Result<(), Error>;
}
