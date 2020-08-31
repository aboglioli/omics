use serde::Serialize;

use crate::domain::author::Author;
use crate::domain::category::Category;
use crate::domain::collection::Collection;
use crate::domain::interaction::Review;
use crate::domain::publication::{Image, Page, Publication, Statistics};
use crate::domain::reader::{Preferences, Reader};

#[derive(Serialize)]
pub struct StatisticsDto {
    pub views: u32,
    pub unique_views: u32,
    pub readings: u32,
    pub likes: u32,
    pub reviews: u32,
    pub stars: f32,
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
pub struct AuthorDto {
    pub id: String,
    pub username: String,
    pub name: String,
    pub lastname: String,
}

impl From<&Author> for AuthorDto {
    fn from(author: &Author) -> Self {
        AuthorDto {
            id: author.base().id().to_string(),
            username: author.username().to_string(),
            name: author.name().to_string(),
            lastname: author.lastname().to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct CategoryDto {
    pub id: String,
    pub name: String,
    pub publications: Option<Vec<PublicationDto>>,
}

impl From<&Category> for CategoryDto {
    fn from(category: &Category) -> Self {
        CategoryDto {
            id: category.base().id().to_string(),
            name: category.name().to_string(),
            publications: None,
        }
    }
}

impl CategoryDto {
    pub fn publications(mut self, publications: Vec<PublicationDto>) -> Self {
        self.publications = Some(publications);
        self
    }
}

#[derive(Serialize)]
pub struct ImageDto {
    pub url: String,
}

impl From<&Image> for ImageDto {
    fn from(image: &Image) -> Self {
        ImageDto {
            url: image.url().to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct PageDto {
    pub number: u32,
    pub images: Vec<ImageDto>,
}

impl From<&Page> for PageDto {
    fn from(page: &Page) -> Self {
        PageDto {
            number: page.number(),
            images: page.images().iter().map(ImageDto::from).collect(),
        }
    }
}

#[derive(Serialize)]
pub struct PublicationDto {
    pub id: String,
    pub author_id: Option<String>,
    pub author: Option<AuthorDto>,
    pub name: String,
    pub synopsis: String,
    pub category_id: Option<String>,
    pub category: Option<CategoryDto>,
    pub tags: Vec<String>,
    pub statistics: StatisticsDto,
    pub pages: Option<Vec<PageDto>>,
    pub status: Option<String>,
}

impl From<&Publication> for PublicationDto {
    fn from(publication: &Publication) -> Self {
        PublicationDto {
            id: publication.base().id().to_string(),
            author_id: Some(publication.author_id().to_string()),
            author: None,
            name: publication.header().name().to_string(),
            synopsis: publication.header().synopsis().to_string(),
            category_id: Some(publication.header().category_id().to_string()),
            category: None,
            tags: publication
                .header()
                .tags()
                .iter()
                .map(|tag| tag.name().to_string())
                .collect(),
            statistics: StatisticsDto::from(publication.statistics()),
            pages: None,
            status: None,
        }
    }
}

impl PublicationDto {
    pub fn author(mut self, author: AuthorDto) -> Self {
        self.author_id = None;
        self.author = Some(author);
        self
    }

    pub fn category(mut self, category: CategoryDto) -> Self {
        self.category_id = None;
        self.category = Some(category);
        self
    }

    pub fn pages(mut self, publication: &Publication) -> Self {
        self.pages = Some(publication.pages().iter().map(PageDto::from).collect());
        self
    }

    pub fn status(mut self, publication: &Publication) -> Self {
        self.status = Some(publication.status_history().current().to_string());
        self
    }
}

#[derive(Serialize)]
pub struct CollectionDto {
    pub id: String,
    pub author_id: Option<String>,
    pub author: Option<AuthorDto>,
    pub name: String,
    pub synopsis: String,
    pub category_id: Option<String>,
    pub category: Option<CategoryDto>,
    pub tags: Vec<String>,
}

impl From<&Collection> for CollectionDto {
    fn from(collection: &Collection) -> Self {
        CollectionDto {
            id: collection.base().id().to_string(),
            author_id: Some(collection.author_id().to_string()),
            author: None,
            name: collection.header().name().to_string(),
            synopsis: collection.header().synopsis().to_string(),
            category_id: Some(collection.header().category_id().to_string()),
            category: None,
            tags: collection
                .header()
                .tags()
                .iter()
                .map(|tag| tag.name().to_string())
                .collect(),
        }
    }
}

impl CollectionDto {
    pub fn author(mut self, author: AuthorDto) -> Self {
        self.author_id = None;
        self.author = Some(author);
        self
    }

    pub fn category(mut self, category: CategoryDto) -> Self {
        self.category_id = None;
        self.category = Some(category);
        self
    }
}

#[derive(Serialize)]
pub struct ReviewDto {
    pub reader_id: Option<String>,
    pub reader: Option<ReaderDto>,
    pub publication_id: String,
    pub stars: u8,
    pub comment: String,
}

impl From<&Review> for ReviewDto {
    fn from(review: &Review) -> Self {
        ReviewDto {
            reader_id: None,
            reader: None,
            publication_id: review.base().publication_id().to_string(),
            stars: review.stars().value(),
            comment: review.comment().to_string(),
        }
    }
}

impl ReviewDto {
    pub fn reader_id(mut self, review: &Review) -> Self {
        self.reader_id = Some(review.base().reader_id().to_string());
        self
    }

    pub fn reader(mut self, review: ReaderDto) -> Self {
        self.reader = Some(review);
        self
    }
}

#[derive(Serialize)]
pub struct PreferencesDto {
    pub categories: Vec<String>,
    pub publications: Vec<String>,
}

impl From<&Preferences> for PreferencesDto {
    fn from(preferences: &Preferences) -> Self {
        PreferencesDto {
            categories: preferences
                .category_ids()
                .iter()
                .map(|category_id| category_id.to_string())
                .collect(),
            publications: preferences
                .publication_ids()
                .iter()
                .map(|publication_id| publication_id.to_string())
                .collect(),
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
    pub preferences: Option<PreferencesDto>,
}

impl From<&Reader> for ReaderDto {
    fn from(reader: &Reader) -> Self {
        ReaderDto {
            id: reader.base().id().to_string(),
            username: reader.username().to_string(),
            name: reader.name().to_string(),
            lastname: reader.lastname().to_string(),
            subscribed: reader.is_subscribed(),
            preferences: None,
        }
    }
}

impl ReaderDto {
    pub fn preferences(mut self, reader: &Reader) -> Self {
        self.preferences = Some(PreferencesDto::from(reader.preferences()));
        self
    }
}

#[derive(Serialize)]
pub struct ReaderInteractionDto {
    pub viewed: bool,
    pub read: bool,
    pub liked: bool,
    pub reviewed: bool,
}

impl ReaderInteractionDto {
    pub fn new(viewed: bool, read: bool, liked: bool, reviewed: bool) -> Self {
        ReaderInteractionDto {
            viewed,
            read,
            liked,
            reviewed,
        }
    }
}
