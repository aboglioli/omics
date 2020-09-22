use std::sync::Arc;

use async_trait::async_trait;

use common::event::{Event, EventHandler};
use common::result::Result;
use identity::domain::user::{UserId, UserRepository};
use publishing::domain::author::AuthorRepository;
use publishing::domain::collection::CollectionRepository;
use publishing::domain::interaction::InteractionRepository;
use publishing::domain::publication::PublicationRepository;
use shared::event::{PublicationEvent, UserEvent};

use crate::domain::notification::{Body, Notification, NotificationRepository};

pub struct NotificationHandler {
    author_repo: Arc<dyn AuthorRepository>,
    collection_repo: Arc<dyn CollectionRepository>,
    interaction_repo: Arc<dyn InteractionRepository>,
    notification_repo: Arc<dyn NotificationRepository>,
    publication_repo: Arc<dyn PublicationRepository>,
    user_repo: Arc<dyn UserRepository>,
}

impl NotificationHandler {
    pub fn new(
        author_repo: Arc<dyn AuthorRepository>,
        collection_repo: Arc<dyn CollectionRepository>,
        interaction_repo: Arc<dyn InteractionRepository>,
        notification_repo: Arc<dyn NotificationRepository>,
        publication_repo: Arc<dyn PublicationRepository>,
        user_repo: Arc<dyn UserRepository>,
    ) -> Self {
        NotificationHandler {
            author_repo,
            collection_repo,
            interaction_repo,
            notification_repo,
            publication_repo,
            user_repo,
        }
    }
}

#[async_trait]
impl EventHandler for NotificationHandler {
    fn topic(&self) -> &str {
        ".*"
    }

    async fn handle(&mut self, event: &Event) -> Result<bool> {
        match event.topic() {
            "user" => {
                let event: UserEvent = serde_json::from_slice(event.payload())?;

                match event {
                    UserEvent::Validated { id } => {
                        let user_id = UserId::new(id)?;
                        let user = self.user_repo.find_by_id(&user_id).await?;

                        let mut body = Body::new()
                            .reader(user.base().id().value(), user.identity().username().value());

                        if let Some(person) = user.person() {
                            body = body.reader_name(
                                person.fullname().name(),
                                person.fullname().lastname(),
                            );
                        }

                        let mut notification = Notification::new(
                            self.notification_repo.next_id().await?,
                            user_id,
                            "welcome",
                            body,
                        )?;

                        self.notification_repo.save(&mut notification).await?;
                    }
                    _ => return Ok(false),
                }
            }
            "publication" => {
                let event: PublicationEvent = serde_json::from_slice(event.payload())?;

                match event {
                    PublicationEvent::Published { id: _, .. } => {}
                    _ => return Ok(false),
                }
            }
            _ => {}
        }

        Ok(true)
    }
}
