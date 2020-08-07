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
pub struct ImageDto {
    id: String,
    url: String,
    size: u32,
    // frames: Vec<FrameDto>, // TODO: not considering frames right now
}

#[derive(Serialize)]
pub struct PageDto {
    number: u32,
    images: Vec<ImageDto>,
}

#[derive(Serialize)]
pub struct ReadResponse {
    id: String,
    name: String,
    synopsis: String,
    author_id: String,
    statistics: StatisticsDto,
    pages: Vec<PageDto>,
    category_id: String,
    tags: Vec<String>,
    status: String,
}

impl From<Publication> for ReadResponse {
    fn from(publication: Publication) -> Self {
        let stats = publication.statistics();
        let statistics = StatisticsDto {
            likes: stats.likes(),
            views: stats.views(),
            unique_views: stats.unique_views(),
            readings: stats.readings(),
        };

        let mut pages = Vec::new();
        for page in publication.pages().iter() {
            let mut images = Vec::new();
            for image in page.images().iter() {
                images.push(ImageDto {
                    id: image.id().to_owned(),
                    url: image.url().to_owned(),
                    size: image.size(),
                });
            }

            pages.push(PageDto {
                number: *page.number(),
                images,
            });
        }

        let tags: Vec<String> = publication
            .tags()
            .iter()
            .map(|t| t.name().to_owned())
            .collect();

        ReadResponse {
            id: publication.base().id(),
            name: publication.name().value().to_owned(),
            synopsis: publication.synopsis().value().to_owned(),
            author_id: publication.author_id().to_owned(),
            statistics,
            pages,
            category_id: publication.category_id().to_owned(),
            tags,
            status: publication.status().current().unwrap().status().to_string(),
        }
    }
}

pub struct Read<'a, PRepo, RRepo, IRepo> {
    publication_repo: &'a PRepo,
    reader_repo: &'a RRepo,
    interaction_repo: &'a IRepo,
}

impl<'a, PRepo, RRepo, IRepo> Read<'a, PRepo, RRepo, IRepo>
where
    PRepo: PublicationRepository,
    RRepo: ReaderRepository,
    IRepo: InteractionRepository,
{
    pub fn new(
        publication_repo: &'a PRepo,
        reader_repo: &'a RRepo,
        interaction_repo: &'a IRepo,
    ) -> Self {
        Read {
            publication_repo,
            reader_repo,
            interaction_repo,
        }
    }

    pub async fn exec(
        &self,
        reader_id: &ReaderId,
        publication_id: &PublicationId,
    ) -> Result<ReadResponse> {
        let publication = self.publication_repo.find_by_id(publication_id).await?;
        let reader = self.reader_repo.find_by_id(reader_id).await?;

        let mut read = reader.read(&publication)?;
        self.interaction_repo.save_read(&mut read).await?;

        Ok(ReadResponse::from(publication))
    }
}
