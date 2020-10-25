use serde::{Serialize, Deserialize};

use common::event::EventPublisher;
use identity::domain::user::UserRepository;

use crate::domain::donation::DonationRepository;

#[derive(Serialize)]
pub struct DonateResponse {
    id: String,
    payment_link: String,
}

pub struct Donate<'a> {
    event_pub: &'a dyn EventPublisher,

    donation_repo: &'a dyn DonationRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> Donate<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        donation_repo: &'a dyn DonationRepository,
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        Donate {
            event_pub,
            donation_repo,
            user_repo,
        }
    }

    pub async fn exec(&self, auth_id: String, author_id: String) -> Result<DonateResponse> {

    }
}
