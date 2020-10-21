use std::str::FromStr;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use common::error::Error;
use common::model::Pagination;
use common::result::Result;

use crate::domain::author::AuthorId;
use crate::domain::category::CategoryId;
use crate::domain::publication::{Publication, PublicationId, Status, Tag};

#[async_trait]
pub trait PublicationRepository: Sync + Send {
    async fn next_id(&self) -> Result<PublicationId> {
        PublicationId::new(Uuid::new_v4().to_string())
    }

    async fn find_by_id(&self, id: &PublicationId) -> Result<Publication>;
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
    ) -> Result<Pagination<Publication>>;

    async fn save(&self, publication: &mut Publication) -> Result<()>;

    async fn delete(&self, id: &PublicationId) -> Result<()>;
}

pub enum PublicationOrderBy {
    Oldest,
    Newest,
    MostViewed,
    MostLiked,
    BestReviews,
}

impl FromStr for PublicationOrderBy {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "newest" => PublicationOrderBy::Newest,
            "most_viewed" => PublicationOrderBy::MostViewed,
            "most_liked" => PublicationOrderBy::MostLiked,
            "best_reviews" => PublicationOrderBy::BestReviews,
            _ => PublicationOrderBy::Oldest,
        })
    }
}
