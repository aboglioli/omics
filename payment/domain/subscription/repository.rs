use async_trait::async_trait;
use uuid::Uuid;

use common::result::Result;
use identity::domain::user::UserId;

use crate::domain::plan::PlanId;
use crate::domain::subscription::{Subscription, SubscriptionId};

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
        status: Option<&String>,
    ) -> Result<Vec<Subscription>>;

    async fn save(&self, subscription: &mut Subscription) -> Result<()>;

    async fn delete(&self, id: &SubscriptionId) -> Result<()>;
}
