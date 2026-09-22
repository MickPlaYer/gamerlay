mod counter;

use counter::Counter;
use iced::{
    Color,
    theme::Style,
    window::{Level, Settings, settings::PlatformSpecific},
};

fn main() -> iced::Result {
    iced::application(Counter::new, Counter::update, Counter::view)
        .subscription(Counter::subscription)
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
            ..Default::default()
        })
        .style(|_state, _theme| Style {
            background_color: Color::TRANSPARENT,
            text_color: Color::WHITE,
        })
        .run()
}
