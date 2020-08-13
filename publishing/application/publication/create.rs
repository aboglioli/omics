use serde::{Deserialize, Serialize};

use common::event::EventPublisher;
use common::result::Result;

use crate::domain::author::AuthorId;
use crate::domain::category::{CategoryId, CategoryRepository};
use crate::domain::publication::{
    Header, Image, Name, Publication, PublicationRepository, Synopsis, Tag,
};

#[derive(Deserialize)]
pub struct CreateCommand {
    name: String,
    synopsis: String,
    category_id: String,
    tags: Vec<String>,
    cover: String,
}

impl CreateCommand {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Serialize)]
pub struct CreateResponse {
    id: String,
}

pub struct Create<'a, EPub, CRepo, PRepo> {
    event_pub: &'a EPub,

    category_repo: &'a CRepo,
    publication_repo: &'a PRepo,
}

impl<'a, EPub, CRepo, PRepo> Create<'a, EPub, CRepo, PRepo>
where
    EPub: EventPublisher,
    CRepo: CategoryRepository,
    PRepo: PublicationRepository,
{
    pub fn new(event_pub: &'a EPub, category_repo: &'a CRepo, publication_repo: &'a PRepo) -> Self {
        Create {
            event_pub,
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

        let mut publication = Publication::new(
            self.publication_repo.next_id().await?,
            AuthorId::new(author_id)?,
            header,
        )?;

        self.publication_repo.save(&mut publication).await?;

        self.event_pub
            .publish_all(publication.base().events()?)
            .await?;

        Ok(CreateResponse {
            id: publication.base().id().value().to_owned(),
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
        let uc = Create::new(c.event_pub(), c.category_repo(), c.publication_repo());

        let author = mocks::author1();
        let mut category = mocks::category1();
        c.category_repo().save(&mut category).await.unwrap();

        let res = uc
            .exec(
                author.base().id().value().to_owned(),
                CreateCommand {
                    name: "Publication 1".to_owned(),
                    synopsis: "Synopsis...".to_owned(),
                    category_id: category.base().id().value().to_owned(),
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
        let uc = Create::new(c.event_pub(), c.category_repo(), c.publication_repo());

        let author = mocks::author1();
        let mut category = mocks::category1();
        c.category_repo().save(&mut category).await.unwrap();

        assert!(uc
            .exec(
                author.base().id().value().to_owned(),
                CreateCommand {
                    name: "".to_owned(),
                    synopsis: "Synopsis...".to_owned(),
                    category_id: category.base().id().value().to_owned(),
                    tags: vec!["Tag 1".to_owned()],
                    cover: "cover.com/cover.jpg".to_owned(),
                }
            )
            .await
            .is_err());

        assert!(uc
            .exec(
                author.base().id().value().to_owned(),
                CreateCommand {
                    name: "Publication 1".to_owned(),
                    synopsis: "".to_owned(),
                    category_id: category.base().id().value().to_owned(),
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
        let uc = Create::new(c.event_pub(), c.category_repo(), c.publication_repo());

        let author = mocks::author1();
        let category = mocks::category1();

        assert!(uc
            .exec(
                author.base().id().value().to_owned(),
                CreateCommand {
                    name: "Publication 1".to_owned(),
                    synopsis: "Synopsis...".to_owned(),
                    category_id: category.base().id().value().to_owned(),
                    tags: vec!["Tag 1".to_owned()],
                    cover: "cover.com/cover.jpg".to_owned(),
                },
            )
            .await
            .is_err());
    }
}
