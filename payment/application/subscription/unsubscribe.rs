use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use identity::domain::user::UserId;
use publishing::domain::reader::ReaderRepository;

use crate::domain::plan::PlanRepository;
use crate::domain::subscription::SubscriptionRepository;

pub struct Unsubscribe<'a> {
    event_pub: &'a dyn EventPublisher,

    subscription_repo: &'a dyn SubscriptionRepository,
}

impl<'a> Unsubscribe<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        subscription_repo: &'a dyn SubscriptionRepository,
    ) -> Self {
        Unsubscribe {
            event_pub,
            subscription_repo,
        }
    }

    pub async fn exec(&self, auth_id: String) -> Result<CommandResponse> {
        let mut subscriptions = self
            .subscription_repo
            .search(
                Some(&UserId::new(auth_id)?),
                None,
                Some(&"active".to_owned()),
            )
            .await?;

        if subscriptions.is_empty() {
            return Err(Error::not_found("subscription"));
        }

        // TODO: maybe we should not check this and take the first subscription always
        if subscriptions.len() > 1 {
            return Err(
                Error::new("subscription", "more_than_one_active_subscription")
                    .set_message("Superman, sálvanos!"),
            );
        }

        let subscription = &mut subscriptions[0];

        subscription.disable()?;

        self.subscription_repo.save(subscription).await?;

        self.event_pub
            .publish_all(subscription.events().to_vec()?)
            .await?;

        Ok(CommandResponse::default())
    }
}
