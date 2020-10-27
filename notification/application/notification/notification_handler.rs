use std::sync::Arc;

use async_trait::async_trait;

use common::event::{Event, EventHandler};
use common::result::Result;
use identity::domain::user::{UserId, UserRepository};
use publishing::domain::author::{AuthorId, AuthorRepository};
use publishing::domain::interaction::InteractionRepository;
use publishing::domain::publication::{PublicationId, PublicationRepository};
use shared::event::{
    AuthorEvent, ContractEvent, DonationEvent, PublicationEvent, SubscriptionEvent, UserEvent,
};

use crate::domain::notification::{Body, Notification, NotificationRepository};

pub struct NotificationHandler {
    author_repo: Arc<dyn AuthorRepository>,
    interaction_repo: Arc<dyn InteractionRepository>,
    notification_repo: Arc<dyn NotificationRepository>,
    publication_repo: Arc<dyn PublicationRepository>,
    user_repo: Arc<dyn UserRepository>,
}

impl NotificationHandler {
    pub fn new(
        author_repo: Arc<dyn AuthorRepository>,
        interaction_repo: Arc<dyn InteractionRepository>,
        notification_repo: Arc<dyn NotificationRepository>,
        publication_repo: Arc<dyn PublicationRepository>,
        user_repo: Arc<dyn UserRepository>,
    ) -> Self {
        NotificationHandler {
            author_repo,
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

    // Notifications:
    // - reader: Welcome
    // - reader: New publication from followed author
    // - author: Approved publication
    // - author: Rejected publication
    // - author: Publication liked
    // - author: Publication reviewed
    // - reader: Publication published
    async fn handle(&mut self, event: &Event) -> Result<bool> {
        match event.topic() {
            "user" => {
                let event: UserEvent = serde_json::from_value(event.payload())?;

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
                let event: PublicationEvent = serde_json::from_value(event.payload())?;

                match event {
                    PublicationEvent::Published {
                        id,
                        author_id,
                        name,
                        ..
                    } => {
                        let author_id = AuthorId::new(author_id)?;
                        let author = self.author_repo.find_by_id(&author_id).await?;
                        let follows = self
                            .interaction_repo
                            .find_follows(None, Some(&author_id), None, None)
                            .await?;

                        let mut body = Body::new()
                            .author(author.base().id().value(), author.username())
                            .publication(id.clone(), name.clone());

                        if author.name().is_some() && author.lastname().is_some() {
                            body = body
                                .author_name(author.name().unwrap(), author.lastname().unwrap());
                        }

                        // Notify readers of followed author
                        for follow in follows.into_iter() {
                            let reader_id = follow.base().id().reader_id();

                            let mut notification = Notification::new(
                                self.notification_repo.next_id().await?,
                                reader_id.clone(),
                                "new-publication-from-followed-author",
                                body.clone(),
                            )?;

                            self.notification_repo.save(&mut notification).await?;
                        }

                        // Notify author of approved publication
                        let body = Body::new().publication(id, name);

                        let mut notification = Notification::new(
                            self.notification_repo.next_id().await?,
                            author_id,
                            "publication-approved",
                            body,
                        )?;

                        self.notification_repo.save(&mut notification).await?;
                    }
                    PublicationEvent::Rejected { id } => {
                        let publication_id = PublicationId::new(id)?;
                        let publication = self.publication_repo.find_by_id(&publication_id).await?;

                        let body = Body::new().publication(
                            publication.base().id().value(),
                            publication.header().name().value(),
                        );

                        let mut notification = Notification::new(
                            self.notification_repo.next_id().await?,
                            publication.author_id().clone(),
                            "publication-rejected",
                            body,
                        )?;

                        self.notification_repo.save(&mut notification).await?;
                    }
                    PublicationEvent::Liked {
                        reader_id,
                        publication_id,
                    } => {
                        let reader_id = UserId::new(reader_id)?;
                        let reader = self.user_repo.find_by_id(&reader_id).await?;

                        let publication_id = PublicationId::new(publication_id)?;
                        let publication = self.publication_repo.find_by_id(&publication_id).await?;

                        let body = Body::new()
                            .reader(
                                reader.base().id().value(),
                                reader.identity().username().value(),
                            )
                            .publication(
                                publication.base().id().value(),
                                publication.header().name().value(),
                            );

                        let mut notification = Notification::new(
                            self.notification_repo.next_id().await?,
                            publication.author_id().clone(),
                            "publication-liked",
                            body,
                        )?;

                        self.notification_repo.save(&mut notification).await?;
                    }
                    _ => return Ok(false),
                }
            }
            "author" => {
                let event: AuthorEvent = serde_json::from_value(event.payload())?;

                match event {
                    AuthorEvent::Followed {
                        author_id,
                        reader_id,
                    } => {
                        let reader = self.user_repo.find_by_id(&UserId::new(reader_id)?).await?;

                        let mut body = Body::new().reader(
                            reader.base().id().value(),
                            reader.identity().username().value(),
                        );

                        if let Some(person) = reader.person() {
                            body = body.reader_name(
                                person.fullname().name(),
                                person.fullname().lastname(),
                            );
                        }

                        let mut notification = Notification::new(
                            self.notification_repo.next_id().await?,
                            AuthorId::new(author_id)?,
                            "author-followed",
                            body,
                        )?;

                        self.notification_repo.save(&mut notification).await?;
                    }
                    _ => return Ok(false),
                }
            }
            "subscription" => {
                let event: SubscriptionEvent = serde_json::from_value(event.payload())?;

                match event {
                    SubscriptionEvent::PaymentAdded { id: _, user_id, .. } => {
                        let user = self.user_repo.find_by_id(&UserId::new(user_id)?).await?;

                        let body = Body::new()
                            .reader(user.base().id().value(), user.identity().username().value());

                        let mut notification = Notification::new(
                            self.notification_repo.next_id().await?,
                            user.base().id().clone(),
                            "subscription-activated",
                            body,
                        )?;

                        self.notification_repo.save(&mut notification).await?;
                    }
                    _ => return Ok(false),
                }
            }
            "contract" => {
                let event: ContractEvent = serde_json::from_value(event.payload())?;

                match event {
                    ContractEvent::Approved { publication_id, .. } => {
                        let publication = self
                            .publication_repo
                            .find_by_id(&PublicationId::new(publication_id)?)
                            .await?;

                        let body = Body::new().publication(
                            publication.base().id().value(),
                            publication.header().name().value(),
                        );

                        let mut notification = Notification::new(
                            self.notification_repo.next_id().await?,
                            publication.author_id().clone(),
                            "contract-approved",
                            body,
                        )?;

                        self.notification_repo.save(&mut notification).await?;
                    }
                    ContractEvent::Rejected { publication_id, .. } => {
                        let publication = self
                            .publication_repo
                            .find_by_id(&PublicationId::new(publication_id)?)
                            .await?;

                        let body = Body::new().publication(
                            publication.base().id().value(),
                            publication.header().name().value(),
                        );

                        let mut notification = Notification::new(
                            self.notification_repo.next_id().await?,
                            publication.author_id().clone(),
                            "contract-rejected",
                            body,
                        )?;

                        self.notification_repo.save(&mut notification).await?;
                    }
                    ContractEvent::PaymentAdded { publication_id, .. } => {
                        let publication = self
                            .publication_repo
                            .find_by_id(&PublicationId::new(publication_id)?)
                            .await?;

                        let body = Body::new().publication(
                            publication.base().id().value(),
                            publication.header().name().value(),
                        );

                        let mut notification = Notification::new(
                            self.notification_repo.next_id().await?,
                            publication.author_id().clone(),
                            "contract-payment-added",
                            body,
                        )?;

                        self.notification_repo.save(&mut notification).await?;
                    }
                    _ => return Ok(false),
                }
            }
            "donation" => {
                let event: DonationEvent = serde_json::from_value(event.payload())?;

                match event {
                    DonationEvent::Paid {
                        author_id,
                        reader_id,
                        total,
                        ..
                    } => {
                        // Reader
                        let reader_id = UserId::new(reader_id)?;
                        let reader = self.user_repo.find_by_id(&reader_id).await?;

                        let author_id = UserId::new(author_id)?;
                        let author = self.user_repo.find_by_id(&author_id).await?;

                        let mut body = Body::new()
                            .reader(
                                reader.base().id().value(),
                                reader.identity().username().value(),
                            )
                            .author(
                                author.base().id().value(),
                                author.identity().username().value(),
                            )
                            .amount(total);

                        if let Some(person) = reader.person() {
                            body = body.reader_name(
                                person.fullname().name(),
                                person.fullname().lastname(),
                            );
                        }

                        if let Some(person) = author.person() {
                            body = body.author_name(
                                person.fullname().name(),
                                person.fullname().lastname(),
                            );
                        }

                        let mut notification = Notification::new(
                            self.notification_repo.next_id().await?,
                            reader_id,
                            "donation-paid",
                            body.clone(),
                        )?;
                        self.notification_repo.save(&mut notification).await?;

                        let mut notification = Notification::new(
                            self.notification_repo.next_id().await?,
                            author_id,
                            "donation-received",
                            body,
                        )?;
                        self.notification_repo.save(&mut notification).await?;
                    }
                    _ => return Ok(false),
                }
            }
            _ => {}
        }

        Ok(true)
    }
}
