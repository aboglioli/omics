use serde::Deserialize;

use common::error::Error;
use common::event::EventPublisher;
use common::result::Result;

use crate::domain::publication::{Image, Page, PublicationId, PublicationRepository};

#[derive(Deserialize)]
pub struct PageDto {
    images: Vec<String>,
}

#[derive(Deserialize)]
pub struct UpdatePagesCommand {
    pages: Vec<PageDto>,
}

impl UpdatePagesCommand {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

pub struct UpdatePages<'a, EPub, PRepo> {
    event_pub: &'a EPub,

    publication_repo: &'a PRepo,
}

impl<'a, EPub, PRepo> UpdatePages<'a, EPub, PRepo>
where
    EPub: EventPublisher,
    PRepo: PublicationRepository,
{
    pub fn new(event_pub: &'a EPub, publication_repo: &'a PRepo) -> Self {
        UpdatePages {
            event_pub,
            publication_repo,
        }
    }

    pub async fn exec(
        &self,
        author_id: String,
        publication_id: String,
        cmd: UpdatePagesCommand,
    ) -> Result<()> {
        cmd.validate()?;

        let publication_id = PublicationId::new(&publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        if publication.author_id().value() != author_id {
            return Err(Error::new("publication", "unauthorized"));
        }

        let mut pages = Vec::new();
        for (page_n, page) in cmd.pages.into_iter().enumerate() {
            let mut images = Vec::new();
            for image in page.images.into_iter() {
                images.push(Image::new(image)?);
            }

            let mut page = Page::new(page_n as u32)?;
            page.set_images(images)?;

            pages.push(page);
        }

        publication.set_pages(pages)?;

        self.publication_repo.save(&mut publication).await?;

        self.event_pub
            .publish_all(publication.base().events()?)
            .await?;

        Ok(())
    }
}
