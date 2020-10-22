use std::str::FromStr;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use common::error::Error;
use common::model::Pagination;
use common::result::Result;
use identity::domain::user::UserId;

use crate::domain::plan::PlanId;
use crate::domain::subscription::{Status, Subscription, SubscriptionId};

#[async_trait]
pub trait SubscriptionRepository: Sync + Send {
    async fn next_id(&self) -> Result<SubscriptionId> {
        SubscriptionId::new(Uuid::new_v4().to_string())
    }

    async fn find_by_id(&self, id: &SubscriptionId) -> Result<Subscription>;
    async fn find_by_user_id(&self, id: &UserId) -> Result<Subscription>;
    async fn search(
        &self,
        user_id: Option<&UserId>,
        plan_id: Option<&PlanId>,
        status: Option<&Status>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
        offset: Option<usize>,
        limit: Option<usize>,
        order_by: Option<&SubscriptionOrderBy>,
    ) -> Result<Pagination<Subscription>>;

    async fn save(&self, subscription: &mut Subscription) -> Result<()>;

    async fn delete(&self, id: &SubscriptionId) -> Result<()>;
}

pub enum SubscriptionOrderBy {
    Oldest,
    Newest,
}

impl FromStr for SubscriptionOrderBy {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "newest" => SubscriptionOrderBy::Newest,
            _ => SubscriptionOrderBy::Oldest,
        })
    }
}
