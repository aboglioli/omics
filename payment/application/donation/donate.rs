use serde::{Serialize, Deserialize};

use common::event::EventPublisher;
use identity::domain::user::UserRepository;
use publishing::domain::reader::{ReaderId, ReaderRepository};
use publishing::domain::author::{AuthorId, AuthorRepository};

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

    payment_serv: &'a dyn PaymentService,
}

impl<'a> Donate<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        author_repo: &'a dyn AuthorRepository,
        donation_repo: &'a dyn DonationRepository,
        reader_repo: &'a dyn ReaderRepository,
        user_repo: &'a dyn UserRepository,
        payment_serv: &'a dyn PaymentService,
    ) -> Self {
        Donate {
            event_pub,
            author_Repo,
            donation_repo,
            reader_repo,
            user_repo,
            payment_serv,
        }
    }

    pub async fn exec(&self, auth_id: String, author_id: String, cmd: DonateCommand) -> Result<DonateResponse> {
        let reader_id = ReaderId::new(auth_id)?;
        let reader = self.reader_repo.find_by_id(&reader_id).await?;
        let reader_user = self.user_repo.find_by_id(&reader_id).await?;

        let author_id = AuthorId::new(author_id)?;
        let author = self.author_repo.find_by_id(&author_id).await?;

        let mut donation = Donation::new(
            self.donation_repo.next_id().await?,
            &author,
            &reader,
            Amount::new(cmd.amount)?,
            cmd.comment,
        )?;

        let payment_link = self
            .payment_serv
            .get_payment_link(
                "Donación".to_owned(),
                format!("Para {}", author.username().to_string()),
                donation.amount().value(),
                format!("donation:{}", donation.base().id().value()),
                &reader_user,
            )
            .await?;

        self.donation_repo.save(&mut donation).await?;

        self.event_pub.publish_all(donation.events().to_vec()?).await?;

        Ok(DonateResponse {
            id: donation.base().id().to_string(),
            payment_link,
        })
    }
}
