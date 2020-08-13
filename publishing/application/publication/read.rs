use serde::Serialize;

use common::event::EventPublisher;
use common::result::Result;

use crate::domain::publication::{Publication, PublicationId, PublicationRepository};
use crate::domain::reader::{ReaderId, ReaderRepository};
use crate::domain::statistics::{Statistics, StatisticsRepository};

#[derive(Serialize)]
pub struct StatisticsDto {
    views: u32,
    unique_views: u32,
    readings: u32,
    likes: u32,
    reviews: u32,
    stars: f32,
}

impl From<&Statistics> for StatisticsDto {
    fn from(statistics: &Statistics) -> Self {
        StatisticsDto {
            views: statistics.views(),
            unique_views: statistics.unique_views(),
            readings: statistics.readings(),
            likes: statistics.likes(),
            reviews: statistics.reviews(),
            stars: statistics.stars(),
        }
    }
}

#[derive(Serialize)]
pub struct ImageDto {
    url: String,
}

#[derive(Serialize)]
pub struct PageDto {
    number: u32,
    images: Vec<ImageDto>,
}

#[derive(Serialize)]
pub struct ReadResponse {
    id: String,
    author_id: String,
    name: String,
    synopsis: String,
    pages: Vec<PageDto>,
    category_id: String,
    tags: Vec<String>,
    status: Option<String>,
    statistics: Option<StatisticsDto>,
}

impl From<&Publication> for ReadResponse {
    fn from(publication: &Publication) -> Self {
        let mut pages = Vec::new();
        for page in publication.pages().iter() {
            let mut images = Vec::new();
            for image in page.images().iter() {
                images.push(ImageDto {
                    url: image.url().to_owned(),
                });
            }

            pages.push(PageDto {
                number: *page.number(),
                images,
            });
        }

        let header = publication.header();

        let tags: Vec<String> = header.tags().iter().map(|t| t.name().to_owned()).collect();

        ReadResponse {
            id: publication.base().id().value().to_owned(),
            author_id: publication.author_id().value().to_owned(),
            name: header.name().value().to_owned(),
            synopsis: header.synopsis().value().to_owned(),
            category_id: header.category_id().value().to_owned(),
            tags,
            pages,
            status: None,
            statistics: None,
        }
    }
}

pub struct Read<'a, EPub, PRepo, RRepo, SRepo> {
    event_pub: &'a EPub,

    publication_repo: &'a PRepo,
    reader_repo: &'a RRepo,
    statistics_repo: &'a SRepo,
}

impl<'a, EPub, PRepo, RRepo, SRepo> Read<'a, EPub, PRepo, RRepo, SRepo>
where
    EPub: EventPublisher,
    PRepo: PublicationRepository,
    RRepo: ReaderRepository,
    SRepo: StatisticsRepository,
{
    pub fn new(
        event_pub: &'a EPub,
        publication_repo: &'a PRepo,
        reader_repo: &'a RRepo,
        statistics_repo: &'a SRepo,
    ) -> Self {
        Read {
            event_pub,
            publication_repo,
            reader_repo,
            statistics_repo,
        }
    }

    pub async fn exec(&self, reader_id: String, publication_id: String) -> Result<ReadResponse> {
        let publication_id = PublicationId::new(publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        let reader_id = ReaderId::new(reader_id)?;
        let reader = self.reader_repo.find_by_id(&reader_id).await?;

        publication.read(&reader)?;

        self.event_pub
            .publish_all(publication.base().events()?)
            .await?;

        let mut res = ReadResponse::from(&publication);

        if publication.author_id() == &reader_id {
            res.status = Some(publication.status_history().current().status().to_string());
        }

        let statistics = self
            .statistics_repo
            .find_by_publication_id(&publication.base().id())
            .await?;
        res.statistics = Some(StatisticsDto::from(&statistics));

        Ok(res)
    }
}
