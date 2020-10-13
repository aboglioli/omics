use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio_postgres::row::Row;
use tokio_postgres::Client;
use uuid::Uuid;

use common::error::Error;
use common::model::AggregateRoot;
use common::result::Result;
use common::sql::where_builder::WhereBuilder;

use crate::domain::author::AuthorId;
use crate::domain::category::CategoryId;
use crate::domain::collection::{Collection, CollectionId, CollectionRepository, Item};
use crate::domain::publication::{Header, Image, Name, PublicationId, Synopsis, Tag};

impl Collection {
    fn from_row(row: Row) -> Result<Self> {
        let id: Uuid = row.get("id");
        let author_id: Uuid = row.get("author_id");

        let name: String = row.get("name");
        let synopsis: String = row.get("synopsis");
        let category_id: String = row.get("category_id");
        let tags: Vec<Tag> = serde_json::from_value(row.get("tags"))?;
        let cover: String = row.get("cover");

        let items: Vec<Item> = serde_json::from_value(row.get("items"))?;

        let created_at: DateTime<Utc> = row.get("created_at");
        let updated_at: Option<DateTime<Utc>> = row.get("updated_at");
        let deleted_at: Option<DateTime<Utc>> = row.get("deleted_at");

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

    // SELECT *
    // FROM mytable
    // WHERE EXISTS (
    //     SELECT TRUE
    //     FROM jsonb_array_elements(data->'tags') x
    //     WHERE x->>'name' IN ('tag2', 'tag3')
    // )
    async fn search(
        &self,
        author_id: Option<&AuthorId>,
        category_id: Option<&CategoryId>,
        publication_id: Option<&PublicationId>,
        tag: Option<&Tag>,
        name: Option<&String>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
        offset: Option<usize>,
        limit: Option<usize>,
    ) -> Result<Vec<Collection>> {
        let author_id = author_id.map(|id| id.to_uuid()).transpose()?;
        let category_id = category_id.map(|id| id.value());
        let publication_id = publication_id.map(|id| id.value());
        let tag = tag.map(|t| t.slug());
        let offset = offset.unwrap_or_else(|| 0);
        let limit = limit
            .map(|l| if l <= 1000 { l } else { 1000 })
            .unwrap_or_else(|| 100);

        let (mut sql, params) = WhereBuilder::new()
            .add_param_opt("author_id = $$", &author_id, author_id.is_some())
            .add_param_opt("category_id = $$", &category_id, category_id.is_some())
            .add_param_opt(
                "to_json(array(select jsonb_array_elements(items)->'publication_id'->>'id'))::jsonb
                    ?| array[$$]",
                &publication_id,
                publication_id.is_some(),
            )
            .add_param_opt(
                "EXISTS (
                    SELECT TRUE
                    FROM jsonb_array_elements(tags) tag
                    WHERE tag->>'slug' = $$
                )",
                &tag,
                tag.is_some(),
            )
            .add_param_opt(
                "LOWER(name) LIKE '%' || LOWER($$) || '%'",
                &name,
                name.is_some(),
            )
            .add_param_opt("created_at >= $$", &from, from.is_some())
            .add_param_opt("created_at <= $$", &to, to.is_some())
            .build();

        sql = format!(
            "SELECT * FROM collections
            {}
            ORDER BY created_at ASC
            OFFSET {}
            LIMIT {}",
            sql, offset, limit,
        );

        let rows = self
            .client
            .query(&sql as &str, &params)
            .await
            .map_err(|err| Error::not_found("collection").wrap_raw(err))?;

        let mut collections = Vec::new();
        for row in rows.into_iter() {
            collections.push(Collection::from_row(row)?);
        }

        Ok(collections)
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

        let tags = serde_json::to_value(collection.header().tags())?;
        let items = serde_json::to_value(collection.items())?;

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
                        created_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
                    &[
                        &collection.base().id().to_uuid()?,
                        &collection.author_id().to_uuid()?,
                        &collection.header().name().value(),
                        &collection.header().synopsis().value(),
                        &collection.header().category_id().value(),
                        &tags,
                        &collection.header().cover().url(),
                        &items,
                        &collection.base().created_at(),
                    ],
                )
                .await
                .map_err(|err| Error::new("collection", "create").wrap_raw(err))?;
        } else {
            self.client
                .execute(
                    "UPDATE collections
                    SET
                        name = $2,
                        synopsis = $3,
                        category_id = $4,
                        tags = $5,
                        cover = $6,
                        items = $7,
                        updated_at = $8,
                        deleted_at = $9
                    WHERE
                        id = $1",
                    &[
                        &collection.base().id().to_uuid()?,
                        &collection.header().name().value(),
                        &collection.header().synopsis().value(),
                        &collection.header().category_id().value(),
                        &tags,
                        &collection.header().cover().url(),
                        &items,
                        &collection.base().updated_at(),
                        &collection.base().deleted_at(),
                    ],
                )
                .await
                .map_err(|err| Error::new("collection", "update").wrap_raw(err))?;
        }

        Ok(())
    }

    async fn delete(&self, id: &CollectionId) -> Result<()> {
        self.client
            .execute(
                "DELETE FROM collections
                WHERE id = $1",
                &[&id.to_uuid()?],
            )
            .await
            .map_err(|err| Error::new("collection", "delete").wrap_raw(err))?;

        Ok(())
    }
}
