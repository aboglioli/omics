use std::sync::Arc;

use async_trait::async_trait;

use common::error::Error;
use common::event::{Event, EventHandler};
use common::result::Result;

use crate::domain::user::{UserId, UserRepository, UserService};
use crate::event::UserEvent;

pub struct SyncHandler {
    user_repo: Arc<dyn UserRepository>,

    user_serv: Arc<dyn UserService>,
}

impl SyncHandler {
    pub fn new(user_repo: Arc<dyn UserRepository>, user_serv: Arc<dyn UserService>) -> Self {
        SyncHandler {
            user_repo,
            user_serv,
        }
    }
}

#[async_trait]
impl EventHandler for SyncHandler {
    fn topic(&self) -> &str {
        "user"
    }

    async fn handle(&mut self, event: &Event) -> Result<bool> {
        let event: UserEvent = serde_json::from_slice(event.payload())
            .map_err(|err| Error::new("user.sync_handler", "deserialize").wrap_raw(err))?;

        match event {
            UserEvent::Validated { id } => {
                let mut user = self.user_serv.get_by_id(&UserId::new(&id)?).await?;
                self.user_repo.save(&mut user).await?;
            }
            UserEvent::Updated {
                id,
                name,
                lastname,
                birthdate: _,
                gender: _,
                biography,
                profile_image,
            } => {
                let mut user = self.user_serv.get_by_id(&UserId::new(id)?).await?;

                user.set_name(name, lastname)?;

                if let Some(biography) = biography {
                    user.set_biography(biography)?;
                }

                if let Some(profile_image) = profile_image {
                    user.set_profile_image(profile_image)?;
                }

                self.user_repo.save(&mut user).await?;
            }
            UserEvent::RoleChanged { id, role_id } => {
                let mut user = self.user_serv.get_by_id(&UserId::new(id)?).await?;

                user.change_role(role_id)?;

                self.user_repo.save(&mut user).await?;
            }
            UserEvent::Deleted { id } => {
                let mut user = self.user_serv.get_by_id(&UserId::new(&id)?).await?;
                user.delete()?;
                self.user_repo.save(&mut user).await?;
            }
            _ => return Ok(false),
        }

        Ok(true)
    }
}
