use std::fs::File;
use std::io::Read;

use async_trait::async_trait;
use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3Client, S3};

use common::config::Config;
use common::error::Error;
use common::result::Result;

use crate::file::{TempFile, UploadedFile};
use crate::uploader::FileUploader;

pub struct S3FileUploader {
    region: Region,
    s3: S3Client,
    bucket: String,
}

impl S3FileUploader {
    pub fn new() -> Self {
        let config = Config::get();
        let region = Region::default();

        S3FileUploader {
            region: region.clone(),
            s3: S3Client::new(region),
            bucket: config.aws_s3_bucket().to_owned(),
        }
    }

    fn url(&self, filename: &str) -> String {
        format!(
            "https://{}.s3.{}.amazonaws.com/{}/{}",
            self.bucket,
            self.region.name(),
            "images",
            filename
        )
    }
}

impl Default for S3FileUploader {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl FileUploader for S3FileUploader {
    async fn upload(&self, temp_file: TempFile) -> Result<UploadedFile> {
        let mut file =
            File::open(temp_file.path()).map_err(|err| Error::new("file", "open").wrap_raw(err))?;

        let mut content: Vec<u8> = Vec::new();
        let _ = file.read_to_end(&mut content);

        let key = format!("images/{}", temp_file.name());

        let put_request = PutObjectRequest {
            bucket: self.bucket.to_owned(),
            key: key.to_owned(),
            body: Some(content.into()),
            ..Default::default()
        };

        let _res = self
            .s3
            .put_object(put_request)
            .await
            .map_err(|err| Error::new("s3", "put_request").wrap_raw(err))?;

        Ok(UploadedFile::new(self.url(temp_file.name())))
    }
}
