use serde::Deserialize;

use common::error::Error;
use common::event::EventPublisher;
use common::result::Result;

use crate::domain::category::CategoryId;
use crate::domain::publication::{
    Header, Image, Name, PublicationId, PublicationRepository, Synopsis, Tag,
};

#[derive(Deserialize)]
pub struct UpdateCommand {
    name: String,
    synopsis: String,
    category_id: String,
    tags: Vec<String>,
    cover: String,
}

impl UpdateCommand {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

pub struct Update<'a, EPub, PRepo> {
    event_pub: &'a EPub,

    publication_repo: &'a PRepo,
}

impl<'a, EPub, PRepo> Update<'a, EPub, PRepo>
where
    EPub: EventPublisher,
    PRepo: PublicationRepository,
{
    pub fn new(event_pub: &'a EPub, publication_repo: &'a PRepo) -> Self {
        Update {
            event_pub,
            publication_repo,
        }
    }

    pub async fn exec(
        &self,
        author_id: String,
        publication_id: String,
        cmd: UpdateCommand,
    ) -> Result<()> {
        cmd.validate()?;

        let publication_id = PublicationId::new(&publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        if publication.author_id().value() != author_id {
            return Err(Error::new("publication", "unauthorized"));
        }

        let name = Name::new(&cmd.name)?;
        let synopsis = Synopsis::new(&cmd.synopsis)?;

        let mut tags = Vec::new();
        for tag in cmd.tags.iter() {
            tags.push(Tag::new(tag)?);
        }

        let cover = Image::new(&cmd.cover)?;

        let category_id = CategoryId::new(&cmd.category_id)?;

        let header = Header::new(name, synopsis, category_id, tags, cover)?;

        publication.set_header(header)?;

        self.publication_repo.save(&mut publication).await?;

        self.event_pub
            .publish_all(publication.base().events()?)
            .await?;

        Ok(())
    }
}
