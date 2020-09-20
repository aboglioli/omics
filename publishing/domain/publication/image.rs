use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub struct Image {
    url: String,
}

impl Image {
    pub fn new<S: Into<String>>(url: S) -> Result<Self> {
        let url = url.into();

        if !url.ends_with(".jpg") && !url.ends_with(".jpeg") && !url.ends_with(".png") {
            return Err(Error::new("image", "wrong_extension"));
        }

        Ok(Image { url: url.into() })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image() {
        // New image
        assert!(Image::new("host.com/image.jpg").is_ok());
        assert!(Image::new("host.com/image.ext").is_err());
        assert!(Image::new("host.com/image").is_err());
    }
}
