use common::model::AggregateRoot;
use common::result::Result;
use identity::domain::role::*;
use identity::domain::user::{Image as UserImage, *};
use publishing::domain::author::Author;
use publishing::domain::category::{Name as CategoryName, *};
use publishing::domain::collection::*;
use publishing::domain::publication::{Image as PublicationImage, Name as PublicationName, *};
use publishing::domain::reader::Reader;

use crate::container::Container;

pub async fn populate(c: &Container) -> Result<()> {
    // Identity
    let mut admin_role = Role::new(RoleId::new("admin")?, "Administrator")?;
    let mut content_manager_role = Role::new(RoleId::new("content-manager")?, "Content Manager")?;
    let mut user_role = Role::new(RoleId::new("user")?, "User")?;
    c.identity.role_repo().save(&mut admin_role).await?;
    c.identity
        .role_repo()
        .save(&mut content_manager_role)
        .await?;
    c.identity.role_repo().save(&mut user_role).await?;

    let hashed_password = c.identity.password_hasher().hash("P@asswd!")?;

    let mut admin = User::build(
        AggregateRoot::new(UserId::new("admin-1")?),
        Identity::new(
            Provider::Local,
            Username::new("admin")?,
            Email::new("admin@omics.com")?,
            Some(Password::new(hashed_password.clone())?),
        )?,
        Some(Person::new(
            Fullname::new("Admin", "Superpowers")?,
            None,
            None,
            None,
            None,
        )?),
        admin_role.base().id().clone(),
        None,
    );
    c.identity.user_repo().save(&mut admin).await?;
    c.publishing
        .author_repo()
        .save(&mut Author::new(admin.base().id().clone())?)
        .await?;
    c.publishing
        .reader_repo()
        .save(&mut Reader::new(admin.base().id().clone())?)
        .await?;

    let mut content_manager = User::build(
        AggregateRoot::new(UserId::new("content_manager-1")?),
        Identity::new(
            Provider::Local,
            Username::new("content-manager")?,
            Email::new("content-manager@omics.com")?,
            Some(Password::new(hashed_password.clone())?),
        )?,
        Some(Person::new(
            Fullname::new("Content", "Manager")?,
            None,
            None,
            None,
            None,
        )?),
        content_manager_role.base().id().clone(),
        None,
    );
    c.identity.user_repo().save(&mut content_manager).await?;
    c.publishing
        .author_repo()
        .save(&mut Author::new(content_manager.base().id().clone())?)
        .await?;
    c.publishing
        .reader_repo()
        .save(&mut Reader::new(content_manager.base().id().clone())?)
        .await?;

    let mut user = User::build(
        AggregateRoot::new(UserId::new("user-1")?),
        Identity::new(
            Provider::Local,
            Username::new("user")?,
            Email::new("user@omics.com")?,
            Some(Password::new(hashed_password.clone())?),
        )?,
        Some(Person::new(
            Fullname::new("TheFirst", "User")?,
            Some(Birthdate::from_str("1994-08-05T15:39:57+00:00")?),
            Some(Gender::Male),
            Some(Biography::new("My amazing biography...")?),
            Some(UserImage::new("https://via.placeholder.com/128x128.jpg")?),
        )?),
        user_role.base().id().clone(),
        None,
    );
    c.identity.user_repo().save(&mut user).await?;
    c.publishing
        .author_repo()
        .save(&mut Author::new(user.base().id().clone())?)
        .await?;
    c.publishing
        .reader_repo()
        .save(&mut Reader::new(user.base().id().clone())?)
        .await?;

    // Publishing
    let mut category_1 = Category::new(
        CategoryId::new("category-1")?,
        CategoryName::new("Category 01")?,
    )?;
    let mut category_2 = Category::new(
        CategoryId::new("category-2")?,
        CategoryName::new("Category 02")?,
    )?;
    c.publishing.category_repo().save(&mut category_1).await?;
    c.publishing.category_repo().save(&mut category_2).await?;

    let mut publication_1 = Publication::new(
        PublicationId::new("publication-1")?,
        user.base().id().clone(),
        Header::new(
            PublicationName::new("Spiderman vs Superman")?,
            Synopsis::new("Buena historia")?,
            category_2.base().id().clone(),
            vec![
                Tag::new("Spiderman")?,
                Tag::new("Superman")?,
                Tag::new("Lucha")?,
            ],
            PublicationImage::new("https://via.placeholder.com/768x1024.jpg")?,
        )?,
    )?;
    let mut page_1 = Page::new(0)?;
    page_1.set_images(vec![
        PublicationImage::new("https://via.placeholder.com/768x1024.jpg")?,
        PublicationImage::new("https://via.placeholder.com/768x1024.jpg")?,
        PublicationImage::new("https://via.placeholder.com/768x1024.jpg")?,
    ])?;
    let mut page_2 = Page::new(1)?;
    page_2.set_images(vec![PublicationImage::new(
        "https://via.placeholder.com/768x1024.jpg",
    )?])?;
    publication_1.set_pages(vec![page_1, page_2])?;
    publication_1.publish()?;
    publication_1.approve(content_manager.base().id().clone())?;
    c.publishing
        .publication_repo()
        .save(&mut publication_1)
        .await?;

    let mut publication_2 = Publication::new(
        PublicationId::new("publication-2")?,
        user.base().id().clone(),
        Header::new(
            PublicationName::new("Final Project")?,
            Synopsis::new("Excelente obra...")?,
            category_1.base().id().clone(),
            vec![Tag::new("Project")?, Tag::new("Final")?],
            PublicationImage::new("https://via.placeholder.com/768x1024.jpg")?,
        )?,
    )?;
    c.publishing
        .publication_repo()
        .save(&mut publication_2)
        .await?;

    let mut publication_3 = Publication::new(
        PublicationId::new("publication-3")?,
        user.base().id().clone(),
        Header::new(
            PublicationName::new("Borrador")?,
            Synopsis::new("Borrador...")?,
            category_1.base().id().clone(),
            vec![],
            PublicationImage::new("https://via.placeholder.com/768x1024.jpg")?,
        )?,
    )?;
    c.publishing
        .publication_repo()
        .save(&mut publication_3)
        .await?;

    let mut collection_1 = Collection::new(
        CollectionId::new("collection-1")?,
        user.base().id().clone(),
        Header::new(
            PublicationName::new("Colección 1")?,
            Synopsis::new("Primera colección.")?,
            category_1.base().id().clone(),
            vec![Tag::new("Saga")?, Tag::new("Proyecto")?],
            PublicationImage::new("https://via.placeholder.com/768x1024.jpg")?,
        )?,
    )?;
    collection_1.add_item(&publication_1)?;
    c.publishing
        .collection_repo()
        .save(&mut collection_1)
        .await?;

    let mut collection_2 = Collection::new(
        CollectionId::new("collection-2")?,
        user.base().id().clone(),
        Header::new(
            PublicationName::new("Colección 2")?,
            Synopsis::new("Segunda colección.")?,
            category_1.base().id().clone(),
            vec![Tag::new("Saga")?, Tag::new("Proyecto")?],
            PublicationImage::new("https://via.placeholder.com/768x1024.jpg")?,
        )?,
    )?;
    collection_2.add_item(&publication_1)?;
    c.publishing
        .collection_repo()
        .save(&mut collection_2)
        .await?;

    Ok(())
}
