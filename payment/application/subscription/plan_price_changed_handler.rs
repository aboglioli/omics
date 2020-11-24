use std::sync::Arc;

use async_trait::async_trait;

use common::event::{Event, EventHandler, EventPublisher};
use common::result::Result;
use shared::event::PlanEvent;

use crate::domain::plan::{PlanId, PlanRepository};
use crate::domain::subscription::SubscriptionRepository;

pub struct PlanPriceChangedHandler {
    event_pub: Arc<dyn EventPublisher>,
    plan_repo: Arc<dyn PlanRepository>,
    subscription_repo: Arc<dyn SubscriptionRepository>,
}

impl PlanPriceChangedHandler {
    pub fn new(
        event_pub: Arc<dyn EventPublisher>,
        plan_repo: Arc<dyn PlanRepository>,
        subscription_repo: Arc<dyn SubscriptionRepository>,
    ) -> Self {
        PlanPriceChangedHandler {
            event_pub,
            plan_repo,
            subscription_repo,
        }
    }
}

#[async_trait]
impl EventHandler for PlanPriceChangedHandler {
    fn topic(&self) -> &str {
        "plan"
    }

    async fn handle(&mut self, event: &Event) -> Result<bool> {
        let event: PlanEvent = serde_json::from_value(event.payload())?;

        match event {
            PlanEvent::PriceChanged { id, .. } => {
                let plan_id = PlanId::new(id)?;
                let plan = self.plan_repo.find_by_id(&plan_id).await?;
                let p_subscriptions = self
                    .subscription_repo
                    .search(None, Some(&plan_id), None, None, None, None, None, None)
                    .await?;

                for mut subscription in p_subscriptions.into_items() {
                    subscription.change_plan(plan.clone())?;
                    self.subscription_repo.save(&mut subscription).await?;
                    self.event_pub
                        .publish_all(subscription.events().to_vec()?)
                        .await?;
                }
            }
            _ => return Ok(false),
        }

        Ok(true)
    }
}
