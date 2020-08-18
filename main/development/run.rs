use common::event::EventSubscriber;
use common::model::AggregateRoot;
use common::result::Result;
use identity::domain::role::*;
use identity::domain::user::*;
use publishing::domain::category::{Name as CategoryName, *};

use crate::container::Container;
use crate::development::EventLogger;

pub async fn run(c: &Container) -> Result<()> {
    populate(c).await?;

    let event_bus = c.event_bus();
    event_bus
        .subscribe(Box::new(EventLogger::new(c.event_repo())))
        .await?;

    Ok(())
}

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
        Some(Person::new(Fullname::new("Admin", "Superpowers")?)?),
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
        Some(Person::new(Fullname::new("Content", "Manager")?)?),
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
        Some(Person::new(Fullname::new("TheFirst", "User")?)?),
        content_manager_role.clone(),
        None,
    );
    c.identity.user_repo().save(&mut admin).await?;
    c.identity.user_repo().save(&mut content_manager).await?;
    c.identity.user_repo().save(&mut user).await?;

    // Publishing
    let mut category_1 = Category::new(CategoryId::new("category-1")?, Name::new("Category 01")?)?;
    let mut category_2 = Category::new(
        CategoryId::new("category-2")?,
        CategoryName::new("Category 02")?,
    )?;
    c.publishing.category_repo().save(&mut category_1).await?;
    c.publishing.category_repo().save(&mut category_2).await?;

    Ok(())
}
