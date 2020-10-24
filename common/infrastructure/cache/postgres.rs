use std::sync::Arc;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;

use tokio_postgres::types::ToSql;
use tokio_postgres::Client;

use crate::cache::Cache;
use crate::error::Error;
use crate::result::Result;

pub struct PostgresCache {
    client: Arc<Client>,
}

impl PostgresCache {
    pub fn new(client: Arc<Client>) -> Self {
        PostgresCache { client }
    }
}

#[async_trait]
impl<K, V> Cache<K, V> for PostgresCache
where
    K: ToSql + Sync + Send + 'static,
    V: Serialize + DeserializeOwned + Sync + Send + 'static,
{
    async fn get(&self, k: &K) -> Option<V> {
        let row = self
            .client
            .query_one(
                "SELECT value FROM cache
                WHERE key = $1",
                &[&k],
            )
            .await
            .ok();

        if let Some(row) = row {
            if let Ok(value) = row.try_get("value") {
                if let Ok(value) = serde_json::from_value(value) {
                    return Some(value);
                }
            }
        }

        None
    }

    async fn set(&self, k: K, v: V) -> Result<()> {
        let create = self
            .client
            .query_one(
                "SELECT value FROM cache
                WHERE key = $1",
                &[&k],
            )
            .await
            .is_err();

        let value = serde_json::to_value(v)?;

        if create {
            self.client
                .execute(
                    "INSERT INTO cache(key, value)
                    VALUES ($1, $2)",
                    &[&k, &value],
                )
                .await
                .map_err(|err| Error::new("cache", "create").wrap_raw(err))?;
        } else {
            self.client
                .execute(
                    "UPDATE cache
                    SET
                        value = $2
                    WHERE key = $1",
                    &[&k, &value],
                )
                .await
                .map_err(|err| Error::new("cache", "update").wrap_raw(err))?;
        }

        Ok(())
    }

    async fn delete(&self, k: &K) -> Result<()> {
        self.client
            .execute(
                "DELETE FROM cache
                WHERE key = $1",
                &[&k],
            )
            .await
            .map_err(|err| Error::new("cache", "delete").wrap_raw(err))?;

        Ok(())
    }
}
