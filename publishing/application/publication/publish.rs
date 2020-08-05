use serde::Deserialize;

use common::result::Result;

use crate::domain::category::CategoryId;
use crate::domain::publication::{
    Image, Name, Page, Publication, PublicationRepository, Synopsis, Tag,
};

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
pub struct PublishCommand {
    name: String,
    synopsis: String,
    pages: Vec<PageDto>,
    category_id: CategoryId,
    tags: Vec<String>,
}

impl PublishCommand {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

pub struct Publish<'a, PRepo> {
    publication_repo: &'a PRepo,
}

impl<'a, PRepo> Publish<'a, PRepo>
where
    PRepo: PublicationRepository,
{
    pub fn new(publication_repo: &'a PRepo) -> Self {
        Publish { publication_repo }
    }

    pub async fn exec(&self, cmd: PublishCommand) -> Result<()> {
        cmd.validate()?;

        let name = Name::new(&cmd.name)?;
        let synopsis = Synopsis::new(&cmd.synopsis)?;

        let pages = Vec::new();
        for (page_n, page) in cmd.pages.iter().enumerate() {
            let mut images = Vec::new();
            for (img_id, image) in page.images.iter().enumerate() {
                images.push(Image::new(img_id.to_string(), &image.url, image.size)?);
            }

            let mut page = Page::new(page_n as u32)?;
            page.set_images(images)?;
        }
        let tags: Vec<Tag> = cmd.tags.iter().map(|t| Tag::new(t).unwrap()).collect();

        let mut publication = Publication::new(
            self.publication_repo.next_id().await?,
            name,
            synopsis,
            "NULL".to_owned(),
            cmd.category_id,
        )?;
        publication.set_pages(pages)?;
        publication.set_tags(tags)?;

        self.publication_repo.save(&mut publication).await?;

        Ok(())
    }
}
