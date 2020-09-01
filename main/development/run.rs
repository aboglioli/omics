use common::model::AggregateRoot;
use common::result::Result;
use identity::domain::role::*;
use identity::domain::user::{Image as UserImage, * };
use publishing::domain::category::{Name as CategoryName, *};
use publishing::domain::publication::{Name as PublicationName, Image as PublicationImage, *};
use publishing::domain::collection::*;

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
        )?),
        admin_role.clone(),
        None,
    );
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
        )?),
        content_manager_role.clone(),
        None,
    );
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
            None,
            None,
            None,
        )?),
        user_role.clone(),
        None,
    );
    c.identity.user_repo().save(&mut admin).await?;
    c.identity.user_repo().save(&mut content_manager).await?;
    c.identity.user_repo().save(&mut user).await?;

    // Publishing
    let mut category_1 = Category::new(CategoryId::new("category-1")?, CategoryName::new("Category 01")?)?;
    let mut category_2 = Category::new(
        CategoryId::new("category-2")?,
        CategoryName::new("Category 02")?,
    )?;
    c.publishing.category_repo().save(&mut category_1).await?;
    c.publishing.category_repo().save(&mut category_2).await?;

    let mut publication = Publication::new(
        PublicationId::new("publication-1")?,
        user.base().id().clone(),
        Header::new(
            PublicationName::new("Spiderman vs Superman")?,
            Synopsis::new("Buena historia")?,
            category_2.base().id().clone(),
            vec![Tag::new("Spiderman")?, Tag::new("Superman")?, Tag::new("Lucha")?],
            PublicationImage::new("https://via.placeholder.com/768x1024")?,
        )?
    )?;
    c.publishing.publication_repo().save(&mut publication).await?;


    Ok(())
}
