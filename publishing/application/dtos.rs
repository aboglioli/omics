use serde::Serialize;

use crate::domain::author::Author;
use crate::domain::category::Category;
use crate::domain::collection::Collection;
use crate::domain::interaction::Review;
use crate::domain::publication::{Page, Publication, Statistics};
use crate::domain::reader::Reader;

#[derive(Serialize)]
pub struct StatisticsDto {
    pub views: u32,
    pub unique_views: u32,
    pub readings: u32,
    pub likes: u32,
    pub reviews: u32,
    pub stars: f32,
}

impl StatisticsDto {
    pub fn new(statistics: &Statistics) -> Self {
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
pub struct AuthorDto {
    pub id: String,
    pub username: String,
    pub name: String,
    pub lastname: String,
    pub publications: Option<Vec<PublicationDto>>,
    pub publication_count: Option<usize>,
    pub collection_count: Option<usize>,
}

impl AuthorDto {
    pub fn new(author: &Author) -> Self {
        AuthorDto {
            id: author.base().id().value().to_owned(),
            username: author.username().to_owned(),
            name: author.name().to_owned(),
            lastname: author.lastname().to_owned(),
            publications: None,
            publication_count: None,
            collection_count: None,
        }
    }

    pub fn publications(mut self, publications: Vec<PublicationDto>) -> Self {
        self.publications = Some(publications);
        self
    }

    pub fn publication_count(mut self, count: usize) -> Self {
        self.publication_count = Some(count);
        self
    }

    pub fn collection_count(mut self, count: usize) -> Self {
        self.collection_count = Some(count);
        self
    }
}

#[derive(Serialize)]
pub struct CategoryDto {
    pub id: String,
    pub name: String,
    pub publications: Option<Vec<PublicationDto>>,
}

impl CategoryDto {
    pub fn new(category: &Category) -> Self {
        CategoryDto {
            id: category.base().id().value().to_owned(),
            name: category.name().value().to_owned(),
            publications: None,
        }
    }

    pub fn publications(mut self, publications: Vec<PublicationDto>) -> Self {
        self.publications = Some(publications);
        self
    }
}

#[derive(Serialize)]
pub struct ImageDto {
    pub url: String,
}

#[derive(Serialize)]
pub struct PageDto {
    pub number: u32,
    pub images: Vec<ImageDto>,
}

impl PageDto {
    pub fn new(pages: &[Page]) -> Vec<Self> {
        pages
            .iter()
            .map(|page| PageDto {
                number: page.number(),
                images: page
                    .images()
                    .iter()
                    .map(|image| ImageDto {
                        url: image.url().to_owned(),
                    })
                    .collect(),
            })
            .collect()
    }
}

#[derive(Serialize)]
pub struct PublicationDto {
    pub id: String,
    pub author: AuthorDto,
    pub name: String,
    pub synopsis: String,
    pub category: CategoryDto,
    pub tags: Vec<String>,
    pub statistics: StatisticsDto,
    pub pages: Option<Vec<PageDto>>,
    pub status: Option<String>,
}

impl PublicationDto {
    pub fn new(
        publication: &Publication,
        author: AuthorDto,
        category: CategoryDto,
        include_pages: bool,
        include_status: bool,
    ) -> Self {
        PublicationDto {
            id: publication.base().id().value().to_owned(),
            author,
            name: publication.header().name().value().to_owned(),
            synopsis: publication.header().synopsis().value().to_owned(),
            category,
            tags: publication
                .header()
                .tags()
                .iter()
                .map(|tag| tag.name().to_owned())
                .collect(),
            statistics: StatisticsDto::new(publication.statistics()),
            pages: if include_pages {
                Some(PageDto::new(publication.pages()))
            } else {
                None
            },
            status: if include_status {
                Some(publication.status_history().current().status().to_string())
            } else {
                None
            },
        }
    }
}

#[derive(Serialize)]
pub struct CollectionDto {
    pub id: String,
    pub author: AuthorDto,
    pub name: String,
    pub synopsis: String,
    pub category: CategoryDto,
    pub tags: Vec<String>,
    pub publications: Vec<PublicationDto>,
}

impl CollectionDto {
    pub fn new(
        collection: &Collection,
        author: AuthorDto,
        category: CategoryDto,
        publications: Vec<PublicationDto>,
    ) -> Self {
        CollectionDto {
            id: collection.base().id().value().to_owned(),
            author,
            name: collection.header().name().value().to_owned(),
            synopsis: collection.header().synopsis().value().to_owned(),
            category,
            tags: collection
                .header()
                .tags()
                .iter()
                .map(|tag| tag.name().to_owned())
                .collect(),
            publications,
        }
    }
}

#[derive(Serialize)]
pub struct ReviewDto {
    pub reader: ReaderDto,
    pub publication_id: String,
    pub stars: u8,
    pub comment: String,
}

impl ReviewDto {
    pub fn new(review: &Review, reader: ReaderDto) -> Self {
        ReviewDto {
            reader,
            publication_id: review.base().publication_id().value().to_owned(),
            stars: review.stars().value(),
            comment: review.comment().value().to_owned(),
        }
    }
}

#[derive(Serialize)]
pub struct ReaderDto {
    pub id: String,
    pub username: String,
    pub name: String,
    pub lastname: String,
    pub subscribed: bool,
}

impl ReaderDto {
    pub fn new(reader: &Reader) -> Self {
        ReaderDto {
            id: reader.base().id().value().to_owned(),
            username: reader.username().to_owned(),
            name: reader.name().to_owned(),
            lastname: reader.lastname().to_owned(),
            subscribed: reader.is_subscribed(),
        }
    }
}
