use serde::Serialize;
use tokio::fs;
use tokio::stream::StreamExt;

use common::error::Error;
use common::result::Result;
use identity::UserIdAndRole;

#[derive(Serialize)]
pub struct BackupFile {
    file: String,
    path: String,
}

pub struct List;

impl List {
    pub fn new() -> Self {
        List
    }

    pub async fn exec(&self, (_auth_id, _auth_role): UserIdAndRole) -> Result<Vec<BackupFile>> {
        if !auth_role.can("generate_backup") {
            return Err(Error::unauthorized());
        }

        let mut dir = fs::read_dir("backups")
            .await
            .map_err(|err| Error::new("backup", "read_files").wrap_raw(err))?;

        let mut files = Vec::new();
        while let Some(f) = dir.next().await {
            let f = f.map_err(|err| Error::new("backup", "read_file").wrap_raw(err))?;
            files.push(BackupFile {
                file: f.file_name().to_str().unwrap().to_string(),
                path: f.path().to_str().unwrap().to_string(),
            });
        }

        Ok(files)
    }
}
