use async_trait::async_trait;

use common::result::Result;

use crate::domain::catalogue::{Author, Category, Publication, PublicationService, Statistics};
use publishing::domain::author::AuthorRepository;
use publishing::domain::category::CategoryRepository;
use publishing::domain::publication::{PublicationId, PublicationRepository};

pub struct SyncPublicationService<'a, ARepo, CRepo, PRepo> {
    author_repo: &'a ARepo,
    category_repo: &'a CRepo,
    publication_repo: &'a PRepo,
}

impl<'a, ARepo, CRepo, PRepo> SyncPublicationService<'a, ARepo, CRepo, PRepo> {
    pub fn new(
        author_repo: &'a ARepo,
        category_repo: &'a CRepo,
        publication_repo: &'a PRepo,
    ) -> Self {
        SyncPublicationService {
            author_repo,
            category_repo,
            publication_repo,
        }
    }
}

#[async_trait]
impl<'a, ARepo, CRepo, PRepo> PublicationService for SyncPublicationService<'a, ARepo, CRepo, PRepo>
where
    ARepo: AuthorRepository + Sync + Send,
    CRepo: CategoryRepository + Sync + Send,
    PRepo: PublicationRepository + Sync + Send,
{
    async fn get_by_id(&self, id: &str) -> Result<Publication> {
        let publication_id = PublicationId::new(id)?;
        let publication = self.publication_repo.find_by_id(&publication_id).await?;
        let author = self.author_repo.find_by_id(publication.author_id()).await?;
        let category = self
            .category_repo
            .find_by_id(publication.header().category_id())
            .await?;

        Ok(Publication::new(
            publication.base().id().to_string(),
            Author::new(
                author.base().id().value(),
                author.username(),
                author.name(),
                author.lastname(),
                self.publication_repo
                    .find_by_author_id(&author.base().id())
                    .await?
                    .len(),
            )?,
            publication.header().name().to_string(),
            publication.header().synopsis().to_string(),
            Category::new(category.base().id().value(), category.name().value())?,
            publication
                .header()
                .tags()
                .iter()
                .map(|tag| tag.name().to_string())
                .collect(),
            publication.header().cover().url().to_string(),
            Statistics::new(
                publication.statistics().views(),
                publication.statistics().unique_views(),
                publication.statistics().readings(),
                publication.statistics().likes(),
                publication.statistics().reviews(),
                publication.statistics().stars(),
            )?,
            publication.has_contract(),
            publication.pages().len(),
        )?)
    }
}
