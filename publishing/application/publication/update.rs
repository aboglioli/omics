use serde::Deserialize;

use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use identity::UserIdAndRole;

use crate::domain::category::{CategoryId, CategoryRepository};
use crate::domain::publication::{
    Header, Image, Name, Page, PublicationId, PublicationRepository, Synopsis, Tag,
};

#[derive(Deserialize)]
pub struct PageDto {
    images: Vec<String>,
}

#[derive(Deserialize)]
pub struct UpdateCommand {
    pub name: String,
    pub synopsis: String,
    pub category_id: String,
    pub tags: Vec<String>,
    pub cover: String,
    pub pages: Option<Vec<PageDto>>,
}

pub struct Update<'a> {
    event_pub: &'a dyn EventPublisher,

    category_repo: &'a dyn CategoryRepository,
    publication_repo: &'a dyn PublicationRepository,
}

impl<'a> Update<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        category_repo: &'a dyn CategoryRepository,
        publication_repo: &'a dyn PublicationRepository,
    ) -> Self {
        Update {
            event_pub,
            category_repo,
            publication_repo,
        }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        publication_id: String,
        cmd: UpdateCommand,
    ) -> Result<CommandResponse> {
        if !auth_role.can("update_publication") {
            return Err(Error::unauthorized());
        }

        let publication_id = PublicationId::new(publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        if publication.author_id() != &auth_id {
            return Err(Error::not_owner("publication"));
        }

        let name = Name::new(cmd.name)?;
        let synopsis = Synopsis::new(cmd.synopsis)?;

        let mut tags = Vec::new();
        for tag in cmd.tags.iter() {
            tags.push(Tag::new(tag)?);
        }

        let cover = Image::new(cmd.cover)?;

        let category_id = CategoryId::new(cmd.category_id)?;
        self.category_repo.find_by_id(&category_id).await?;

        let header = Header::new(name, synopsis, category_id, tags, cover)?;

        publication.set_header(header)?;

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

        Ok(CommandResponse::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use identity::domain::user::UserId;
    use identity::mocks as identity_mocks;

    use crate::domain::publication::Status;
    use crate::mocks;

    #[tokio::test]
    async fn valid() {
        let c = mocks::container();
        let uc = Update::new(c.event_pub(), c.category_repo(), c.publication_repo());

        let mut publication = mocks::publication(
            "#publication01",
            "#user01",
            "Publication 01",
            "category-1",
            vec!["Tag 1", "Tag 2"],
            "domain.com/cover.jpg",
            3,
            false,
            false,
            false,
        );
        c.publication_repo().save(&mut publication).await.unwrap();
        let mut category = mocks::category("Category 2");
        c.category_repo().save(&mut category).await.unwrap();
        let role = identity_mocks::role("User");

        uc.exec(
            (UserId::new("#user01").unwrap(), role),
            publication.base().id().to_string(),
            UpdateCommand {
                name: "New name".to_owned(),
                synopsis: "New synopsis...".to_owned(),
                category_id: category.base().id().to_string(),
                tags: vec!["New tag".to_owned()],
                cover: "domain.com/new-cover.jpg".to_owned(),
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
            .find_by_id(publication.base().id())
            .await
            .unwrap();
        assert_eq!(publication.header().name().value(), "New name");
        assert_eq!(publication.header().synopsis().value(), "New synopsis...");
        assert_eq!(publication.header().category_id().value(), "category-2");
        assert!(matches!(
            publication.status_history().current(),
            Status::Draft
        ));
        assert_eq!(publication.pages().len(), 2);

        assert!(!c.event_pub().events().await.is_empty());
    }

    #[tokio::test]
    async fn published_publication() {
        let c = mocks::container();
        let uc = Update::new(c.event_pub(), c.category_repo(), c.publication_repo());

        let mut publication = mocks::publication(
            "#publication01",
            "#user01",
            "Publication 01",
            "category-1",
            vec!["Tag 1", "Tag 2"],
            "domain.com/cover.jpg",
            3,
            true,
            true,
            false,
        );
        c.publication_repo().save(&mut publication).await.unwrap();
        let mut category = mocks::category("Category 2");
        c.category_repo().save(&mut category).await.unwrap();
        let role = identity_mocks::role("User");

        uc.exec(
            (UserId::new("#user01").unwrap(), role),
            publication.base().id().to_string(),
            UpdateCommand {
                name: "New name".to_owned(),
                synopsis: "New synopsis...".to_owned(),
                category_id: category.base().id().to_string(),
                tags: vec!["New tag".to_owned()],
                cover: "domain.com/new-cover.jpg".to_owned(),
                pages: None,
            },
        )
        .await
        .unwrap();

        let publication = c
            .publication_repo()
            .find_by_id(publication.base().id())
            .await
            .unwrap();
        assert!(matches!(
            publication.status_history().current(),
            Status::Draft
        ));
    }

    #[tokio::test]
    async fn not_owner() {
        let c = mocks::container();
        let uc = Update::new(c.event_pub(), c.category_repo(), c.publication_repo());

        let mut publication = mocks::publication(
            "#publication01",
            "#user01",
            "Publication 01",
            "category-1",
            vec!["Tag 1", "Tag 2"],
            "domain.com/cover.jpg",
            3,
            false,
            false,
            false,
        );
        c.publication_repo().save(&mut publication).await.unwrap();
        let mut category = mocks::category("Category 2");
        c.category_repo().save(&mut category).await.unwrap();
        let role = identity_mocks::role("User");

        assert!(uc
            .exec(
                (UserId::new("#user02").unwrap(), role),
                publication.base().id().to_string(),
                UpdateCommand {
                    name: "New name".to_owned(),
                    synopsis: "New synopsis...".to_owned(),
                    category_id: category.base().id().to_string(),
                    tags: vec!["New tag".to_owned()],
                    cover: "domain.com/new-cover.jpg".to_owned(),
                    pages: None,
                },
            )
            .await
            .is_err());
    }

    #[tokio::test]
    async fn non_existing_category() {
        let c = mocks::container();
        let uc = Update::new(c.event_pub(), c.category_repo(), c.publication_repo());

        let mut publication = mocks::publication(
            "#publication01",
            "#user01",
            "Publication 01",
            "category-1",
            vec!["Tag 1", "Tag 2"],
            "domain.com/cover.jpg",
            3,
            false,
            false,
            false,
        );
        c.publication_repo().save(&mut publication).await.unwrap();
        let role = identity_mocks::role("User");

        assert!(uc
            .exec(
                (UserId::new("#user01").unwrap(), role),
                publication.base().id().to_string(),
                UpdateCommand {
                    name: "New name".to_owned(),
                    synopsis: "New synopsis...".to_owned(),
                    category_id: "category-2".to_owned(),
                    tags: vec!["New tag".to_owned()],
                    cover: "domain.com/new-cover.jpg".to_owned(),
                    pages: None,
                },
            )
            .await
            .is_err());
    }
}
