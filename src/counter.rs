use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
    hotkey::{Code, HotKey, Modifiers},
};
use iced::{
    Element, Subscription, Task,
    futures::{SinkExt, Stream},
    stream, widget, window,
};
use std::collections::HashMap;

pub struct Counter {
    pub count: i32,
    _manager: GlobalHotKeyManager,
    hotkeys: HashMap<u32, KeyAction>,
}

#[derive(Debug, Clone)]
pub enum Message {
    IncrementCount,
    DecrementCount,
    HotKey(u32),
}

enum KeyAction {
    Exit,
    Add,
}

// Implement our Counter
impl Counter {
    pub fn new() -> (Self, Task<Message>) {
        let manager = GlobalHotKeyManager::new().unwrap();
        let mut hotkeys = HashMap::new();
        let hotkey = HotKey::new(Some(Modifiers::SHIFT), Code::F10);
        manager.register(hotkey).unwrap();
        hotkeys.insert(hotkey.id, KeyAction::Exit);
        let hotkey = HotKey::new(Some(Modifiers::SHIFT), Code::F11);
        manager.register(hotkey).unwrap();
        hotkeys.insert(hotkey.id, KeyAction::Add);
        (
            Self {
                count: 0,
                _manager: manager,
                hotkeys,
            },
            window::oldest().and_then(window::enable_mouse_passthrough),
        )
    }

    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        println!("!!!");
        match message {
            Message::IncrementCount => self.count += 1,
            Message::DecrementCount => self.count -= 1,
            Message::HotKey(id) => {
                if let Some(key) = self.hotkeys.get(&id) {
                    match key {
                        KeyAction::Exit => return window::oldest().and_then(window::close),
                        KeyAction::Add => self.count += 1,
                    }
                }
            }
        }
        iced::Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let row = widget::row![
            widget::button("-").on_press(Message::DecrementCount),
            widget::text!("Count: {}", self.count),
            widget::button("+").on_press(Message::IncrementCount)
        ]
        .spacing(10);
        widget::container(row).center(iced::Length::Fill).into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::run(global_hotkey_worker)
    }
}

pub fn global_hotkey_worker() -> impl Stream<Item = Message> {
    stream::channel(0, async |mut output| {
        let receiver = GlobalHotKeyEvent::receiver();
        loop {
            if let Ok(event) = receiver.try_recv()
                && event.state == HotKeyState::Pressed
            {
                output.send(Message::HotKey(event.id)).await.unwrap();
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    })
}
