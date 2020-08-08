use serde::Serialize;

use common::result::Result;

use crate::domain::interaction::InteractionRepository;
use crate::domain::publication::{Publication, PublicationId, PublicationRepository};
use crate::domain::reader::{ReaderId, ReaderRepository};

#[derive(Serialize)]
pub struct StatisticsDto {
    likes: u32,
    views: u32,
    unique_views: u32,
    readings: u32,
}

#[derive(Serialize)]
pub struct ViewResponse {
    id: String,
    name: String,
    synopsis: String,
    author_id: String,
    statistics: StatisticsDto,
    category_id: String,
    tags: Vec<String>,
}

impl From<Publication> for ViewResponse {
    fn from(publication: Publication) -> Self {
        let stats = publication.statistics();
        let statistics = StatisticsDto {
            likes: stats.likes(),
            views: stats.views(),
            unique_views: stats.unique_views(),
            readings: stats.readings(),
        };

        let tags: Vec<String> = publication
            .tags()
            .iter()
            .map(|t| t.name().to_owned())
            .collect();

        ViewResponse {
            id: publication.base().id(),
            name: publication.name().value().to_owned(),
            synopsis: publication.synopsis().value().to_owned(),
            author_id: publication.author_id().to_owned(),
            statistics,
            category_id: publication.category_id().to_owned(),
            tags,
        }
    }
}

pub struct View<'a, IRepo, PRepo, RRepo> {
    interaction_repo: &'a IRepo,
    publication_repo: &'a PRepo,
    reader_repo: &'a RRepo,
}

impl<'a, IRepo, PRepo, RRepo> View<'a, IRepo, PRepo, RRepo>
where
    IRepo: InteractionRepository,
    PRepo: PublicationRepository,
    RRepo: ReaderRepository,
{
    pub fn new(
        interaction_repo: &'a IRepo,
        publication_repo: &'a PRepo,
        reader_repo: &'a RRepo,
    ) -> Self {
        View {
            interaction_repo,
            publication_repo,
            reader_repo,
        }
    }

    pub async fn exec(
        &self,
        reader_id: &ReaderId,
        publication_id: &PublicationId,
    ) -> Result<ViewResponse> {
        let publication = self.publication_repo.find_by_id(publication_id).await?;
        let reader = self.reader_repo.find_by_id(reader_id).await?;

        let mut view = reader.view(&publication)?;
        self.interaction_repo.save_view(&mut view).await?;

        Ok(ViewResponse::from(publication))
    }
}
