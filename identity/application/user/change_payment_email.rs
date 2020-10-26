use serde::Deserialize;

use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::user::{Email, UserId, UserRepository};

#[derive(Deserialize)]
pub struct ChangePaymentEmailCommand {
    pub payment_email: String,
}

pub struct ChangePaymentEmail<'a> {
    event_pub: &'a dyn EventPublisher,

    user_repo: &'a dyn UserRepository,
}

impl<'a> ChangePaymentEmail<'a> {
    pub fn new(event_pub: &'a dyn EventPublisher, user_repo: &'a dyn UserRepository) -> Self {
        ChangePaymentEmail {
            event_pub,
            user_repo,
        }
    }

    pub async fn exec(
        &self,
        auth_id: String,
        user_id: String,
        cmd: ChangePaymentEmailCommand,
    ) -> Result<CommandResponse> {
        if auth_id != user_id {
            let auth_user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
            if !auth_user.is_admin() {
                return Err(Error::unauthorized());
            }
        }

        let mut user = self.user_repo.find_by_id(&UserId::new(user_id)?).await?;

        user.set_payment_email(Email::new(cmd.payment_email)?)?;

        self.user_repo.save(&mut user).await?;

        self.event_pub.publish_all(user.events().to_vec()?).await?;

        Ok(CommandResponse::default())
    }
}
