use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::donation::{DonationId, DonationRepository};
use crate::domain::payment::PaymentService;
use crate::domain::subscription::{SubscriptionId, SubscriptionRepository};

pub struct Validate<'a> {
    event_pub: &'a dyn EventPublisher,

    donation_repo: &'a dyn DonationRepository,
    subscription_repo: &'a dyn SubscriptionRepository,

    payment_serv: &'a dyn PaymentService,
}

impl<'a> Validate<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        donation_repo: &'a dyn DonationRepository,
        subscription_repo: &'a dyn SubscriptionRepository,
        payment_serv: &'a dyn PaymentService,
    ) -> Self {
        Validate {
            event_pub,
            donation_repo,
            subscription_repo,
            payment_serv,
        }
    }

    pub async fn exec(&self, payment_id: String) -> Result<CommandResponse> {
        let external_reference = self
            .payment_serv
            .get_external_reference_from_payment(payment_id)
            .await?;

        let e_id: Vec<&str> = external_reference.split(":").collect();

        if e_id.len() == 2 {
            let entity = e_id[0];
            let id = e_id[1];

            match entity {
                "subscription" => {
                    let subscription_id = SubscriptionId::new(id)?;
                    let mut subscription =
                        self.subscription_repo.find_by_id(&subscription_id).await?;

                    subscription.pay()?;

                    self.subscription_repo.save(&mut subscription).await?;

                    self.event_pub
                        .publish_all(subscription.events().to_vec()?)
                        .await?;

                    println!("Subscription {} paid.", subscription.base().id().value());
                }
                "donation" => {
                    let donation_id = DonationId::new(id)?;
                    let mut donation = self.donation_repo.find_by_id(&donation_id).await?;

                    donation.pay()?;

                    self.donation_repo.save(&mut donation).await?;

                    self.event_pub
                        .publish_all(donation.events().to_vec()?)
                        .await?;

                    println!("Donation {} paid.", donation.base().id().value());
                }
                _ => {}
            }
        }

        Ok(CommandResponse::default())
    }
}
