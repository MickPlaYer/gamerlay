use iced::{
    Element, Subscription,
    time::{every, milliseconds},
    widget,
};

pub struct Clock {
    time: Option<String>,
}

pub enum Message {
    Tick,
}

impl Clock {
    pub fn new() -> Self {
        Self { time: Option::None }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Tick => self.refresh(),
        }
    }

    pub fn view(&self) -> Option<Element<'_, Message>> {
        let time = self.time.clone()?;
        Some(widget::text!("{}", time).into())
    }

    pub fn subscription(&self) -> Subscription<Message> {
        every(milliseconds(500)).map(|_| Message::Tick)
    }

    fn refresh(&mut self) {
        let time = time::OffsetDateTime::now_local().unwrap();
        self.time = Some(format!("{:02}:{:02}", time.hour(), time.minute()));
    }
}
