use common::event::EventPublisher;
use common::result::Result;
use serde::Deserialize;

use crate::domain::author::AuthorId;
use crate::domain::category::CategoryId;
use crate::domain::publication::{
    Header, Image, Name, Publication, PublicationRepository, Synopsis, Tag,
};

#[derive(Deserialize)]
pub struct ImageDto {
    url: String,
    size: u32,
}

#[derive(Deserialize)]
pub struct CreateCommand {
    name: String,
    synopsis: String,
    category_id: String,
    tags: Vec<String>,
    cover: ImageDto,
}

impl CreateCommand {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

pub struct Create<'a, EPub, PRepo> {
    event_pub: &'a EPub,

    publication_repo: &'a PRepo,
}

impl<'a, EPub, PRepo> Create<'a, EPub, PRepo>
where
    EPub: EventPublisher,
    PRepo: PublicationRepository,
{
    pub fn new(event_pub: &'a EPub, publication_repo: &'a PRepo) -> Self {
        Create {
            event_pub,
            publication_repo,
        }
    }

    pub async fn exec(&self, author_id: &AuthorId, cmd: CreateCommand) -> Result<()> {
        cmd.validate()?;

        let name = Name::new(&cmd.name)?;
        let synopsis = Synopsis::new(&cmd.synopsis)?;

        let mut tags = Vec::new();
        for tag in cmd.tags.iter() {
            tags.push(Tag::new(tag)?);
        }

        let cover = Image::new(&cmd.cover.url, cmd.cover.size)?;

        let category_id = CategoryId::new(&cmd.category_id)?;

        let header = Header::new(name, synopsis, category_id, tags, cover)?;

        let mut publication = Publication::new(
            self.publication_repo.next_id().await?,
            author_id.to_owned(),
            header,
        )?;

        self.publication_repo.save(&mut publication).await?;

        self.event_pub
            .publish_all(publication.base().events()?)
            .await?;

        Ok(())
    }
}
