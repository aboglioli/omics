use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use identity::domain::user::{UserId, UserRepository};
use identity::UserIdAndRole;

use crate::domain::donation::{DonationRepository, Status};
use crate::domain::payment::PaymentService;

pub struct Charge<'a> {
    event_pub: &'a dyn EventPublisher,

    donation_repo: &'a dyn DonationRepository,
    user_repo: &'a dyn UserRepository,

    payment_serv: &'a dyn PaymentService,
}

impl<'a> Charge<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        donation_repo: &'a dyn DonationRepository,
        user_repo: &'a dyn UserRepository,
        payment_serv: &'a dyn PaymentService,
    ) -> Self {
        Charge {
            event_pub,
            donation_repo,
            user_repo,
            payment_serv,
        }
    }

    pub async fn exec(&self, (auth_id, auth_role): UserIdAndRole) -> Result<CommandResponse> {
        if !auth_role.can("charge_donations") {
            return Err(Error::unauthorized());
        }

        let user = self.user_repo.find_by_id(&auth_id).await?;
        if user.payment_email().is_none() {
            return Err(Error::new("donation", "missing_payment_email"));
        }

        let pagination_donations = self
            .donation_repo
            .search(
                Some(&auth_id),
                None,
                Some(&Status::Paid),
                None,
                None,
                None,
                None,
                None,
            )
            .await?;

        let mut donations = pagination_donations.into_items();
        let mut total = 0.0;

        for donation in donations.iter_mut() {
            let payment = donation.charge()?;
            total += payment.amount().value();

            self.donation_repo.save(donation).await?;

            self.event_pub
                .publish_all(donation.events().to_vec()?)
                .await?;
        }

        self.payment_serv
            .send_payment(
                user.payment_email().unwrap().to_string(),
                "Pago de Omics en concepto de donaciones".to_owned(),
                total,
            )
            .await?;

        Ok(CommandResponse::default())
    }
}
