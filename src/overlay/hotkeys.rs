use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
    hotkey::{Code, HotKey, Modifiers},
};
use iced::{
    Subscription,
    time::{milliseconds, repeat},
};
use std::collections::HashMap;

pub enum KeyAction {
    Exit,
    Add,
    Toggle,
}

pub trait HotKeyMessage {
    fn message(id: u32) -> Self;
    fn none() -> Self;
}

macro_rules! hotkey {
    ($(($x:expr, $y:expr)),*) => {
        {
            let manager = GlobalHotKeyManager::new().unwrap();
            let mut hotkeys = HashMap::new();
            $(
                manager.register($x).unwrap();
                hotkeys.insert($x.id, $y);
            )*
            (manager, hotkeys)
        }
    };
}

pub fn build() -> (GlobalHotKeyManager, HashMap<u32, KeyAction>) {
    hotkey![
        (
            HotKey::new(Some(Modifiers::SHIFT), Code::F9),
            KeyAction::Exit
        ),
        (
            HotKey::new(Some(Modifiers::SHIFT), Code::F10),
            KeyAction::Toggle
        ),
        (
            HotKey::new(Some(Modifiers::SHIFT), Code::F11),
            KeyAction::Add
        )
    ]
}

pub fn subscription<T: HotKeyMessage + Send + 'static>() -> Subscription<T> {
    repeat(
        || {
            let receiver = GlobalHotKeyEvent::receiver();
            async {
                if let Ok(event) = receiver.try_recv()
                    && event.state == HotKeyState::Pressed
                {
                    T::message(event.id)
                } else {
                    T::none()
                }
            }
        },
        milliseconds(50),
    )
}
