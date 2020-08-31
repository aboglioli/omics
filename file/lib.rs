pub mod file;
pub mod uploader;

use std::fs::File;
use std::io::Write;

use actix_multipart::Multipart;
use actix_web::web;
use bytes::Bytes;
use futures::StreamExt;

use common::error::Error;
use common::result::Result;

use crate::file::TempFile;

pub async fn extract_payload(payload: &mut Multipart) -> Result<(Bytes, Vec<TempFile>)> {
    let mut data = Bytes::new();
    let mut files: Vec<TempFile> = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field =
            item.map_err(|err| Error::new("file", "invalid").set_message(err.to_string()))?;

        let content_disposition = field
            .content_disposition()
            .ok_or_else(|| Error::new("file", "invalid"))?;
        let name = content_disposition
            .get_name()
            .ok_or_else(|| Error::new("file", "invalud"))?;

        if name == "data" {
            while let Some(chunk) = field.next().await {
                data = chunk.map_err(|_| Error::new("chunk", "read"))?;
            }
        } else {
            if let Some(filename) = content_disposition.get_filename() {
                let file = TempFile::new(sanitize_filename::sanitize(&filename));
                let path = file.path().to_owned();
                let mut f = web::block(move || File::create(path))
                    .await
                    .map_err(|err| Error::new("file", "create").wrap_raw(err))?;

                while let Some(chunk) = field.next().await {
                    let data = chunk.map_err(|_| Error::new("chunk", "read"))?;
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .map_err(|err| Error::new("file", "write").wrap_raw(err))?;
                }

                files.push(file.clone());
            }
        }
    }

    Ok((data, files))
}
