use async_trait::async_trait;

use common::result::Result;

use crate::domain::subscription::{Subscription, SubscriptionId};

#[async_trait]
pub trait SubscriptionRepository: Sync + Send {
    async fn next_id(&self) -> Result<SubscriptionId>;

    async fn find_by_id(&self, id: &SubscriptionId) -> Result<Subscription>;

    async fn save(&self, subscription: &mut Subscription) -> Result<()>;
}
