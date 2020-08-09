use std::cmp::PartialEq;

use common::result::Result;

#[derive(Debug, Clone)]
pub struct Position(u32, u32);

impl Position {
    pub fn new(x: u32, y: u32) -> Result<Position> {
        Ok(Position(x, y))
    }

    pub fn x(&self) -> u32 {
        self.0
    }

    pub fn y(&self) -> u32 {
        self.1
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

#[derive(Debug, Clone)]
pub struct Size(u32, u32);

impl Size {
    pub fn new(w: u32, h: u32) -> Result<Size> {
        Ok(Size(w, h))
    }

    pub fn width(&self) -> u32 {
        self.0
    }

    pub fn height(&self) -> u32 {
        self.1
    }
}

impl PartialEq for Size {
    fn eq(&self, other: &Size) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

#[derive(Debug, Clone)]
pub struct Frame {
    order: u32,
    position: Position,
    size: Size,
}

impl Frame {
    pub fn new(order: u32, position: Position, size: Size) -> Result<Frame> {
        Ok(Frame {
            order,
            position,
            size,
        })
    }

    pub fn order(&self) -> u32 {
        self.order
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    url: String,
    size: u32,
    frames: Vec<Frame>,
}

impl Image {
    pub fn new(url: &str, size: u32) -> Result<Image> {
        Ok(Image {
            url: url.to_owned(),
            size,
            frames: Vec::new(),
        })
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn frames(&self) -> &[Frame] {
        &self.frames
    }

    pub fn set_frames(&mut self, frames: Vec<Frame>) -> Result<()> {
        self.frames = frames;
        Ok(())
    }
}

pub type PageNumber = u32;

#[derive(Debug, Clone)]
pub struct Page {
    number: PageNumber,
    images: Vec<Image>,
}

impl Page {
    pub fn new(number: u32) -> Result<Page> {
        Ok(Page {
            number,
            images: Vec::new(),
        })
    }

    pub fn number(&self) -> &PageNumber {
        &self.number
    }

    pub fn images(&self) -> &[Image] {
        &self.images
    }

    pub fn set_images(&mut self, images: Vec<Image>) -> Result<()> {
        self.images = images;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image() {
        // New image
        let mut image = Image::new("host.com/image.jpg", 1024).unwrap();
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
