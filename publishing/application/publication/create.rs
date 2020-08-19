use serde::{Deserialize, Serialize};

use common::event::EventPublisher;
use common::result::Result;

use crate::domain::author::{AuthorId, AuthorRepository};
use crate::domain::category::{CategoryId, CategoryRepository};
use crate::domain::publication::{
    Header, Image, Name, Publication, PublicationRepository, Synopsis, Tag,
};

#[derive(Deserialize)]
pub struct CreateCommand {
    pub name: String,
    pub synopsis: String,
    pub category_id: String,
    pub tags: Vec<String>,
    pub cover: String,
}

impl CreateCommand {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
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

    pub async fn exec(&self, author_id: String, cmd: CreateCommand) -> Result<CreateResponse> {
        cmd.validate()?;

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

        let author_id = AuthorId::new(author_id)?;
        self.author_repo.find_by_id(&author_id).await?;

        let mut publication =
            Publication::new(self.publication_repo.next_id().await?, author_id, header)?;

        self.publication_repo.save(&mut publication).await?;

        self.event_pub
            .publish_all(publication.base().events()?)
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

        let mut author = mocks::author1();
        c.author_repo().save(&mut author).await.unwrap();
        let mut category = mocks::category1();
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
        assert_eq!(publication.pages().len(), 0);

        assert_eq!(c.event_pub().events().await.len(), 1);
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

        let mut author = mocks::author1();
        c.author_repo().save(&mut author).await.unwrap();
        let mut category = mocks::category1();
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

        let mut author = mocks::author1();
        c.author_repo().save(&mut author).await.unwrap();
        let category = mocks::category1();

        assert!(uc
            .exec(
                author.base().id().to_string(),
                CreateCommand {
                    name: "Publication 1".to_owned(),
                    synopsis: "Synopsis...".to_owned(),
                    category_id: category.base().id().to_string(),
                    tags: vec!["Tag 1".to_owned()],
                    cover: "cover.com/cover.jpg".to_owned(),
                },
            )
            .await
            .is_err());
    }
}
