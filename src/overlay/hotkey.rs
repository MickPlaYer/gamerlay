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
    Cycle,
    Toggle,
}

pub trait HotKeyMessage {
    fn message(id: u32) -> Self;
    fn sleep() -> Self;
}

macro_rules! hotkey {
    ($(($x:expr, $y:expr)),*) => {
        {
            let manager = GlobalHotKeyManager::new().unwrap();
            let mut hotkeys = HashMap::new();
            $(
                let key = HotKey::new(Some(Modifiers::SHIFT), $x);
                manager.register(key).unwrap();
                hotkeys.insert(key.id, $y);
            )*
            (manager, hotkeys)
        }
    };
}

pub fn build() -> (GlobalHotKeyManager, HashMap<u32, KeyAction>) {
    hotkey![
        (Code::F9, KeyAction::Exit),
        (Code::F10, KeyAction::Toggle),
        (Code::F11, KeyAction::Cycle)
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
                    T::sleep()
                }
            }
        },
        milliseconds(50),
    )
}
