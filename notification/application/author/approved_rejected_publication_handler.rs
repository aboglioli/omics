use std::sync::Arc;

use async_trait::async_trait;

use common::event::{Event, EventHandler};
use common::result::Result;
use identity::domain::user::UserRepository;
use publishing::domain::publication::{PublicationId, PublicationRepository};
use shared::event::PublicationEvent;

use crate::domain::email::{Email, EmailService};

pub struct ApprovedRejectedPublicationHandler {
    publication_repo: Arc<dyn PublicationRepository>,
    user_repo: Arc<dyn UserRepository>,

    email_serv: Arc<dyn EmailService>,
}

impl ApprovedRejectedPublicationHandler {
    pub fn new(
        publication_repo: Arc<dyn PublicationRepository>,
        user_repo: Arc<dyn UserRepository>,
        email_serv: Arc<dyn EmailService>,
    ) -> Self {
        ApprovedRejectedPublicationHandler {
            publication_repo,
            user_repo,
            email_serv,
        }
    }
}

#[async_trait]
impl EventHandler for ApprovedRejectedPublicationHandler {
    fn topic(&self) -> &str {
        "publication"
    }

    async fn handle(&mut self, event: &Event) -> Result<bool> {
        let event: PublicationEvent = serde_json::from_value(event.payload())?;

        match event {
            PublicationEvent::Published { id, .. } => {
                let publication_id = PublicationId::new(id)?;
                let publication = self.publication_repo.find_by_id(&publication_id).await?;

                let user = self.user_repo.find_by_id(publication.author_id()).await?;

                let email = Email::new(
                    user.identity().email().to_string(),
                    "Tu publicacion fue aprobada".to_owned(),
                    format!(
                        r#"
                        <p>
                            Tu publicaci&oacute;n <b>{}</b> fue aprobada.
                        </p>
                        <p>
                            De ahora en m&aacute;s aparecer&aacute; en nuestro cat&aacute;logo.
                        </p>
                        "#,
                        publication.header().name().value(),
                    ),
                )?;

                self.email_serv.send(&email).await?;
            }
            PublicationEvent::Rejected { id } => {
                let publication_id = PublicationId::new(id)?;
                let publication = self.publication_repo.find_by_id(&publication_id).await?;

                let user = self.user_repo.find_by_id(publication.author_id()).await?;

                let email = Email::new(
                    user.identity().email().to_string(),
                    "Tu publicacion fue rechazada".to_owned(),
                    format!(
                        r#"
                        <p>
                            Tu publicaci&oacute;n <b>{}</b> fue rechazada.
                        </p>
                        <p>
                            Puedes ver el motivo en la plataforma de Omics.
                        </p>
                        "#,
                        publication.header().name().value(),
                    ),
                )?;

                self.email_serv.send(&email).await?;
            }
            _ => return Ok(false),
        }

        Ok(true)
    }
}
