use iced::{
    button, executor,
    widget::{Button, Column, Container, Row, Text},
    Application, Clipboard, Command, Element, Length, Settings,
};

pub fn run_gui() -> iced::Result {
    Menu::run(Settings::default())
}

#[derive(Debug, Default)]
struct Events {
    enabled: bool,
    exit: button::State,
    should_exit: bool,
}

#[derive(Debug, Default)]
struct Menu {
    enabled: bool,
    stat_button: button::State,
    record_button: button::State,
    report_button: button::State,
    annotate_button: button::State,
    top_button: button::State,
    bench_button: button::State,
    should_exit: bool,
}

impl Application for Menu {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (Menu, Command<Self::Message>) {
        (Menu::default(), Command::none())
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
            .push(Button::new(&mut self.stat_button, Text::new("stat")))
            .push(Button::new(&mut self.record_button, Text::new("record")))
            .push(Button::new(&mut self.report_button, Text::new("report")))
            .push(Button::new(
                &mut self.annotate_button,
                Text::new("annotate"),
            ))
            .push(Button::new(&mut self.top_button, Text::new("top")))
            .push(Button::new(&mut self.bench_button, Text::new("bench")));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
