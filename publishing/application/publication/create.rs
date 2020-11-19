use serde::{Deserialize, Serialize};

use common::error::Error;
use common::event::EventPublisher;
use common::result::Result;
use identity::UserIdAndRole;

use crate::domain::author::{AuthorId, AuthorRepository};
use crate::domain::category::{CategoryId, CategoryRepository};
use crate::domain::publication::{
    Header, Image, Name, Page, Publication, PublicationRepository, Synopsis, Tag,
};

#[derive(Deserialize)]
pub struct PageDto {
    pub images: Vec<String>,
}

#[derive(Deserialize)]
pub struct CreateCommand {
    pub name: String,
    pub synopsis: String,
    pub category_id: String,
    pub tags: Vec<String>,
    pub cover: String,
    pub pages: Option<Vec<PageDto>>,
}

#[derive(Serialize)]
pub struct CreateResponse {
    pub id: String,
}

pub struct Create<'a> {
    event_pub: &'a dyn EventPublisher,

    author_repo: &'a dyn AuthorRepository,
    category_repo: &'a dyn CategoryRepository,
    publication_repo: &'a dyn PublicationRepository,
}

impl<'a> Create<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        author_repo: &'a dyn AuthorRepository,
        category_repo: &'a dyn CategoryRepository,
        publication_repo: &'a dyn PublicationRepository,
    ) -> Self {
        Create {
            event_pub,
            author_repo,
            category_repo,
            publication_repo,
        }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        cmd: CreateCommand,
    ) -> Result<CreateResponse> {
        if !auth_role.can("create_publication") {
            return Err(Error::unauthorized());
        }

        let name = Name::new(cmd.name)?;
        let synopsis = Synopsis::new(cmd.synopsis)?;

        let mut tags = Vec::new();
        for tag in cmd.tags.into_iter() {
            tags.push(Tag::new(tag)?);
        }

        let cover = Image::new(cmd.cover)?;

        let category_id = CategoryId::new(cmd.category_id)?;
        self.category_repo.find_by_id(&category_id).await?;

        let header = Header::new(name, synopsis, category_id, tags, cover)?;

        self.author_repo.find_by_id(&auth_id).await?;

        let mut publication =
            Publication::new(self.publication_repo.next_id().await?, auth_id, header)?;

        // Add pages
        if let Some(page_dtos) = cmd.pages {
            let mut pages = Vec::new();
            for (page_n, page) in page_dtos.into_iter().enumerate() {
                let mut images = Vec::new();
                for image in page.images.into_iter() {
                    images.push(Image::new(image)?);
                }

                let mut page = Page::new(page_n as u32)?;
                page.set_images(images)?;

                pages.push(page);
            }

            publication.set_pages(pages)?;
        }

        self.publication_repo.save(&mut publication).await?;

        self.event_pub
            .publish_all(publication.events().to_vec()?)
            .await?;

        Ok(CreateResponse {
            id: publication.base().id().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::publication::PublicationId;
    use crate::mocks;

    #[tokio::test]
    async fn valid() {
        let c = mocks::container();
        let uc = Create::new(
            c.event_pub(),
            c.author_repo(),
            c.category_repo(),
            c.publication_repo(),
        );

        let mut author = mocks::author("#user01", "user-1");
        c.author_repo().save(&mut author).await.unwrap();
        let mut category = mocks::category("Category 1");
        c.category_repo().save(&mut category).await.unwrap();

        let res = uc
            .exec(
                author.base().id().to_string(),
                CreateCommand {
                    name: "Publication 1".to_owned(),
                    synopsis: "Synopsis...".to_owned(),
                    category_id: category.base().id().to_string(),
                    tags: vec!["Tag 1".to_owned()],
                    cover: "cover.com/cover.jpg".to_owned(),
                    pages: Some(vec![
                        PageDto {
                            images: vec![
                                "http://domain.com/image1.jpg".to_owned(),
                                "http://domain.com/image2.jpg".to_owned(),
                            ],
                        },
                        PageDto {
                            images: vec![
                                "http://domain.com/image3.jpg".to_owned(),
                                "http://domain.com/image4.jpg".to_owned(),
                            ],
                        },
                    ]),
                },
            )
            .await
            .unwrap();

        let publication = c
            .publication_repo()
            .find_by_id(&PublicationId::new(&res.id).unwrap())
            .await
            .unwrap();
        assert_eq!(publication.base().id().value(), res.id);
        assert_eq!(publication.header().name().value(), "Publication 1");
        assert_eq!(publication.header().synopsis().value(), "Synopsis...");
        assert_eq!(publication.pages().len(), 2);

        assert!(!c.event_pub().events().await.is_empty());
    }

    #[tokio::test]
    async fn invalid_data() {
        let c = mocks::container();
        let uc = Create::new(
            c.event_pub(),
            c.author_repo(),
            c.category_repo(),
            c.publication_repo(),
        );

        let mut author = mocks::author("#user01", "user-1");
        c.author_repo().save(&mut author).await.unwrap();
        let mut category = mocks::category("Category 1");
        c.category_repo().save(&mut category).await.unwrap();

        assert!(uc
            .exec(
                author.base().id().to_string(),
                CreateCommand {
                    name: "".to_owned(),
                    synopsis: "Synopsis...".to_owned(),
                    category_id: category.base().id().to_string(),
                    tags: vec!["Tag 1".to_owned()],
                    cover: "cover.com/cover.jpg".to_owned(),
                    pages: None,
                }
            )
            .await
            .is_err());

        assert!(uc
            .exec(
                author.base().id().to_string(),
                CreateCommand {
                    name: "Publication 1".to_owned(),
                    synopsis: "".to_owned(),
                    category_id: category.base().id().to_string(),
                    tags: vec!["Tag 1".to_owned()],
                    cover: "cover.com/cover.jpg".to_owned(),
                    pages: None,
                }
            )
            .await
            .is_err());
    }

    #[tokio::test]
    async fn not_existing_category() {
        let c = mocks::container();
        let uc = Create::new(
            c.event_pub(),
            c.author_repo(),
            c.category_repo(),
            c.publication_repo(),
        );

        let mut author = mocks::author("#user01", "user-1");
        c.author_repo().save(&mut author).await.unwrap();

        assert!(uc
            .exec(
                author.base().id().to_string(),
                CreateCommand {
                    name: "Publication 1".to_owned(),
                    synopsis: "Synopsis...".to_owned(),
                    category_id: "category-1".to_owned(),
                    tags: vec!["Tag 1".to_owned()],
                    cover: "cover.com/cover.jpg".to_owned(),
                    pages: None,
                },
            )
            .await
            .is_err());
    }
}
