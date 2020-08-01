use serde::Deserialize;

use common::result::Result;

#[derive(Deserialize)]
pub struct PublishCommand {
    pub name: String,
    pub synopsis: String,
}

impl PublishCommand {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

pub struct Publish {}

impl Publish {
    pub async fn exec(&self, cmd: PublishCommand) -> Result<()> {
        cmd.validate()?;
        Ok(())
    }
}
