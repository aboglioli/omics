
use common::result::Result;

#[derive(Debug, Clone)]
pub struct Image {
    url: String,
}

impl Image {
    pub fn new<S: Into<String>>(url: S) -> Result<Self> {
        let url = url.into();

        // if !url.ends_with(".jpg") && !url.ends_with(".jpeg") && !url.ends_with(".png") {
        //     return Err(Error::new("image", "wrong_extension"));
        // }

        Ok(Image { url })
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

impl ToString for Image {
    fn to_string(&self) -> String {
        self.url().to_owned()
    }
}
