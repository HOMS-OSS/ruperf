use iced::{
    executor,
    widget::{Column, Container, Row, Text},
    Application, Background, Clipboard, Color, Command, Element, Length, Settings, Vector,
};

pub fn run_gui() -> iced::Result {
    Hello::run(Settings::default())
}

struct Hello;

impl Application for Hello {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (Hello, Command<Self::Message>) {
        (Hello, Command::none())
    }

    fn title(&self) -> String {
        String::from("Ruperf")
    }

    fn update(
        &mut self,
        _message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let content = Column::new()
            .spacing(20)
            .padding(20)
            .width(Length::Fill)
            .push(Text::new("stat").size(40))
            .push(Text::new("record").size(40))
            .push(Text::new("report").size(40))
            .push(Text::new("annotate").size(40))
            .push(Text::new("top").size(40))
            .push(Text::new("bench").size(40));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Primary,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }
}
