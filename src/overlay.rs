mod clock;
mod hotkeys;

use clock::Clock;
use global_hotkey::GlobalHotKeyManager;
use hotkeys::{HotKeyMessage, KeyAction};
use iced::{Element, Length, Subscription, Task, widget, window};
use std::collections::HashMap;

pub struct Overlay {
    count: i32,
    show: bool,
    _manager: GlobalHotKeyManager,
    hotkeys: HashMap<u32, KeyAction>,
    clock: Clock,
}

pub enum Message {
    HotKey(u32),
    Clock(clock::Message),
    Sleep,
}

impl HotKeyMessage for Message {
    fn message(id: u32) -> Self {
        Message::HotKey(id)
    }

    fn none() -> Self {
        Message::Sleep
    }
}

impl Overlay {
    pub fn new() -> (Self, Task<Message>) {
        let (manager, hotkeys) = hotkeys::build();
        (
            Self {
                count: 0,
                show: true,
                _manager: manager,
                hotkeys,
                clock: Clock::new(),
            },
            window::oldest().and_then(window::enable_mouse_passthrough),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        let mut task = None;

        match message {
            Message::Clock(message) => self.clock.update(message),
            Message::HotKey(id) => match self.hotkeys.get(&id) {
                Some(KeyAction::Exit) => task = Some(window::oldest().and_then(window::close)),
                Some(KeyAction::Add) => self.count += 1,
                Some(KeyAction::Toggle) => self.show = !self.show,
                None => (),
            },
            Message::Sleep => (),
        }
        if let Some(task) = task {
            task
        } else {
            iced::Task::none()
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        if !self.show {
            return widget::space().into();
        }
        let mut column = widget::column![widget::text!("Count: {}", self.count)].spacing(10);
        if let Some(clock) = self.clock.view() {
            column = column.push(clock.map(Message::Clock));
        }
        widget::container(column)
            .align_bottom(Length::Fill)
            .align_left(Length::Fill)
            .padding(10)
            .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            hotkeys::subscription::<Message>(),
            self.clock.subscription().map(Message::Clock),
        ])
    }
}
