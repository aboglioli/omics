use crate::domain::author::{Author, AuthorId};
use crate::domain::category::{Category, CategoryId, Name as CategoryName};
use crate::domain::collection::{Collection, CollectionId};
use crate::domain::content_manager::{ContentManager, ContentManagerId};
use crate::domain::publication::{
    Header, Image, Name, Page, Publication, PublicationId, Synopsis, Tag,
};
use crate::domain::reader::{Reader, ReaderId};

pub fn publication1() -> Publication {
    let mut publication = Publication::new(
        PublicationId::new("#publication01").unwrap(),
        author1().base().id(),
        Header::new(
            Name::new("Publication 01").unwrap(),
            Synopsis::new("Synopsis...").unwrap(),
            category1().base().id(),
            vec![Tag::new("Tag 1").unwrap(), Tag::new("Tag 2").unwrap()],
            Image::new("domain.com/image.jpg").unwrap(),
        )
        .unwrap(),
    )
    .unwrap();

    let mut page_1 = Page::new(0).unwrap();
    page_1
        .set_images(vec![
            Image::new("domain.com/p1_image1.jpg").unwrap(),
            Image::new("domain.com/p1_image2.jpg").unwrap(),
        ])
        .unwrap();
    let mut page_2 = Page::new(1).unwrap();
    page_2
        .set_images(vec![
            Image::new("domain.com/p2_image1.jpg").unwrap(),
            Image::new("domain.com/p2_image2.jpg").unwrap(),
        ])
        .unwrap();
    publication.set_pages(vec![page_1, page_2]).unwrap();

    publication
}

pub fn published_publication1() -> Publication {
    let mut publication = publication1();

    let mut page1 = Page::new(2).unwrap();
    page1
        .set_images(vec![
            Image::new("domain.com/img1.jpg").unwrap(),
            Image::new("domain.com/img2.jpg").unwrap(),
            Image::new("domain.com/img3.jpg").unwrap(),
        ])
        .unwrap();

    let mut page2 = Page::new(3).unwrap();
    page2
        .set_images(vec![
            Image::new("domain.com/img4.jpg").unwrap(),
            Image::new("domain.com/img5.jpg").unwrap(),
        ])
        .unwrap();

    publication.set_pages(vec![page1, page2]).unwrap();

    publication.publish().unwrap();
    publication.approve(&content_manager1()).unwrap();
    publication
}

pub fn empty_collection1() -> Collection {
    Collection::new(
        CollectionId::new("#collection01").unwrap(),
        author1().base().id(),
        Header::new(
            Name::new("Collection 01").unwrap(),
            Synopsis::new("Synopsis...").unwrap(),
            category1().base().id(),
            vec![Tag::new("Tag 1").unwrap(), Tag::new("Tag 2").unwrap()],
            Image::new("domain.com/image.jpg").unwrap(),
        )
        .unwrap(),
    )
    .unwrap()
}

pub fn content_manager1() -> ContentManager {
    ContentManager::new(ContentManagerId::new("#content-manager01").unwrap()).unwrap()
}

pub fn reader1() -> Reader {
    Reader::new(
        ReaderId::new("#reader01").unwrap(),
        "reader-01",
        "Name 01",
        "Lastname 01",
    )
    .unwrap()
}

pub fn author1() -> Author {
    Author::new(
        AuthorId::new("#author01").unwrap(),
        "author-01",
        "Name 01",
        "Lastname 01",
    )
    .unwrap()
}

pub fn author_as_reader1() -> Reader {
    Reader::new(
        ReaderId::new("#author01").unwrap(),
        "author-01",
        "Name 01",
        "Lastname 01",
    )
    .unwrap()
}

pub fn author2() -> Author {
    Author::new(
        AuthorId::new("#author02").unwrap(),
        "author-02",
        "Name 02",
        "Lastname 02",
    )
    .unwrap()
}

pub fn category1() -> Category {
    Category::new(
        CategoryId::new("#category01").unwrap(),
        CategoryName::new("Category 01").unwrap(),
    )
    .unwrap()
}

pub fn category2() -> Category {
    Category::new(
        CategoryId::new("#category02").unwrap(),
        CategoryName::new("Category 02").unwrap(),
    )
    .unwrap()
}
