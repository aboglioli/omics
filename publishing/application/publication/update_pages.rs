use serde::Deserialize;

use common::event::EventPublisher;
use common::result::Result;

use crate::domain::publication::{Image, Page, PublicationId, PublicationRepository};

#[derive(Deserialize)]
pub struct ImageDto {
    url: String,
    size: u32,
}

#[derive(Deserialize)]
pub struct PageDto {
    images: Vec<ImageDto>,
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

    pub async fn exec(&self, id: &PublicationId, cmd: UpdatePagesCommand) -> Result<()> {
        cmd.validate()?;

        let mut pages = Vec::new();
        for (page_n, page) in cmd.pages.iter().enumerate() {
            let mut images = Vec::new();
            for image in page.images.iter() {
                images.push(Image::new(&image.url, image.size)?);
            }

            let mut page = Page::new(page_n as u32)?;
            page.set_images(images)?;

            pages.push(page);
        }

        let mut publication = self.publication_repo.find_by_id(id).await?;

        publication.set_pages(pages)?;

        self.publication_repo.save(&mut publication).await?;

        self.event_pub
            .publish_all(publication.base().events()?)
            .await?;

        Ok(())
    }
}
