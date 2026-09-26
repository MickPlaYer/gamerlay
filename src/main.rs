#![windows_subsystem = "windows"]

mod icon;
mod overlay;

use iced::{
    Color,
    theme::Style,
    window::{Level, Settings, settings::PlatformSpecific},
};
use overlay::Overlay;

fn main() -> iced::Result {
    iced::application(Overlay::new, Overlay::update, Overlay::view)
        .subscription(Overlay::subscription)
        .title("_Gamerlay")
        .decorations(false)
        .window(Settings {
            level: Level::AlwaysOnTop,
            fullscreen: true,
            transparent: true,
            platform_specific: PlatformSpecific {
                skip_taskbar: true,
                ..Default::default()
            },
            icon: Some(icon::load()),
            ..Default::default()
        })
        .style(|_state, _theme| Style {
            background_color: Color::TRANSPARENT,
            text_color: Color::WHITE,
        })
        .font(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/CascadiaMono-Bold.ttf"
        )))
        .run()
}
