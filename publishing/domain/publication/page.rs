pub struct Page {
    number: u32,
    images: Vec<Image>,
}

pub type ImageID = String;

pub struct Image {
    id: ImageID,
    url: String,
    frames: Vec<Frame>,
    size: u32,
}

pub struct Frame {
    order: u32,
    position: Position,
    size: Size,
}

pub struct Position(u32, u32);

pub struct Size(u32, u32);
