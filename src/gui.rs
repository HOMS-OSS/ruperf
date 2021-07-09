use iced::{
    button, executor,
    widget::{Button, Column, Container, Row, Text},
    Application, Clipboard, Command, Element, Length, Settings,
};

pub fn run_gui() -> iced::Result {
    Menu::run(Settings::default())
}

#[derive(Debug, Default)]
struct Menu {
    description: String,
    enabled: bool,
    stat_button: button::State,
    record_button: button::State,
    report_button: button::State,
    annotate_button: button::State,
    top_button: button::State,
    bench_button: button::State,
    should_exit: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    stat_pressed,
    record_pressed,
    report_pressed,
    annotate_pressed,
    top_pressed,
    bench_pressed,
}

impl Application for Menu {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Menu, Command<Self::Message>) {
        (Menu::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Ruperf")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::stat_pressed => {
                self.description = String::from("stat");
            }
            Message::record_pressed => {
                self.description = String::from("record");
            }
            Message::report_pressed => {
                self.description = String::from("report");
            }
            Message::annotate_pressed => {
                self.description = String::from("annotate");
            }
            Message::top_pressed => {
                self.description = String::from("top");
            }
            Message::bench_pressed => {
                self.description = String::from("bench");
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let content = Column::new()
            .spacing(5)
            .padding(5)
            .width(Length::Fill)
            .push(
                Button::new(&mut self.stat_button, Text::new("stat"))
                    .on_press(Message::stat_pressed)
                    .width(Length::Fill),
            )
            .push(
                Button::new(&mut self.record_button, Text::new("record"))
                    .on_press(Message::record_pressed)
                    .width(Length::Fill),
            )
            .push(
                Button::new(&mut self.report_button, Text::new("report"))
                    .on_press(Message::report_pressed)
                    .width(Length::Fill),
            )
            .push(
                Button::new(&mut self.annotate_button, Text::new("annotate"))
                    .on_press(Message::annotate_pressed)
                    .width(Length::Fill),
            )
            .push(
                Button::new(&mut self.top_button, Text::new("top"))
                    .on_press(Message::top_pressed)
                    .width(Length::Fill),
            )
            .push(
                Button::new(&mut self.bench_button, Text::new("bench"))
                    .on_press(Message::bench_pressed)
                    .width(Length::Fill),
            )
            .push(Text::new(&self.description));

        Container::new(content)
            .width(Length::from(100))
            .height(Length::Fill)
            .into()
    }
}
