use std::str::FromStr;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemJson {
    publication_id: String,
    datetime: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemsJson {
    items: Vec<ItemJson>,
}

impl ItemsJson {
    fn to_items(self) -> Result<Vec<Item>> {
        let mut items = Vec::new();
        for item in self.items.into_iter() {
            items.push(Item::build(
                PublicationId::new(item.publication_id)?,
                DateTime::from_str(&item.datetime).unwrap(),
            ));
        }

        Ok(items)
    }

    fn from_items(items: &[Item]) -> Result<ItemsJson> {
        let mut items_json = Vec::new();
        for item in items.iter() {
            items_json.push(ItemJson {
                publication_id: item.publication_id().to_string(),
                datetime: item.date().to_rfc3339(),
            })
        }

        Ok(ItemsJson { items: items_json })
    }
}

impl Collection {
    fn from_row(row: Row) -> Result<Self> {
        let id: Uuid = row.get("id");
        let author_id: Uuid = row.get("author_id");

        let name: String = row.get("name");
        let synopsis: String = row.get("synopsis");
        let category_id: String = row.get("category_id");
        let tag_strs: Vec<String> = row.get("tags");
        let cover: String = row.get("cover");

        // let items: Value = row.get("items");
        // let items: ItemsJson = serde_json::from_value(items).unwrap();
        // let items = items.to_items()?;
        let items: Vec<Item> = serde_json::from_value(row.get("items"))?;

        let created_at: DateTime<Utc> = row.get("created_at");
        let updated_at: Option<DateTime<Utc>> = row.get("updated_at");
        let deleted_at: Option<DateTime<Utc>> = row.get("deleted_at");

        let mut tags = Vec::new();
        for tag in tag_strs.into_iter() {
            tags.push(Tag::new(tag)?);
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
            .query("SELECT * FROM collections", &[])
            .await
            .map_err(|err| Error::not_found("collection").wrap_raw(err))?;

        let mut collections = Vec::new();
        for row in rows.into_iter() {
            collections.push(Collection::from_row(row)?);
        }

        Ok(collections)
    }

    async fn find_by_id(&self, id: &CollectionId) -> Result<Collection> {
        let row = self
            .client
            .query_one(
                "SELECT * FROM collections
                WHERE id = $1",
                &[&id.to_uuid()?],
            )
            .await
            .map_err(|err| Error::not_found("collection").wrap_raw(err))?;

        Collection::from_row(row)
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

        let items = ItemsJson::from_items(collection.items())?;
        let items = serde_json::to_value(items).unwrap();

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
                        items,
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
                        &items,
                        &collection.base().created_at(),
                        &collection.base().updated_at(),
                        &collection.base().deleted_at(),
                    ],
                )
                .await
                .map_err(|err| Error::new("collection", "create").wrap_raw(err))?;
        } else {
        }

        Ok(())
    }
}
