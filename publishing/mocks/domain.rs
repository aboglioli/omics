use crate::domain::author::AuthorId;
use crate::domain::category::CategoryId;
use crate::domain::content_manager::{ContentManager, ContentManagerId};
use crate::domain::publication::{Header, Image, Name, Publication, PublicationId, Synopsis, Tag};
use crate::domain::reader::{Reader, ReaderId};

pub fn publication1() -> Publication {
    Publication::new(
        PublicationId::new("#pub01").unwrap(),
        AuthorId::new("#author01").unwrap(),
        Header::new(
            Name::new("Pub 01").unwrap(),
            Synopsis::new("Synopsis...").unwrap(),
            CategoryId::new("cat_01").unwrap(),
            vec![Tag::new("Tag 1").unwrap(), Tag::new("Tag 2").unwrap()],
            Image::new("domain.com/image.jpg").unwrap(),
        )
        .unwrap(),
    )
    .unwrap()
}

pub fn published_publication1() -> Publication {
    let mut publication = publication1();
    publication.publish().unwrap();
    publication.approve(content_manager1()).unwrap();
    publication
}

pub fn content_manager1() -> ContentManager {
    ContentManager::new(ContentManagerId::new("#content-manager01").unwrap()).unwrap()
}

pub fn reader1() -> Reader {
    Reader::new(ReaderId::new("#reader01").unwrap(), "Reader 01").unwrap()
}
