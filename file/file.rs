use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, Clone)]
pub struct TempFile {
    original_name: String,
    name: String,
    path: String,
}

impl TempFile {
    pub fn new<S: Into<String>>(name: S) -> Self {
        let original_name = name.into();
        let name = Uuid::new_v4().to_string();
        let name = format!("{}.jpg", name);

        TempFile {
            original_name,
            name: name.clone(),
            path: format!("./tmp/{}", name),
        }
    }

    pub fn original_name(&self) -> &str {
        &self.original_name
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct UploadedFile {
    url: String,
}

impl UploadedFile {
    pub fn new<S: Into<String>>(url: S) -> Self {
        UploadedFile { url: url.into() }
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}
