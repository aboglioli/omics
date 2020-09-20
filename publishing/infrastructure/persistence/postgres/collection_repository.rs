use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio_postgres::row::Row;
use tokio_postgres::Client;
use uuid::Uuid;

use common::error::Error;
use common::model::AggregateRoot;
use common::result::Result;

use crate::domain::author::AuthorId;
use crate::domain::category::CategoryId;
use crate::domain::collection::{Collection, CollectionId, CollectionRepository, Item};
use crate::domain::publication::{Header, Image, Name, PublicationId, Synopsis, Tag};

impl Item {
    fn from_row(row: Row) -> Result<Self> {
        let publication_id: Uuid = row.get("publication_id");
        let date: DateTime<Utc> = row.get("datetime");

        Ok(Item::build(
            PublicationId::new(publication_id.to_string())?,
            date,
        ))
    }
}

impl Collection {
    fn from_rows(rows: Vec<Row>) -> Result<Self> {
        if rows.is_empty() {
            return Err(Error::new("collection", "empty"));
        }

        let row = &rows[0];

        let id: Uuid = row.get("id");
        let author_id: Uuid = row.get("author_id");

        let name: String = row.get("name");
        let synopsis: String = row.get("synopsis");
        let category_id: String = row.get("category_id");
        let tag_strs: Vec<String> = row.get("tags");
        let cover: String = row.get("cover");

        let created_at: DateTime<Utc> = row.get("created_at");
        let updated_at: Option<DateTime<Utc>> = row.get("updated_at");
        let deleted_at: Option<DateTime<Utc>> = row.get("deleted_at");

        let mut tags = Vec::new();
        for tag in tag_strs.into_iter() {
            tags.push(Tag::new(tag)?);
        }

        let mut items = Vec::new();
        for row in rows.into_iter() {
            items.push(Item::from_row(row)?);
        }

        Ok(Collection::build(
            AggregateRoot::build(
                CollectionId::new(id.to_string())?,
                created_at,
                updated_at,
                deleted_at,
            ),
            AuthorId::new(author_id.to_string())?,
            Header::new(
                Name::new(name)?,
                Synopsis::new(synopsis)?,
                CategoryId::new(category_id)?,
                tags,
                Image::new(cover)?,
            )?,
            items,
        ))
    }
}

pub struct PostgresCollectionRepository {
    client: Arc<Client>,
}

impl PostgresCollectionRepository {
    pub fn new(client: Arc<Client>) -> Self {
        PostgresCollectionRepository { client }
    }
}

#[async_trait]
impl CollectionRepository for PostgresCollectionRepository {
    async fn find_all(&self) -> Result<Vec<Collection>> {
        let rows = self
            .client
            .query(
                "SELECT * FROM collections
                LEFT JOIN collection_items AS ci ON ci.collection_id = id",
                &[],
            )
            .await
            .map_err(|err| Error::not_found("collection").wrap_raw(err))?;

        let mut collection_rows: HashMap<Uuid, Vec<Row>> = HashMap::new();
        for row in rows.into_iter() {
            let id: Uuid = row.get("id");
            match collection_rows.get_mut(&id) {
                Some(rows) => {
                    rows.push(row);
                }
                None => {
                    let rows = vec![row];
                    collection_rows.insert(id, rows);
                }
            }
        }

        let mut collections = Vec::new();
        for (_, rows) in collection_rows.into_iter() {
            collections.push(Collection::from_rows(rows)?);
        }

        Ok(collections)
    }

    async fn find_by_id(&self, id: &CollectionId) -> Result<Collection> {
        let row = self
            .client
            .query_one(
                "SELECT * FROM collections
                LEFT JOIN collection_items AS ci ON ci.collection_id = id
                WHERE id = $1",
                &[&id.value()],
            )
            .await
            .map_err(|err| Error::not_found("collection").wrap_raw(err))?;

        Collection::from_rows(vec![row])
    }

    async fn search(
        &self,
        _author_id: Option<&AuthorId>,
        _category_id: Option<&CategoryId>,
        _publication_id: Option<&PublicationId>,
        _name: Option<&String>,
    ) -> Result<Vec<Collection>> {
        self.find_all().await
    }

    async fn save(&self, collection: &mut Collection) -> Result<()> {
        let create = self
            .client
            .query_one(
                "SELECT * FROM collections WHERE id = $1",
                &[&collection.base().id().to_uuid()?],
            )
            .await
            .is_err();

        if create {
            self.client
                .execute(
                    "INSERT INTO collections(
                        id,
                        author_id,
                        name,
                        synopsis,
                        category_id,
                        tags,
                        cover,
                        created_at,
                        updated_at,
                        deleted_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
                    &[
                        &collection.base().id().to_uuid()?,
                        &collection.author_id().to_uuid()?,
                        &collection.header().name().value(),
                        &collection.header().synopsis().value(),
                        &collection.header().category_id().value(),
                        &collection
                            .header()
                            .tags()
                            .iter()
                            .map(|tag| tag.name())
                            .collect::<Vec<&str>>(),
                        &collection.header().cover().url(),
                        &collection.base().created_at(),
                        &collection.base().updated_at(),
                        &collection.base().deleted_at(),
                    ],
                )
                .await
                .map_err(|err| Error::new("collection", "create").wrap_raw(err))?;

            for item in collection.items().iter() {
                self.client
                    .execute(
                        "INSERT INTO collection_items(
                            collection_id,
                            publication_id,
                            datetime
                        ) VALUES (
                            $1,
                            $2,
                            $3
                        )",
                        &[
                            &collection.base().id().to_uuid()?,
                            &item.publication_id().to_uuid()?,
                            &item.date(),
                        ],
                    )
                    .await
                    .map_err(|err| Error::new("collection_item", "create").wrap_raw(err))?;
            }
        } else {
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use common::config::Config;
    use tokio_postgres::NoTls;

    #[tokio::test]
    async fn all() {
        let config = Config::get();
        let (client, connection) = tokio_postgres::connect(
            &format!(
                "host={} user={} password={} dbname={}",
                config.postgres_host(),
                config.postgres_username(),
                config.postgres_password(),
                config.postgres_database()
            ),
            NoTls,
        )
        .await
        .unwrap();

        tokio::spawn(async move {
            if let Err(err) = connection.await {
                eprintln!("error: {}", err);
            }
        });

        let _repo = PostgresCollectionRepository::new(Arc::new(client));
    }
}
