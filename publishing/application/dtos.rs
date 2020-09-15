use serde::Serialize;

use common::model::StatusItem;
use shared::domain::user::User;

use crate::domain::author::Author;
use crate::domain::category::Category;
use crate::domain::collection::Collection;
use crate::domain::interaction::Review;
use crate::domain::publication::{Image, Page, Publication, Statistics, Status};
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
    pub name: Option<String>,
    pub lastname: Option<String>,
    pub biography: Option<String>,
    pub profile_image: Option<String>,
    pub followers: u32,
    pub created_at: String,
    pub updated_at: Option<String>,
}

impl AuthorDto {
    pub fn from(user: &User, author: &Author) -> Self {
        AuthorDto {
            id: author.base().id().to_string(),
            username: user.username().to_string(),
            name: user.name().map(|name| name.to_string()),
            lastname: user.lastname().map(|lastname| lastname.to_string()),
            biography: user.biography().map(|biography| biography.to_string()),
            profile_image: user
                .profile_image()
                .map(|profile_image| profile_image.to_string()),
            followers: author.followers(),
            created_at: user.base().created_at().to_rfc3339(),
            updated_at: user.base().updated_at().map(|d| d.to_rfc3339()),
        }
    }
}

#[derive(Serialize)]
pub struct CategoryDto {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<&Category> for CategoryDto {
    fn from(category: &Category) -> Self {
        CategoryDto {
            id: category.base().id().to_string(),
            name: category.name().to_string(),
            created_at: category.base().created_at().to_rfc3339(),
            updated_at: category.base().updated_at().map(|d| d.to_rfc3339()),
        }
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
pub struct PublicationStatusDto {
    pub status: String,
    pub changed_at: String,
    pub changed_by: Option<String>,
    pub comment: Option<String>,
}

impl From<&StatusItem<Status>> for PublicationStatusDto {
    fn from(status_item: &StatusItem<Status>) -> Self {
        let status = status_item.status();

        let mut dto = PublicationStatusDto {
            status: status.to_string(),
            changed_at: status_item.date().to_rfc3339(),
            changed_by: None,
            comment: None,
        };

        match status {
            Status::Published { admin_id, comment } | Status::Rejected { admin_id, comment } => {
                dto.changed_by = Some(admin_id.to_string());
                dto.comment = Some(comment.to_string());
            }
            _ => {}
        }

        dto
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
    pub cover: String,
    pub statistics: StatisticsDto,
    pub pages: Option<Vec<PageDto>>,
    pub status: PublicationStatusDto,
    pub created_at: String,
    pub updated_at: Option<String>,
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
            cover: publication.header().cover().to_string(),
            statistics: StatisticsDto::from(publication.statistics()),
            pages: None,
            status: PublicationStatusDto::from(publication.status_history().current_item()),
            created_at: publication.base().created_at().to_rfc3339(),
            updated_at: publication.base().updated_at().map(|d| d.to_rfc3339()),
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
    pub cover: String,
    pub created_at: String,
    pub updated_at: Option<String>,
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
            cover: collection.header().cover().to_string(),
            created_at: collection.base().created_at().to_rfc3339(),
            updated_at: collection.base().updated_at().map(|d| d.to_rfc3339()),
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
    pub created_at: String,
}

impl From<&Review> for ReviewDto {
    fn from(review: &Review) -> Self {
        ReviewDto {
            reader_id: Some(review.base().id().reader_id().to_string()),
            reader: None,
            publication_id: review.base().id().publication_id().to_string(),
            stars: review.stars().value(),
            comment: review.comment().to_string(),
            created_at: review.base().created_at().to_rfc3339(),
        }
    }
}

impl ReviewDto {
    pub fn reader(mut self, review: ReaderDto) -> Self {
        self.reader_id = None;
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
    pub name: Option<String>,
    pub lastname: Option<String>,
    pub subscribed: bool,
    pub preferences: Option<PreferencesDto>,
    pub created_at: String,
    pub updated_at: Option<String>,
}

impl ReaderDto {
    pub fn from(user: &User, reader: &Reader) -> Self {
        ReaderDto {
            id: reader.base().id().to_string(),
            username: user.username().to_string(),
            name: user.name().map(|name| name.to_string()),
            lastname: user.name().map(|lastname| lastname.to_string()),
            subscribed: reader.is_subscribed(),
            preferences: None,
            created_at: user.base().created_at().to_rfc3339(),
            updated_at: user.base().updated_at().map(|d| d.to_rfc3339()),
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
pub struct ReaderPublicationInteractionDto {
    pub viewed: bool,
    pub read: bool,
    pub liked: bool,
    pub reviewed: bool,
}

impl ReaderPublicationInteractionDto {
    pub fn new(viewed: bool, read: bool, liked: bool, reviewed: bool) -> Self {
        ReaderPublicationInteractionDto {
            viewed,
            read,
            liked,
            reviewed,
        }
    }
}

#[derive(Serialize)]
pub struct ReaderAuthorInteractionDto {
    pub followed: bool,
}

impl ReaderAuthorInteractionDto {
    pub fn new(followed: bool) -> Self {
        ReaderAuthorInteractionDto { followed }
    }
}
