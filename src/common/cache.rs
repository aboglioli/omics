use crate::common::error::Error;

pub trait Cache<K, V> {
    fn get(&self, k: &K) -> Option<V>;
    fn set(&self, k: K, v: V) -> Result<(), Error>;
    fn delete(&self, k: &K) -> Result<(), Error>;
}
