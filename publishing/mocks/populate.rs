use common::event::EventPublisher;
use common::result::Result;
use shared::domain::user;

use crate::container::Container;
use crate::domain::author;
use crate::domain::category;
use crate::domain::collection;
use crate::domain::interaction;
use crate::domain::publication;
use crate::domain::reader;

pub async fn populate<EPub>(c: &Container<EPub>) -> Result<()>
where
    EPub: EventPublisher,
{
    // Categories
    let category1_id = category::CategoryId::new("category-1")?;
    let mut category1 =
        category::Category::new(category1_id.clone(), category::Name::new("Category 1")?)?;
    c.category_repo().save(&mut category1).await?;

    let category2_id = category::CategoryId::new("category-2")?;
    let mut category2 =
        category::Category::new(category2_id.clone(), category::Name::new("Category 2")?)?;
    c.category_repo().save(&mut category2).await?;

    // Users
    let user1_id = c.user_repo().next_id().await?;
    let mut user1 = user::User::new(user1_id.clone(), "user-1", "user")?;
    let mut author1 = author::Author::new(user1_id.clone())?;
    let mut reader1 = reader::Reader::new(user1_id.clone())?;
    c.user_repo().save(&mut user1).await?;
    c.author_repo().save(&mut author1).await?;
    c.reader_repo().save(&mut reader1).await?;

    // Publications
    let publication1_id = c.publication_repo().next_id().await?;
    let mut publication1 = publication::Publication::new(
        publication1_id.clone(),
        user1_id.clone(),
        publication::Header::new(
            publication::Name::new("Publication 1")?,
            publication::Synopsis::new("Synopsis...")?,
            category1_id.clone(),
            vec![publication::Tag::new("Tag 1")?],
            publication::Image::new("http://domain.com/image.jpg")?,
        )?,
    )?;
    c.publication_repo().save(&mut publication1).await?;

    let publication2_id = c.publication_repo().next_id().await?;
    let mut publication2 = publication::Publication::new(
        publication2_id.clone(),
        user1_id.clone(),
        publication::Header::new(
            publication::Name::new("Publication 2")?,
            publication::Synopsis::new("Synopsis...")?,
            category2_id.clone(),
            vec![publication::Tag::new("Tag 1")?],
            publication::Image::new("http://domain.com/image.jpg")?,
        )?,
    )?;
    publication2.set_pages(vec![publication::Page::with_images(
        0,
        vec![
            publication::Image::new("http://domain.com/image.jpg")?,
            publication::Image::new("http://domain.com/image.jpg")?,
        ],
    )?])?;
    publication2.publish()?;
    publication2.approve(
        user::UserId::new("00000000-0000-0000-0000-000000000001")?,
        interaction::Comment::new("Comment...")?,
    )?;
    c.publication_repo().save(&mut publication2).await?;

    // Collections
    let collection1_id = c.collection_repo().next_id().await?;
    let mut collection1 = collection::Collection::new(
        collection1_id.clone(),
        user1_id.clone(),
        publication::Header::new(
            publication::Name::new("Collection 1")?,
            publication::Synopsis::new("Synopsis...")?,
            category1_id.clone(),
            vec![publication::Tag::new("Tag 1")?],
            publication::Image::new("http://domain.com/image.jpg")?,
        )?,
    )?;
    collection1.add_item(&publication1)?;
    collection1.add_item(&publication2)?;
    c.collection_repo().save(&mut collection1).await?;

    Ok(())
}
