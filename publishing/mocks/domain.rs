use identity::domain::user::UserId;

use crate::domain::author::{Author, AuthorId};
use crate::domain::category::{Category, CategoryId, Name as CategoryName};
use crate::domain::collection::{Collection, CollectionId};
use crate::domain::interaction::Comment;
use crate::domain::publication::{
    Header, Image, Name, Page, Publication, PublicationId, Synopsis, Tag,
};
use crate::domain::reader::{Reader, ReaderId};

pub fn publication(
    publication_id: &str,
    author_id: &str,
    name: &str,
    category_id: &str,
    tags: Vec<&str>,
    cover_url: &str,
    pages_count: u32,
    published: bool,
    approved: bool,
    contract: bool,
) -> Publication {
    let mut publication = Publication::new(
        PublicationId::new(publication_id).unwrap(),
        AuthorId::new(author_id).unwrap(),
        Header::new(
            Name::new(name).unwrap(),
            Synopsis::new("Synopsis...").unwrap(),
            CategoryId::new(category_id).unwrap(),
            tags.into_iter().map(|t| Tag::new(t).unwrap()).collect(),
            Image::new(cover_url).unwrap(),
        )
        .unwrap(),
    )
    .unwrap();

    let mut pages = Vec::new();
    for i in 0..pages_count {
        let mut page = Page::new(i).unwrap();
        page.set_images(vec![Image::new("domain.com/image1.jpg").unwrap()])
            .unwrap();
        pages.push(page);
    }
    publication.set_pages(pages).unwrap();

    if published {
        publication.publish().unwrap();

        if approved {
            publication
                .approve(
                    UserId::new("content-manager-1").unwrap(),
                    Comment::new("Comment...").unwrap(),
                )
                .unwrap();

            if contract {
                publication.add_contract().unwrap();
            }
        }
    }

    publication
}

pub fn collection(
    collection_id: &str,
    author_id: &str,
    name: &str,
    category_id: &str,
    tags: Vec<&str>,
    cover_url: &str,
) -> Collection {
    let collection = Collection::new(
        CollectionId::new(collection_id).unwrap(),
        AuthorId::new(author_id).unwrap(),
        Header::new(
            Name::new(name).unwrap(),
            Synopsis::new("Synopsis...").unwrap(),
            CategoryId::new(category_id).unwrap(),
            tags.into_iter().map(|t| Tag::new(t).unwrap()).collect(),
            Image::new(cover_url).unwrap(),
        )
        .unwrap(),
    )
    .unwrap();

    collection
}

pub fn category(_id: &str, name: &str) -> Category {
    Category::new(CategoryName::new(name).unwrap()).unwrap()
}

pub fn author(id: &str, username: &str) -> Author {
    Author::new(AuthorId::new(id).unwrap(), username).unwrap()
}

pub fn reader(id: &str, username: &str) -> Reader {
    Reader::new(ReaderId::new(id).unwrap(), username).unwrap()
}
