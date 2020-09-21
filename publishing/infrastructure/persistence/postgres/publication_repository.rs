use std::str::FromStr;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use tokio_postgres::row::Row;
use tokio_postgres::Client;
use uuid::Uuid;

use common::error::Error;
use common::model::{AggregateRoot, StatusHistory, StatusItem};
use common::result::Result;
use identity::domain::user::UserId;

use crate::domain::author::AuthorId;
use crate::domain::category::CategoryId;
use crate::domain::interaction::Comment;
use crate::domain::publication::{
    Header, Image, Name, Page, Publication, PublicationId, PublicationRepository, Statistics,
    Status, Synopsis, Tag,
};

#[derive(Debug, Serialize, Deserialize)]
struct StatusItemJson {
    status: String,
    admin_id: Option<String>,
    comment: Option<String>,
    datetime: String,
}

fn to_status_history(vec: Vec<StatusItemJson>) -> Result<StatusHistory<Status>> {
    let mut items = Vec::new();
    for item in vec.into_iter() {
        match item.status.as_ref() {
            "draft" => {
                items.push(StatusItem::build(
                    Status::Draft,
                    DateTime::from_str(&item.datetime).unwrap(),
                ));
            }
            "waiting-approval" => {
                items.push(StatusItem::build(
                    Status::WaitingApproval,
                    DateTime::from_str(&item.datetime).unwrap(),
                ));
            }
            "published" => {
                items.push(StatusItem::build(
                    Status::Published {
                        admin_id: UserId::new(item.admin_id.unwrap())?,
                        comment: Comment::new(item.comment.unwrap())?,
                    },
                    DateTime::from_str(&item.datetime).unwrap(),
                ));
            }
            "rejected" => {
                items.push(StatusItem::build(
                    Status::Rejected {
                        admin_id: UserId::new(item.admin_id.unwrap())?,
                        comment: Comment::new(item.comment.unwrap())?,
                    },
                    DateTime::from_str(&item.datetime).unwrap(),
                ));
            }
            _ => return Err(Error::new("publication_status", "invalid")),
        }
    }

    Ok(StatusHistory::build(items))
}

fn from_status_history(status_history: &StatusHistory<Status>) -> Result<Vec<StatusItemJson>> {
    let mut items = Vec::new();
    for item in status_history.history().iter() {
        let mut item_json = StatusItemJson {
            status: item.status().to_string(),
            admin_id: None,
            comment: None,
            datetime: item.date().to_rfc3339(),
        };

        match item.status() {
            Status::Published { admin_id, comment } | Status::Rejected { admin_id, comment } => {
                item_json.admin_id = Some(admin_id.to_string());
                item_json.comment = Some(comment.to_string());
            }
            _ => {}
        }

        items.push(item_json);
    }

    Ok(items)
}

impl Publication {
    fn from_row(row: Row) -> Result<Self> {
        let id: Uuid = row.get("id");
        let author_id: Uuid = row.get("author_id");

        let name: String = row.get("name");
        let synopsis: String = row.get("synopsis");
        let category_id: String = row.get("category_id");
        let tag_strs: Vec<String> = row.get("tags");
        let cover: String = row.get("cover");

        let contract: bool = row.get("contract");

        let statistics: Statistics = serde_json::from_value(row.get("statistics"))?;

        let status_history: Vec<StatusItemJson> =
            serde_json::from_value(row.get("status_history"))?;
        let status_history = to_status_history(status_history)?;

        let pages: Vec<Page> = serde_json::from_value(row.get("pages"))?;

        let created_at: DateTime<Utc> = row.get("created_at");
        let updated_at: Option<DateTime<Utc>> = row.get("updated_at");
        let deleted_at: Option<DateTime<Utc>> = row.get("deleted_at");

        let mut tags = Vec::new();
        for tag in tag_strs.into_iter() {
            tags.push(Tag::new(tag)?);
        }

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
            status_history,
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
    async fn find_all(&self) -> Result<Vec<Publication>> {
        let rows = self
            .client
            .query("SELECT * FROM publications", &[])
            .await
            .map_err(|err| Error::not_found("publications").wrap_raw(err))?;

        let mut publications = Vec::new();
        for row in rows.into_iter() {
            publications.push(Publication::from_row(row)?);
        }

        Ok(publications)
    }

    async fn find_by_id(&self, id: &PublicationId) -> Result<Publication> {
        let row = self
            .client
            .query_one(
                "SELECT * FROM publications
                WHERE id = $1",
                &[&id.to_uuid()?],
            )
            .await
            .map_err(|err| Error::not_found("publications").wrap_raw(err))?;

        Publication::from_row(row)
    }

    async fn search(
        &self,
        _author_id: Option<&AuthorId>,
        _category_id: Option<&CategoryId>,
        _status: Option<&String>,
        _name: Option<&String>,
    ) -> Result<Vec<Publication>> {
        self.find_all().await
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
        let status_history =
            serde_json::to_value(from_status_history(publication.status_history())?)?;
        let pages = serde_json::to_value(publication.pages())?;

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
                        statistics,
                        status_history,
                        pages,
                        created_at,
                        updated_at,
                        deleted_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
                    &[
                        &publication.base().id().to_uuid()?,
                        &publication.author_id().to_uuid()?,
                        &publication.header().name().value(),
                        &publication.header().synopsis().value(),
                        &publication.header().category_id().value(),
                        &publication
                            .header()
                            .tags()
                            .iter()
                            .map(|tag| tag.name())
                            .collect::<Vec<&str>>(),
                        &statistics,
                        &status_history,
                        &pages,
                        &publication.header().cover().url(),
                        &publication.base().created_at(),
                        &publication.base().updated_at(),
                        &publication.base().deleted_at(),
                    ],
                )
                .await
                .map_err(|err| Error::new("publication", "create").wrap_raw(err))?;
        } else {
        }

        Ok(())
    }
}
