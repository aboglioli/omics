use serde::Deserialize;

use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::publication::{Image, Page, PublicationId, PublicationRepository};

#[derive(Deserialize)]
pub struct PageDto {
    pub images: Vec<String>,
}

#[derive(Deserialize)]
pub struct UpdatePagesCommand {
    pub pages: Vec<PageDto>,
}

pub struct UpdatePages<'a> {
    event_pub: &'a dyn EventPublisher,

    publication_repo: &'a dyn PublicationRepository,
}

impl<'a> UpdatePages<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        publication_repo: &'a dyn PublicationRepository,
    ) -> Self {
        UpdatePages {
            event_pub,
            publication_repo,
        }
    }

    pub async fn exec(
        &self,
        auth_id: String,
        publication_id: String,
        cmd: UpdatePagesCommand,
    ) -> Result<CommandResponse> {
        let publication_id = PublicationId::new(&publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        if publication.author_id().value() != auth_id {
            return Err(Error::not_owner("publication"));
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
            .publish_all(publication.events().to_vec()?)
            .await?;

        Ok(CommandResponse::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::mocks;

    #[tokio::test]
    async fn valid() {
        let c = mocks::container();
        let uc = UpdatePages::new(c.event_pub(), c.publication_repo());

        let mut publication = mocks::publication(
            "#publication01",
            "#user01",
            "Publication 01",
            "#category01",
            vec!["Tag 1", "Tag 2"],
            "domain.com/cover.jpg",
            3,
            false,
            false,
            false,
        );
        c.publication_repo().save(&mut publication).await.unwrap();

        uc.exec(
            "#user01".to_owned(),
            publication.base().id().to_string(),
            UpdatePagesCommand {
                pages: vec![
                    PageDto {
                        images: vec![
                            "domain.com/image.jpg".to_owned(),
                            "domain.com/image2.jpg".to_owned(),
                        ],
                    },
                    PageDto {
                        images: vec![
                            "domain.com/image3.jpg".to_owned(),
                            "domain.com/image4.jpg".to_owned(),
                        ],
                    },
                    PageDto {
                        images: vec!["domain.com/image5.jpg".to_owned()],
                    },
                ],
            },
        )
        .await
        .unwrap();

        let publication = c
            .publication_repo()
            .find_by_id(publication.base().id())
            .await
            .unwrap();
        assert_eq!(publication.pages().len(), 3);

        assert_eq!(c.event_pub().events().await.len(), 1);
    }

    #[tokio::test]
    async fn invalid() {
        let c = mocks::container();
        let uc = UpdatePages::new(c.event_pub(), c.publication_repo());

        assert!(uc
            .exec(
                "#user01".to_owned(),
                "#invalid".to_owned(),
                UpdatePagesCommand {
                    pages: vec![
                        PageDto {
                            images: vec![
                                "domain.com/image.jpg".to_owned(),
                                "domain.com/image2.jpg".to_owned()
                            ],
                        },
                        PageDto {
                            images: vec![
                                "domain.com/image3.jpg".to_owned(),
                                "domain.com/image4.jpg".to_owned()
                            ],
                        },
                        PageDto {
                            images: vec!["domain.com/image5.jpg".to_owned()],
                        },
                    ],
                },
            )
            .await
            .is_err());
    }
}
