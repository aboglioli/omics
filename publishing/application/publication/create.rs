use serde::Deserialize;

use common::result::Result;

use crate::domain::author::AuthorId;
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
pub struct CreateCommand {
    author_id: AuthorId,
    name: String,
    synopsis: String,
    pages: Vec<PageDto>,
    category_id: CategoryId,
    tags: Vec<String>,
}

impl CreateCommand {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

pub struct Create<'a, PRepo> {
    publication_repo: &'a PRepo,
}

impl<'a, PRepo> Create<'a, PRepo>
where
    PRepo: PublicationRepository,
{
    pub fn new(publication_repo: &'a PRepo) -> Self {
        Create { publication_repo }
    }

    pub async fn exec(&self, cmd: CreateCommand) -> Result<()> {
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
            cmd.author_id,
            cmd.category_id,
        )?;
        publication.set_pages(pages)?;
        publication.set_tags(tags)?;

        self.publication_repo.save(&mut publication).await?;

        Ok(())
    }
}
