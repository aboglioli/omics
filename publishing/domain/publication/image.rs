use common::error::Error;
use common::result::Result;

use crate::domain::publication::Frame;

#[derive(Debug, Clone)]
pub struct Image {
    url: String,
    frames: Vec<Frame>,
}

impl Image {
    pub fn new<S: Into<String>>(url: S) -> Result<Self> {
        let url = url.into();

        if !url.ends_with(".jpg") && !url.ends_with(".jpeg") && !url.ends_with(".png") {
            return Err(Error::new("image", "wrong_extension"));
        }

        Ok(Image {
            url: url.into(),
            frames: Vec::new(),
        })
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn frames(&self) -> &[Frame] {
        &self.frames
    }

    pub fn set_frames(&mut self, frames: Vec<Frame>) -> Result<()> {
        self.frames = frames;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::publication::{Position, Size};

    #[test]
    fn image() {
        // New image
        let mut image = Image::new("host.com/image.jpg").unwrap();
        let frames = vec![
            Frame::new(
                0,
                Position::new(0, 0).unwrap(),
                Size::new(800, 600).unwrap(),
            )
            .unwrap(),
            Frame::new(
                1,
                Position::new(800, 0).unwrap(),
                Size::new(800, 600).unwrap(),
            )
            .unwrap(),
            Frame::new(
                2,
                Position::new(1600, 0).unwrap(),
                Size::new(800, 600).unwrap(),
            )
            .unwrap(),
        ];
        image.set_frames(frames).unwrap();

        assert_eq!(image.frames().len(), 3);
        assert_eq!(image.frames()[0].order(), 0);
        assert_eq!(image.frames()[1].order(), 1);
        assert_eq!(image.frames()[2].order(), 2);
    }
}
