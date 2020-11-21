use serde::{Deserialize, Serialize};

use common::config::ConfigService;
use common::error::Error;
use common::event::EventPublisher;
use common::result::Result;
use identity::domain::user::UserRepository;
use identity::UserIdAndRole;
use publishing::domain::author::{AuthorId, AuthorRepository};
use publishing::domain::reader::ReaderRepository;

use crate::domain::donation::{Donation, DonationRepository};
use crate::domain::payment::{Amount, PaymentService};

#[derive(Deserialize)]
pub struct DonateCommand {
    amount: f64,
    comment: String,
}

#[derive(Serialize)]
pub struct DonateResponse {
    id: String,
    payment_link: String,
}

pub struct Donate<'a> {
    event_pub: &'a dyn EventPublisher,

    author_repo: &'a dyn AuthorRepository,
    donation_repo: &'a dyn DonationRepository,
    reader_repo: &'a dyn ReaderRepository,
    user_repo: &'a dyn UserRepository,

    config_serv: &'a ConfigService,
    payment_serv: &'a dyn PaymentService,
}

impl<'a> Donate<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        author_repo: &'a dyn AuthorRepository,
        donation_repo: &'a dyn DonationRepository,
        reader_repo: &'a dyn ReaderRepository,
        user_repo: &'a dyn UserRepository,
        config_serv: &'a ConfigService,
        payment_serv: &'a dyn PaymentService,
    ) -> Self {
        Donate {
            event_pub,
            author_repo,
            donation_repo,
            reader_repo,
            user_repo,
            config_serv,
            payment_serv,
        }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        author_id: String,
        cmd: DonateCommand,
    ) -> Result<DonateResponse> {
        if !auth_role.can("donate") {
            return Err(Error::unauthorized());
        }

        let business_rules = self.config_serv.get_business_rules().await?;
        if cmd.amount < business_rules.minimum_donation_amount {
            return Err(Error::new("donation", "minimum_amount"));
        }

        let reader = self.reader_repo.find_by_id(&auth_id).await?;
        let reader_user = self.user_repo.find_by_id(&auth_id).await?;

        let author_id = AuthorId::new(author_id)?;
        let author = self.author_repo.find_by_id(&author_id).await?;

        let author_percentage = 1.0 - business_rules.donation_percentage_retention;

        let mut donation = Donation::new(
            self.donation_repo.next_id().await?,
            &author,
            &reader,
            Amount::new(cmd.amount)?,
            cmd.comment,
            author_percentage,
        )?;

        let payment_link = self
            .payment_serv
            .get_payment_link(
                "Donación".to_owned(),
                format!("Para {}", author.username().to_string()),
                donation.total().value(),
                format!("donation:{}", donation.base().id().value()),
                &reader_user,
            )
            .await?;

        self.donation_repo.save(&mut donation).await?;

        self.event_pub
            .publish_all(donation.events().to_vec()?)
            .await?;

        Ok(DonateResponse {
            id: donation.base().id().to_string(),
            payment_link,
        })
    }
}
