use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use identity::domain::user::UserId;
use publishing::domain::reader::ReaderRepository;

use crate::domain::plan::{PlanId, PlanRepository};
use crate::domain::subscription::{Subscription, SubscriptionRepository};

pub struct Subscribe<'a> {
    event_pub: &'a dyn EventPublisher,

    plan_repo: &'a dyn PlanRepository,
    reader_repo: &'a dyn ReaderRepository,
    subscription_repo: &'a dyn SubscriptionRepository,
}

impl<'a> Subscribe<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        plan_repo: &'a dyn PlanRepository,
        reader_repo: &'a dyn ReaderRepository,
        subscription_repo: &'a dyn SubscriptionRepository,
    ) -> Self {
        Subscribe {
            event_pub,
            plan_repo,
            reader_repo,
            subscription_repo,
        }
    }

    pub async fn exec(&self, auth_id: String, plan_id: String) -> Result<CommandResponse> {
        let user_id = UserId::new(auth_id)?;
        let reader = self.reader_repo.find_by_id(&user_id).await?;
        let plan = self.plan_repo.find_by_id(&PlanId::new(plan_id)?).await?;

        let mut subscription =
            Subscription::new(self.subscription_repo.next_id().await?, &reader, plan)?;

        self.subscription_repo.save(&mut subscription).await?;

        self.event_pub
            .publish_all(subscription.events().to_vec()?)
            .await?;

        Ok(CommandResponse::default())
    }
}
