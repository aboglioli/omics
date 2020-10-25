use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use identity::domain::user::UserId;

use crate::domain::donation::{DonationRepository, Status};

pub struct Charge<'a> {
    event_pub: &'a dyn EventPublisher,

    donation_repo: &'a dyn DonationRepository,
}

impl<'a> Charge<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        donation_repo: &'a dyn DonationRepository,
    ) -> Self {
        Charge {
            event_pub,
            donation_repo,
        }
    }

    pub async fn exec(&self, auth_id: String) -> Result<CommandResponse> {
        let pagination_donations = self
            .donation_repo
            .search(
                Some(&UserId::new(auth_id)?),
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

        // TODO: PaymentService

        Ok(CommandResponse::default())
    }
}
