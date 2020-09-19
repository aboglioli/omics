use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio_postgres::row::Row;
use tokio_postgres::Client;
use uuid::Uuid;

use common::error::Error;
use common::model::AggregateRoot;
use common::result::Result;

use crate::domain::category::{Category, CategoryId, CategoryRepository, Name};

impl Category {
    fn from_row(row: Row) -> Result<Self> {
        let id: Uuid = row.get("id");
        let name: String = row.get("name");

        let created_at: DateTime<Utc> = row.get("created_at");
        let updated_at: Option<DateTime<Utc>> = row.get("updated_at");
        let deleted_at: Option<DateTime<Utc>> = row.get("deleted_at");

        Ok(Category::build(
            AggregateRoot::build(
                CategoryId::new(id.to_string())?,
                created_at,
                updated_at,
                deleted_at,
            ),
            Name::new(name)?,
        ))
    }
}

pub struct PostgresCategoryRepository {
    client: Arc<Client>,
}

#[async_trait]
impl CategoryRepository for PostgresCategoryRepository {
    async fn find_all(&self) -> Result<Vec<Category>> {
        let rows = self
            .client
            .query("SELECT * FROM categories", &[])
            .await
            .map_err(|err| Error::not_found("category").wrap_raw(err))?;

        let mut categories = Vec::new();

        for row in rows.into_iter() {
            categories.push(Category::from_row(row)?);
        }

        Ok(categories)
    }

    async fn find_by_id(&self, id: &CategoryId) -> Result<Category> {
        let row = self
            .client
            .query_one("SELECT * FROM categories WHERE id = $1", &[&id.value()])
            .await
            .map_err(|err| Error::not_found("category").wrap_raw(err))?;

        Category::from_row(row)
    }

    async fn save(&self, category: &mut Category) -> Result<()> {
        let create = self
            .client
            .query_one(
                "SELECT * FROM categories WHERE id = $1",
                &[&category.base().id().value()],
            )
            .await
            .is_err();

        if create {
            self.client
                .execute(
                    "INSERT INTO categories(id, name, created_at, updated_at, deleted_at)",
                    &[
                        &category.base().id().value(),
                        &category.name().value(),
                        &category.base().created_at(),
                        &category.base().updated_at(),
                        &category.base().deleted_at(),
                    ],
                )
                .await
                .map_err(|err| Error::new("category", "create").wrap_raw(err))?;
        } else {
            self.client
                .execute(
                    "UPDATE categories
                    SET
                        name = $2
                        updated_at = $3
                        deleted_at = $4
                    WHERE
                        id = $1",
                    &[
                        &category.base().id().value(),
                        &category.name().value(),
                        &category.base().updated_at(),
                        &category.base().deleted_at(),
                    ],
                )
                .await
                .map_err(|err| Error::new("category", "update").wrap_raw(err))?;
        }

        Ok(())
    }
}
