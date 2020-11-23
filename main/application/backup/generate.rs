use tokio::process::Command;

use common::error::Error;
use common::request::CommandResponse;
use common::result::Result;
use identity::UserIdAndRole;

pub struct Generate;

impl Generate {
    pub fn new() -> Self {
        Generate
    }

    pub async fn exec(&self, (_auth_id, auth_role): UserIdAndRole) -> Result<CommandResponse> {
        if !auth_role.can("generate_backup") {
            return Err(Error::unauthorized());
        }

        Command::new("docker-compose")
            .arg("exec")
            .arg("postgres")
            .arg("scripts/scripts/backup.sh")
            .spawn()
            .map_err(|err| Error::new("backup", "execute_command").wrap_raw(err))?
            .await
            .map_err(|err| Error::new("backup", "execute_command").wrap_raw(err))?;

        Ok(CommandResponse::default())
    }
}
