use identity::domain::user::User;
use identity::mocks;

use crate::domain::author::{Author, AuthorId};
use crate::domain::category::{Category, CategoryId, Name as CategoryName};
use crate::domain::collection::{Collection, CollectionId};
use crate::domain::interaction::Comment;
use crate::domain::publication::{
    Header, Image, Name, Page, Publication, PublicationId, Synopsis, Tag,
};
use crate::domain::reader::{Reader, ReaderId};

#[allow(dead_code)]
pub fn publication1() -> Publication {
    let author_id = user1().0.base().id().clone();

    let mut publication = Publication::new(
        PublicationId::new("#publication01").unwrap(),
        author_id,
        Header::new(
            Name::new("Publication 01").unwrap(),
            Synopsis::new("Synopsis...").unwrap(),
            category1().base().id().clone(),
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

#[allow(dead_code)]
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

    let _author_id = publication.author_id().clone();
    publication.set_pages(vec![page1, page2]).unwrap();

    publication.publish().unwrap();
    publication
        .approve(
            content_manager1().0.base().id().clone(),
            Comment::new("comment").unwrap(),
        )
        .unwrap();
    publication
}

#[allow(dead_code)]
pub fn empty_collection1() -> Collection {
    Collection::new(
        CollectionId::new("#collection01").unwrap(),
        user1().0.base().id().clone(),
        Header::new(
            Name::new("Collection 01").unwrap(),
            Synopsis::new("Synopsis...").unwrap(),
            category1().base().id().clone(),
            vec![Tag::new("Tag 1").unwrap(), Tag::new("Tag 2").unwrap()],
            Image::new("domain.com/image.jpg").unwrap(),
        )
        .unwrap(),
    )
    .unwrap()
}

#[allow(dead_code)]
pub fn content_manager1() -> (User, Author, Reader) {
    (
        mocks::admin1(),
        Author::new(
            AuthorId::new("content-manager-1").unwrap(),
            "content-manager-1",
        )
        .unwrap(),
        Reader::new(ReaderId::new("content-manager-1").unwrap()).unwrap(),
    )
}

#[allow(dead_code)]
pub fn user1() -> (User, Author, Reader) {
    (
        mocks::user1(),
        Author::new(AuthorId::new("user-1").unwrap(), "user-1").unwrap(),
        Reader::new(ReaderId::new("user-1").unwrap()).unwrap(),
    )
}

#[allow(dead_code)]
pub fn user2() -> (User, Author, Reader) {
    (
        mocks::user2(),
        Author::new(AuthorId::new("user-2").unwrap(), "user-2").unwrap(),
        Reader::new(ReaderId::new("user-2").unwrap()).unwrap(),
    )
}

#[allow(dead_code)]
pub fn category1() -> Category {
    Category::new(
        CategoryId::new("#category01").unwrap(),
        CategoryName::new("Category 01").unwrap(),
    )
    .unwrap()
}

#[allow(dead_code)]
pub fn category2() -> Category {
    Category::new(
        CategoryId::new("#category02").unwrap(),
        CategoryName::new("Category 02").unwrap(),
    )
    .unwrap()
}
