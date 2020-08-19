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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::mocks;

    #[tokio::test]
    async fn valid() {
        let c = mocks::container();
        let uc = UpdatePages::new(c.event_pub(), c.publication_repo());

        let author = mocks::author1();
        let mut publication = mocks::publication1();
        c.publication_repo().save(&mut publication).await.unwrap();

        uc.exec(
            author.base().id().to_string(),
            publication.base().id().to_string(),
            UpdatePagesCommand {
                pages: vec![
                    PageDto {
                        images: vec![
                            "domain.com/image1".to_owned(),
                            "domain.com/image2".to_owned(),
                        ],
                    },
                    PageDto {
                        images: vec![
                            "domain.com/image3".to_owned(),
                            "domain.com/image4".to_owned(),
                        ],
                    },
                    PageDto {
                        images: vec!["domain.com/image5".to_owned()],
                    },
                ],
            },
        )
        .await
        .unwrap();

        let publication = c
            .publication_repo()
            .find_by_id(&publication.base().id())
            .await
            .unwrap();
        assert_eq!(publication.pages().len(), 3);

        assert_eq!(c.event_pub().events().await.len(), 1);
    }

    #[tokio::test]
    async fn invalid() {
        let c = mocks::container();
        let uc = UpdatePages::new(c.event_pub(), c.publication_repo());

        let author = mocks::author1();

        assert!(uc
            .exec(
                author.base().id().to_string(),
                "#invalid".to_owned(),
                UpdatePagesCommand {
                    pages: vec![
                        PageDto {
                            images: vec![
                                "domain.com/image1".to_owned(),
                                "domain.com/image2".to_owned()
                            ],
                        },
                        PageDto {
                            images: vec![
                                "domain.com/image3".to_owned(),
                                "domain.com/image4".to_owned()
                            ],
                        },
                        PageDto {
                            images: vec!["domain.com/image5".to_owned()],
                        },
                    ],
                },
            )
            .await
            .is_err());
    }
}
