use iced::{
    Element,
    widget::{self, image::Handle},
};

pub struct Dot {
    image: Handle,
}

impl Dot {
    const PNG: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/dot.png"));

    pub fn new() -> Self {
        Self {
            image: Handle::from_bytes(Self::PNG),
        }
    }

    pub fn view<Message>(&self) -> Element<'_, Message> {
        widget::image(&self.image).into()
    }
}
