use serde::Deserialize;

use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::subscription::{SubscriptionId, SubscriptionRepository};

#[derive(Deserialize)]
pub struct PayCommand {
    subscription_id: String,
}

pub struct Pay<'a> {
    event_pub: &'a dyn EventPublisher,

    subscription_repo: &'a dyn SubscriptionRepository,
}

impl<'a> Pay<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        subscription_repo: &'a dyn SubscriptionRepository,
    ) -> Self {
        Pay {
            event_pub,
            subscription_repo,
        }
    }

    pub async fn exec(&self, cmd: PayCommand) -> Result<CommandResponse> {
        let mut subscription = self
            .subscription_repo
            .find_by_id(&SubscriptionId::new(cmd.subscription_id)?)
            .await?;

        subscription.pay()?;

        self.subscription_repo.save(&mut subscription).await?;

        self.event_pub
            .publish_all(subscription.events().to_vec()?)
            .await?;

        Ok(CommandResponse::default())
    }
}
