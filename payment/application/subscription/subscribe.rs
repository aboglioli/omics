use serde::Serialize;

use common::error::Error;
use common::event::EventPublisher;
use common::result::Result;
use identity::domain::user::UserRepository;
use identity::UserIdAndRole;
use publishing::domain::reader::ReaderRepository;

use crate::domain::payment::PaymentService;
use crate::domain::plan::{PlanId, PlanRepository};
use crate::domain::subscription::{Status, Subscription, SubscriptionRepository};

#[derive(Serialize)]
pub struct SubscriptionResponse {
    id: String,
    payment_link: String,
}

pub struct Subscribe<'a> {
    event_pub: &'a dyn EventPublisher,

    plan_repo: &'a dyn PlanRepository,
    reader_repo: &'a dyn ReaderRepository,
    subscription_repo: &'a dyn SubscriptionRepository,
    user_repo: &'a dyn UserRepository,

    payment_serv: &'a dyn PaymentService,
}

impl<'a> Subscribe<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        plan_repo: &'a dyn PlanRepository,
        reader_repo: &'a dyn ReaderRepository,
        subscription_repo: &'a dyn SubscriptionRepository,
        user_repo: &'a dyn UserRepository,
        payment_serv: &'a dyn PaymentService,
    ) -> Self {
        Subscribe {
            event_pub,
            plan_repo,
            reader_repo,
            subscription_repo,
            user_repo,
            payment_serv,
        }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        plan_id: String,
    ) -> Result<SubscriptionResponse> {
        if !auth_role.can("subscribe") {
            return Err(Error::unauthorized());
        }

        let user = self.user_repo.find_by_id(&auth_id).await?;
        let reader = self.reader_repo.find_by_id(&auth_id).await?;
        let plan = self.plan_repo.find_by_id(&PlanId::new(plan_id)?).await?;

        // TODO: should be done by a domain service
        if let Ok(subscription) = self.subscription_repo.find_by_user_id(&auth_id).await {
            match subscription.status_history().current() {
                Status::Active => {
                    return Err(Error::new("subscription", "already_exists"));
                }
                _ => {
                    self.subscription_repo
                        .delete(subscription.base().id())
                        .await?;
                }
            }
        }

        let mut subscription =
            Subscription::new(self.subscription_repo.next_id().await?, &reader, plan)?;

        let payment_link = self
            .payment_serv
            .get_payment_link(
                "Suscripción de Omics".to_owned(),
                "Plan básico.".to_owned(),
                subscription.plan().price(),
                format!("subscription:{}", subscription.base().id().value()),
                &user,
            )
            .await?;

        self.subscription_repo.save(&mut subscription).await?;

        self.event_pub
            .publish_all(subscription.events().to_vec()?)
            .await?;

        Ok(SubscriptionResponse {
            id: subscription.base().id().to_string(),
            payment_link,
        })
    }
}
