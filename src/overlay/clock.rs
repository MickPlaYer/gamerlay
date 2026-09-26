use iced::{
    Background, Border, Color, Element, Font, Subscription,
    border::Radius,
    font::{Family, Weight},
    time::{every, milliseconds},
    widget::{container, text},
};

pub enum Message {
    Tick,
}

pub struct Clock {
    time: Option<String>,
}

impl Clock {
    const TEXT_COLOR: Color = Color::from_rgba8(142, 151, 159, 0.7);
    const BACK_COLOR: Color = Color::from_rgba8(33, 37, 41, 0.7);

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
        Some(
            container(text!("{time}").size(32).font(Font {
                family: Family::Name("Cascadia Mono"),
                weight: Weight::Bold,
                ..Default::default()
            }))
            .style(|_| container::Style {
                text_color: Some(Self::TEXT_COLOR),
                background: Some(Background::Color(Self::BACK_COLOR)),
                border: Border {
                    radius: Radius::new(7),
                    ..Default::default()
                },
                ..Default::default()
            })
            .padding([5, 10])
            .into(),
        )
    }

    pub fn subscription(&self) -> Subscription<Message> {
        every(milliseconds(500)).map(|_| Message::Tick)
    }

    fn refresh(&mut self) {
        let time = time::OffsetDateTime::now_local().unwrap();
        self.time = Some(format!("{:02}:{:02}", time.hour(), time.minute()));
    }
}
