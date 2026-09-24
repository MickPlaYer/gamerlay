mod clock;
mod dot;
mod hotkey;

use crate::overlay::clock::Clock;
use crate::overlay::dot::Dot;
use crate::overlay::hotkey::{HotKeyMessage, KeyAction};
use global_hotkey::GlobalHotKeyManager;
use iced::{
    Element, Length, Subscription, Task,
    widget::{container, grid, space},
    window,
};
use std::collections::HashMap;

enum Cycle {
    All,
    Clock,
    Dot,
    None,
}

impl Cycle {
    fn next(&mut self) {
        *self = match self {
            Cycle::All => Cycle::Clock,
            Cycle::Clock => Cycle::Dot,
            Cycle::Dot => Cycle::None,
            Cycle::None => Cycle::All,
        }
    }
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

    fn sleep() -> Self {
        Message::Sleep
    }
}

pub struct Overlay {
    show: bool,
    cycle: Cycle,
    clock: Clock,
    dot: Dot,
    _hotkey_manager: GlobalHotKeyManager,
    hotkeys: HashMap<u32, KeyAction>,
}

impl Overlay {
    pub fn new() -> (Self, Task<Message>) {
        let (manager, hotkeys) = hotkey::build();
        (
            Self {
                cycle: Cycle::All,
                show: true,
                _hotkey_manager: manager,
                hotkeys,
                clock: Clock::new(),
                dot: Dot::new(),
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
                Some(KeyAction::Cycle) => self.cycle.next(),
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
            return space().into();
        }
        let (need_clock, need_dot) = match self.cycle {
            Cycle::All => (true, true),
            Cycle::Clock => (true, false),
            Cycle::Dot => (false, true),
            Cycle::None => (false, false),
        };
        let mut grid = grid!().columns(3).height(Length::Fill);
        grid = if need_clock && let Some(clock) = self.clock.view() {
            grid.push(
                container(clock.map(Message::Clock))
                    .align_bottom(Length::Fill)
                    .align_left(Length::Fill)
                    .padding(10),
            )
        } else {
            grid.push(space())
        };
        grid = if need_dot {
            grid.push(container(self.dot.view()).center(Length::Fill))
        } else {
            grid.push(space())
        };
        grid.into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            hotkey::subscription::<Message>(),
            self.clock.subscription().map(Message::Clock),
        ])
    }
}
