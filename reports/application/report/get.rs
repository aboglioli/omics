use chrono::Utc;

use common::error::Error;
use common::event::EventRepository;
use common::result::Result;
use identity::domain::user::{UserId, UserRepository};

use crate::domain::report::Report;

pub struct Get<'a> {
    event_repo: &'a dyn EventRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> Get<'a> {
    pub fn new(event_repo: &'a dyn EventRepository, user_repo: &'a dyn UserRepository) -> Self {
        Get {
            event_repo,
            user_repo,
        }
    }

    pub async fn exec(&self, auth_id: String) -> Result<Report> {
        let user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
        if !user.is_admin() {
            return Err(Error::unauthorized());
        }

        Report::new(Utc::now(), Utc::now())
    }
}
