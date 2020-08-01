use serde::Deserialize;

use common::error::Error;

#[derive(Deserialize)]
pub struct PublishCommand {
    pub name: String,
    pub synopsis: String,
}

impl PublishCommand {
    pub fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

pub struct Publish {}

impl Publish {
    pub async fn exec(&self, cmd: PublishCommand) -> Result<(), Error> {
        cmd.validate()?;
        Ok(())
    }
}
