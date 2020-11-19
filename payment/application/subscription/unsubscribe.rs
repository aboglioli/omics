use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use identity::domain::user::UserId;
use identity::UserIdAndRole;

use crate::domain::subscription::{Status, SubscriptionRepository};

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

    pub async fn exec(&self, (auth_id, auth_role): UserIdAndRole) -> Result<CommandResponse> {
        if !auth_role.can("unsubscribe") {
            return Err(Error::unauthorized());
        }

        let mut subscriptions = self
            .subscription_repo
            .search(Some(&auth_id), None, None, None, None, None, None, None)
            .await?
            .into_items();

        subscriptions = subscriptions
            .into_iter()
            .filter(|s| !matches!(s.status_history().current(), Status::Inactive))
            .collect();

        if subscriptions.is_empty() {
            return Err(Error::not_found("subscription"));
        }

        let subscription = &mut subscriptions[0];

        subscription.disable()?;

        // self.subscription_repo.save(subscription).await?;
        self.subscription_repo
            .delete(subscription.base().id())
            .await?;

        self.event_pub
            .publish_all(subscription.events().to_vec()?)
            .await?;

        Ok(CommandResponse::default())
    }
}
