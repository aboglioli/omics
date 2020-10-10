use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::payment::PaymentService;
use crate::domain::subscription::{SubscriptionId, SubscriptionRepository};

pub struct Pay<'a> {
    event_pub: &'a dyn EventPublisher,

    subscription_repo: &'a dyn SubscriptionRepository,

    payment_serv: &'a dyn PaymentService,
}

impl<'a> Pay<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        subscription_repo: &'a dyn SubscriptionRepository,
        payment_serv: &'a dyn PaymentService,
    ) -> Self {
        Pay {
            event_pub,
            subscription_repo,
            payment_serv,
        }
    }

    pub async fn exec(&self, payment_id: String) -> Result<CommandResponse> {
        let external_reference = self
            .payment_serv
            .get_external_reference_from_payment(payment_id)
            .await?;
        let subscription_id = SubscriptionId::new(external_reference)?;

        let mut subscription = self.subscription_repo.find_by_id(&subscription_id).await?;

        subscription.pay()?;

        self.subscription_repo.save(&mut subscription).await?;

        self.event_pub
            .publish_all(subscription.events().to_vec()?)
            .await?;

        println!("Subscription {} paid.", subscription.base().id().value());

        Ok(CommandResponse::default())
    }
}
