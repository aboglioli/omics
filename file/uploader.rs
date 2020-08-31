mod s3_uploader;
pub use s3_uploader::*;

use async_trait::async_trait;

use common::result::Result;

use crate::file::{TempFile, UploadedFile};

#[async_trait]
pub trait FileUploader {
    async fn upload(&self, file: TempFile) -> Result<UploadedFile>;
}
