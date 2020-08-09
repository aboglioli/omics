use serde::Deserialize;

use common::event::EventPublisher;
use common::result::Result;

use crate::domain::publication::{
    Header, Image, Name, PublicationId, PublicationRepository, Synopsis, Tag,
};

#[derive(Deserialize)]
pub struct ImageDto {
    url: String,
    size: u32,
}

#[derive(Deserialize)]
pub struct UpdateCommand {
    name: String,
    synopsis: String,
    category_id: String,
    tags: Vec<String>,
    cover: ImageDto,
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

    pub async fn exec(&self, id: &PublicationId, cmd: UpdateCommand) -> Result<()> {
        cmd.validate()?;

        let name = Name::new(&cmd.name)?;
        let synopsis = Synopsis::new(&cmd.synopsis)?;

        let mut tags = Vec::new();
        for tag in cmd.tags.iter() {
            tags.push(Tag::new(tag)?);
        }

        let cover = Image::new(&cmd.cover.url, cmd.cover.size)?;

        let header = Header::new(name, synopsis, cmd.category_id, tags, cover)?;

        let mut publication = self.publication_repo.find_by_id(id).await?;

        publication.set_header(header)?;

        self.publication_repo.save(&mut publication).await?;

        self.event_pub
            .publish_all(publication.base().events()?)
            .await?;

        Ok(())
    }
}
