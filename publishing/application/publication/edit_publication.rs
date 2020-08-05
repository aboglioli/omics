use serde::Deserialize;

use common::result::Result;

use crate::domain::category::CategoryId;
use crate::domain::publication::{
    Image, Name, Page, PublicationId, PublicationRepository, Synopsis, Tag,
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
pub struct EditPublicationCommand {
    name: String,
    synopsis: String,
    pages: Vec<PageDto>,
    category_id: CategoryId,
    tags: Vec<String>,
}

impl EditPublicationCommand {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

pub struct EditPublication<'a, PRepo> {
    publication_repo: &'a PRepo,
}

impl<'a, PRepo> EditPublication<'a, PRepo>
where
    PRepo: PublicationRepository,
{
    pub fn new(publication_repo: &'a PRepo) -> Self {
        EditPublication { publication_repo }
    }

    pub async fn exec(&self, id: &PublicationId, cmd: EditPublicationCommand) -> Result<()> {
        cmd.validate()?;

        let mut publication = self.publication_repo.find_by_id(id).await?;

        let name = Name::new(&cmd.name)?;
        publication.set_name(name)?;
        let synopsis = Synopsis::new(&cmd.synopsis)?;
        publication.set_synopsis(synopsis)?;

        let pages = Vec::new();
        for (page_n, page) in cmd.pages.iter().enumerate() {
            let mut images = Vec::new();
            for (img_id, image) in page.images.iter().enumerate() {
                images.push(Image::new(img_id.to_string(), &image.url, image.size)?);
            }

            let mut page = Page::new(page_n as u32)?;
            page.set_images(images)?;
        }
        publication.set_pages(pages)?;

        let tags: Vec<Tag> = cmd.tags.iter().map(|t| Tag::new(t).unwrap()).collect();
        publication.set_tags(tags);

        publication.set_cateogry(cmd.category_id)?;

        self.publication_repo.save(&mut publication).await?;

        Ok(())
    }
}
