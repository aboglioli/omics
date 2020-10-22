use chrono::Utc;
use serde::Deserialize;

use common::error::Error;
use common::result::Result;
use identity::domain::user::{UserId, UserRepository};
use payment::domain::contract::ContractRepository;
use payment::domain::subscription::SubscriptionRepository;
use publishing::domain::author::AuthorRepository;
use publishing::domain::publication::PublicationRepository;

use crate::domain::report::Report;

#[derive(Deserialize)]
pub struct GenerateCommand {
    pub date_from: Option<String>,
    pub date_to: Option<String>,
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

    pub async fn exec(&self, auth_id: String, _cmd: GenerateCommand) -> Result<Report> {
        let user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
        if !user.is_admin() {
            return Err(Error::unauthorized());
        }

        Report::new(Utc::now(), Utc::now())
    }
}
