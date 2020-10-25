use std::str::FromStr;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use common::error::Error;
use common::model::Pagination;
use common::result::Result;
use identity::domain::user::UserId;

use crate::domain::donation::{Donation, DonationId, Status};

#[async_trait]
pub trait DonationRepository: Sync + Send {
    async fn next_id(&self) -> Result<DonationId> {
        DonationId::new(Uuid::new_v4().to_string())
    }

    async fn find_by_id(&self, id: &DonationId) -> Result<Donation>;
    async fn search(
        &self,
        author_id: Option<&UserId>,
        reader_id: Option<&UserId>,
        status: Option<&Status>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
        offset: Option<usize>,
        limit: Option<usize>,
        order_by: Option<&DonationOrderBy>,
    ) -> Result<Pagination<Donation>>;

    async fn save(&self, donation: &mut Donation) -> Result<()>;

    async fn delete(&self, id: &DonationId) -> Result<()>;
}

pub enum DonationOrderBy {
    Oldest,
    Newest,
    Amount,
}

impl FromStr for DonationOrderBy {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "newest" => DonationOrderBy::Newest,
            "amount" => DonationOrderBy::Amount,
            _ => DonationOrderBy::Oldest,
        })
    }
}
