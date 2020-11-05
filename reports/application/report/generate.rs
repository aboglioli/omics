use std::str::FromStr;

use chrono::DateTime;
use serde::Deserialize;

use common::error::Error;
use common::result::Result;
use identity::domain::user::{UserId, UserRepository};
use payment::domain::contract::ContractRepository;
use payment::domain::donation::DonationRepository;
use payment::domain::subscription::SubscriptionRepository;
use publishing::domain::author::AuthorRepository;
use publishing::domain::publication::PublicationRepository;

use crate::domain::report::{
    Authors, Contracts, Payments, Publications, Report, Subscriptions, Users,
};

#[derive(Deserialize)]
pub struct GenerateCommand {
    pub date_from: String,
    pub date_to: String,
}

pub struct Generate<'a> {
    author_repo: &'a dyn AuthorRepository,
    contract_repo: &'a dyn ContractRepository,
    donation_repo: &'a dyn DonationRepository,
    publication_repo: &'a dyn PublicationRepository,
    subscription_repo: &'a dyn SubscriptionRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> Generate<'a> {
    pub fn new(
        author_repo: &'a dyn AuthorRepository,
        contract_repo: &'a dyn ContractRepository,
        donation_repo: &'a dyn DonationRepository,
        publication_repo: &'a dyn PublicationRepository,
        subscription_repo: &'a dyn SubscriptionRepository,
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        Generate {
            author_repo,
            contract_repo,
            donation_repo,
            publication_repo,
            subscription_repo,
            user_repo,
        }
    }

    pub async fn exec(&self, auth_id: String, cmd: GenerateCommand) -> Result<Report> {
        let user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
        if !user.is_admin() {
            return Err(Error::unauthorized());
        }

        let date_from = DateTime::from_str(&cmd.date_from)
            .map_err(|err| Error::bad_format("date_from").wrap_raw(err))?;
        let date_to = DateTime::from_str(&cmd.date_to)
            .map_err(|err| Error::bad_format("date_to").wrap_raw(err))?;

        let p_users = self
            .user_repo
            .search(None, Some(&date_from), Some(&date_to), None, None, None)
            .await?;

        let p_publications = self
            .publication_repo
            .search(
                None,
                None,
                None,
                None,
                None,
                Some(&date_from),
                Some(&date_to),
                None,
                None,
                None,
            )
            .await?;

        let p_subscriptions = self
            .subscription_repo
            .search(
                None,
                None,
                None,
                Some(&date_from),
                Some(&date_to),
                None,
                None,
                None,
            )
            .await?;

        let p_contracts = self
            .contract_repo
            .search(
                None,
                None,
                Some(&date_from),
                Some(&date_to),
                None,
                None,
                None,
            )
            .await?;

        let p_donations = self
            .donation_repo
            .search(
                None,
                None,
                None,
                Some(&date_from),
                Some(&date_to),
                None,
                None,
                None,
            )
            .await?;

        let mut report = Report::new(date_from, date_to)?;

        report.map_users(p_users.items());
        report.map_publications(p_publications.items());
        report.map_subscriptions(p_subscriptions.items());
        report.map_contracts(p_contracts.items());
        report.map_donations(p_donations.items());
        report.map_payments(p_subscriptions.items(), p_contracts.items(), p_donations.items());

        Ok(report)
    }
}
