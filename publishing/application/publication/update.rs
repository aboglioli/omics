use serde::Deserialize;

use common::error::Error;
use common::event::EventPublisher;
use common::result::Result;

use crate::application::dtos::CommandResponse;
use crate::domain::category::{CategoryId, CategoryRepository};
use crate::domain::publication::{
    Header, Image, Name, PublicationId, PublicationRepository, Synopsis, Tag,
};

#[derive(Deserialize)]
pub struct UpdateCommand {
    name: String,
    synopsis: String,
    category_id: String,
    tags: Vec<String>,
    cover: String,
}

impl UpdateCommand {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
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
        auth_id: String,
        publication_id: String,
        cmd: UpdateCommand,
    ) -> Result<CommandResponse> {
        cmd.validate()?;

        let publication_id = PublicationId::new(publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        if publication.author_id().value() != auth_id {
            return Err(Error::unauthorized());
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

        self.publication_repo.save(&mut publication).await?;

        self.event_pub
            .publish_all(publication.base().events()?)
            .await?;

        Ok(CommandResponse::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::publication::Status;
    use crate::mocks;

    #[tokio::test]
    async fn valid() {
        let c = mocks::container();
        let uc = Update::new(c.event_pub(), c.category_repo(), c.publication_repo());

        let author = mocks::author1();
        let mut publication = mocks::publication1();
        c.publication_repo().save(&mut publication).await.unwrap();
        let mut category = mocks::category2();
        c.category_repo().save(&mut category).await.unwrap();

        uc.exec(
            author.base().id().to_string(),
            publication.base().id().to_string(),
            UpdateCommand {
                name: "New name".to_owned(),
                synopsis: "New synopsis...".to_owned(),
                category_id: category.base().id().to_string(),
                tags: vec!["New tag".to_owned()],
                cover: "domain.com/new-cover.jpg".to_owned(),
            },
        )
        .await
        .unwrap();

        let publication = c
            .publication_repo()
            .find_by_id(&publication.base().id())
            .await
            .unwrap();
        assert_eq!(publication.header().name().value(), "New name");
        assert_eq!(publication.header().synopsis().value(), "New synopsis...");
        assert_eq!(publication.header().category_id().value(), "#category02");
        assert!(matches!(
            publication.status_history().current().status(),
            Status::Draft
        ));

        assert_eq!(c.event_pub().events().await.len(), 1);
    }

    #[tokio::test]
    async fn published_publication() {
        let c = mocks::container();
        let uc = Update::new(c.event_pub(), c.category_repo(), c.publication_repo());

        let author = mocks::author1();
        let mut publication = mocks::published_publication1();
        c.publication_repo().save(&mut publication).await.unwrap();
        let mut category = mocks::category2();
        c.category_repo().save(&mut category).await.unwrap();

        uc.exec(
            author.base().id().to_string(),
            publication.base().id().to_string(),
            UpdateCommand {
                name: "New name".to_owned(),
                synopsis: "New synopsis...".to_owned(),
                category_id: category.base().id().to_string(),
                tags: vec!["New tag".to_owned()],
                cover: "domain.com/new-cover.jpg".to_owned(),
            },
        )
        .await
        .unwrap();

        let publication = c
            .publication_repo()
            .find_by_id(&publication.base().id())
            .await
            .unwrap();
        assert!(matches!(
            publication.status_history().current().status(),
            Status::Draft
        ));
    }

    #[tokio::test]
    async fn not_owner() {
        let c = mocks::container();
        let uc = Update::new(c.event_pub(), c.category_repo(), c.publication_repo());

        let author = mocks::author2();
        let mut publication = mocks::publication1();
        c.publication_repo().save(&mut publication).await.unwrap();
        let mut category = mocks::category2();
        c.category_repo().save(&mut category).await.unwrap();

        assert!(uc
            .exec(
                author.base().id().to_string(),
                publication.base().id().to_string(),
                UpdateCommand {
                    name: "New name".to_owned(),
                    synopsis: "New synopsis...".to_owned(),
                    category_id: category.base().id().to_string(),
                    tags: vec!["New tag".to_owned()],
                    cover: "domain.com/new-cover.jpg".to_owned(),
                },
            )
            .await
            .is_err());
    }

    #[tokio::test]
    async fn non_existing_category() {
        let c = mocks::container();
        let uc = Update::new(c.event_pub(), c.category_repo(), c.publication_repo());

        let author = mocks::author1();
        let mut publication = mocks::publication1();
        c.publication_repo().save(&mut publication).await.unwrap();
        let category = mocks::category2();

        assert!(uc
            .exec(
                author.base().id().to_string(),
                publication.base().id().to_string(),
                UpdateCommand {
                    name: "New name".to_owned(),
                    synopsis: "New synopsis...".to_owned(),
                    category_id: category.base().id().to_string(),
                    tags: vec!["New tag".to_owned()],
                    cover: "domain.com/new-cover.jpg".to_owned(),
                },
            )
            .await
            .is_err());
    }
}
