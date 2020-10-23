use std::collections::HashMap;
use std::str::FromStr;

use chrono::DateTime;
use serde::Deserialize;

use common::error::Error;
use common::result::Result;
use identity::domain::user::{UserId, UserRepository};
use payment::domain::contract::ContractRepository;
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
    publication_repo: &'a dyn PublicationRepository,
    subscription_repo: &'a dyn SubscriptionRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> Generate<'a> {
    pub fn new(
        author_repo: &'a dyn AuthorRepository,
        contract_repo: &'a dyn ContractRepository,
        publication_repo: &'a dyn PublicationRepository,
        subscription_repo: &'a dyn SubscriptionRepository,
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        Generate {
            author_repo,
            contract_repo,
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

        // Users
        let p_users = self
            .user_repo
            .search(None, Some(&date_from), Some(&date_to), None, None, None)
            .await?;

        let users = Users {
            total: p_users.total(),
            new: p_users.matching_criteria(),
            by_gender: HashMap::new(),
            by_age: HashMap::new(),
        };

        // Authors
        let p_authors = self
            .author_repo
            .search(None, Some(&date_from), Some(&date_to), None, None, None)
            .await?;

        let authors = Authors {
            total: p_authors.total(),
            new: p_authors.matching_criteria(),
        };

        // Publications
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

        let publications = Publications {
            total: p_publications.total(),
            new: p_publications.matching_criteria(),
            by_category: HashMap::new(),
            by_preferences: HashMap::new(),
        };

        // Subscriptions
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

        let subscriptions = Subscriptions {
            total: p_subscriptions.total(),
            new: p_subscriptions.matching_criteria(),
            amount: 0.0,
        };

        // Contracts
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

        let contracts = Contracts {
            total: p_contracts.total(),
            new: p_contracts.matching_criteria(),
            amount: 0.0,
        };

        // Payments
        let payments = Payments {
            income: 0.0,
            outcome: 0.0,
        };

        let mut report = Report::new(date_from, date_to)?;
        report.users = Some(users);
        report.authors = Some(authors);
        report.publications = Some(publications);
        report.subscriptions = Some(subscriptions);
        report.contracts = Some(contracts);
        report.payments = Some(payments);

        Ok(report)
    }
}
