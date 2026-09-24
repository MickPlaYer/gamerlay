use iced::window::Icon;

pub fn load() -> Icon {
    iced::window::icon::from_file(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/icon.png"))
        .unwrap()
}
