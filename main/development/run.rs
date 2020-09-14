use common::model::AggregateRoot;
use common::result::Result;
use identity::domain::role::*;
use identity::domain::user::*;
use publishing::domain::author::Author;
use publishing::domain::category::*;
use publishing::domain::collection::*;

use publishing::domain::publication::*;
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
    let mut admin = c
        .publishing
        .user_serv()
        .get_by_id(admin.base().id())
        .await?;
    c.publishing.user_repo().save(&mut admin).await?;
    c.publishing
        .author_repo()
        .save(&mut Author::new(admin.base().id().clone())?)
        .await?;
    c.publishing
        .reader_repo()
        .save(&mut Reader::new(admin.base().id().clone())?)
        .await?;

    Ok(())
}
