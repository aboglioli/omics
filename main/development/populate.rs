use common::result::Result;

use identity::application::user::{Login, LoginCommand, Register, RegisterCommand, Validate};

use crate::container::Container;

pub async fn populate(c: &Container) -> Result<()> {
    let uc = Register::new(
        c.identity.event_pub(),
        c.identity.user_repo(),
        c.identity.user_serv(),
    );
    let res = uc
        .exec(RegisterCommand {
            username: "user".to_owned(),
            email: "user@domain.com".to_owned(),
            password: "P@asswd!".to_owned(),
        })
        .await?;

    let uc = Validate::new(c.identity.event_pub(), c.identity.user_repo());
    uc.exec(res.id, res.validation_code).await?;

    let uc = Login::new(c.identity.event_pub(), c.identity.authentication_serv());
    let _res = uc
        .exec(LoginCommand {
            username_or_email: "user".to_owned(),
            password: "P@asswd!".to_owned(),
        })
        .await?;

    Ok(())
}
