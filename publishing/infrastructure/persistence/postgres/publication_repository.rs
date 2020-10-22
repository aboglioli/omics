use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio_postgres::row::Row;
use tokio_postgres::Client;
use uuid::Uuid;

use common::error::Error;
use common::model::{AggregateRoot, Pagination, StatusHistory, StatusItem};
use common::result::Result;
use common::sql::where_builder::WhereBuilder;

use crate::domain::author::AuthorId;
use crate::domain::category::CategoryId;

use crate::domain::publication::{
    Header, Image, Name, Page, Publication, PublicationId, PublicationOrderBy,
    PublicationRepository, Statistics, Status, Synopsis, Tag,
};

impl Publication {
    fn from_row(row: Row) -> Result<Self> {
        let id: Uuid = row.get("id");
        let author_id: Uuid = row.get("author_id");

        let name: String = row.get("name");
        let synopsis: String = row.get("synopsis");
        let category_id: String = row.get("category_id");
        let tags: Vec<Tag> = serde_json::from_value(row.get("tags"))?;
        let cover: String = row.get("cover");

        let contract: bool = row.get("contract");

        let statistics: Statistics = serde_json::from_value(row.get("statistics"))?;

        let status_items: Vec<StatusItem<Status>> =
            serde_json::from_value(row.get("status_history"))?;

        let pages: Vec<Page> = serde_json::from_value(row.get("pages"))?;

        let created_at: DateTime<Utc> = row.get("created_at");
        let updated_at: Option<DateTime<Utc>> = row.get("updated_at");
        let deleted_at: Option<DateTime<Utc>> = row.get("deleted_at");

        Ok(Publication::build(
            AggregateRoot::build(
                PublicationId::new(id.to_string())?,
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
            pages,
            contract,
            statistics,
            StatusHistory::build(status_items),
        ))
    }
}

pub struct PostgresPublicationRepository {
    client: Arc<Client>,
}

impl PostgresPublicationRepository {
    pub fn new(client: Arc<Client>) -> Self {
        PostgresPublicationRepository { client }
    }
}

#[async_trait]
impl PublicationRepository for PostgresPublicationRepository {
    async fn find_by_id(&self, id: &PublicationId) -> Result<Publication> {
        let row = self
            .client
            .query_one(
                "SELECT * FROM publications
                WHERE id = $1",
                &[&id.to_uuid()?],
            )
            .await
            .map_err(|err| Error::not_found("publication").wrap_raw(err))?;

        Publication::from_row(row)
    }

    async fn search(
        &self,
        author_id: Option<&AuthorId>,
        category_id: Option<&CategoryId>,
        tag: Option<&Tag>,
        status: Option<&Status>,
        name: Option<&String>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
        offset: Option<usize>,
        limit: Option<usize>,
        order_by: Option<&PublicationOrderBy>,
    ) -> Result<Pagination<Publication>> {
        let author_id = author_id.map(|id| id.to_uuid()).transpose()?;
        let category_id = category_id.map(|id| id.value());
        let tag = tag.map(|t| t.slug());
        let status = status.map(|s| s.to_string());

        let (sql, params) = WhereBuilder::new()
            .add_param_opt("author_id = $$", &author_id, author_id.is_some())
            .add_param_opt("category_id = $$", &category_id, category_id.is_some())
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
                "status_history->-1->>'status' = $$",
                &status,
                status.is_some(),
            )
            .add_param_opt(
                "LOWER(name) LIKE '%' || LOWER($$) || '%'",
                &name,
                name.is_some(),
            )
            .add_param_opt("created_at >= $$", &from, from.is_some())
            .add_param_opt("created_at <= $$", &to, to.is_some())
            .build();

        // Total
        let row = self
            .client
            .query_one(&format!("SELECT COUNT(*) FROM publications") as &str, &[])
            .await
            .map_err(|err| Error::new("publication", "total").wrap_raw(err))?;
        let total: i64 = row.get(0);

        // Matching criteria
        let row = self
            .client
            .query_one(
                &format!(
                    "SELECT COUNT(*) FROM publications
                    {}",
                    sql,
                ) as &str,
                &params,
            )
            .await
            .map_err(|err| Error::new("publication", "matching_criteria").wrap_raw(err))?;
        let matching_criteria: i64 = row.get(0);

        // Query
        let offset = offset.unwrap_or_else(|| 0);
        let limit = limit.unwrap_or_else(|| total as usize);
        let order_by = match order_by {
            Some(PublicationOrderBy::Newest) => "created_at DESC",
            Some(PublicationOrderBy::MostViewed) => "statistics->'views' DESC",
            Some(PublicationOrderBy::MostLiked) => "statistics->'likes' DESC",
            Some(PublicationOrderBy::BestReviews) => "statistics->'stars' DESC",
            _ => "created_at ASC",
        };

        let rows = self
            .client
            .query(
                &format!(
                    "SELECT * FROM publications
                    {}
                    ORDER BY {}
                    OFFSET {}
                    LIMIT {}",
                    sql, order_by, offset, limit,
                ) as &str,
                &params,
            )
            .await
            .map_err(|err| Error::not_found("publication").wrap_raw(err))?;

        let mut publications = Vec::new();
        for row in rows.into_iter() {
            publications.push(Publication::from_row(row)?);
        }

        Ok(
            Pagination::new(offset, limit, total as usize, matching_criteria as usize)
                .add_items(publications),
        )
    }

    async fn save(&self, publication: &mut Publication) -> Result<()> {
        let create = self
            .client
            .query_one(
                "SELECT * FROM publications WHERE id = $1",
                &[&publication.base().id().to_uuid()?],
            )
            .await
            .is_err();

        let statistics = serde_json::to_value(publication.statistics())?;
        let status_history = serde_json::to_value(publication.status_history().history())?;
        let pages = serde_json::to_value(publication.pages())?;
        let tags = serde_json::to_value(publication.header().tags())?;

        if create {
            self.client
                .execute(
                    "INSERT INTO publications(
                        id,
                        author_id,
                        name,
                        synopsis,
                        category_id,
                        tags,
                        cover,
                        contract,
                        statistics,
                        pages,
                        status_history,
                        created_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
                    &[
                        &publication.base().id().to_uuid()?,
                        &publication.author_id().to_uuid()?,
                        &publication.header().name().value(),
                        &publication.header().synopsis().value(),
                        &publication.header().category_id().value(),
                        &tags,
                        &publication.header().cover().url(),
                        &publication.has_contract(),
                        &statistics,
                        &pages,
                        &status_history,
                        &publication.base().created_at(),
                    ],
                )
                .await
                .map_err(|err| Error::new("publication", "create").wrap_raw(err))?;
        } else {
            self.client
                .execute(
                    "UPDATE publications
                    SET
                        name = $2,
                        synopsis = $3,
                        category_id = $4,
                        tags = $5,
                        cover = $6,
                        contract = $7,
                        statistics = $8,
                        pages = $9,
                        status_history = $10,
                        updated_at = $11,
                        deleted_at = $12
                    WHERE
                        id = $1",
                    &[
                        &publication.base().id().to_uuid()?,
                        &publication.header().name().value(),
                        &publication.header().synopsis().value(),
                        &publication.header().category_id().value(),
                        &tags,
                        &publication.header().cover().url(),
                        &publication.has_contract(),
                        &statistics,
                        &pages,
                        &status_history,
                        &publication.base().updated_at(),
                        &publication.base().deleted_at(),
                    ],
                )
                .await
                .map_err(|err| Error::new("publication", "update").wrap_raw(err))?;
        }

        Ok(())
    }

    async fn delete(&self, id: &PublicationId) -> Result<()> {
        self.client
            .execute(
                "DELETE FROM publications
                WHERE id = $1",
                &[&id.to_uuid()?],
            )
            .await
            .map_err(|err| Error::new("publication", "delete").wrap_raw(err))?;

        Ok(())
    }
}
