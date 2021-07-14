use iced::{
    button, executor, pane_grid,
    widget::{Button, Column, Container, PaneGrid, Row, Text},
    Align, Application, Clipboard, Command, Element, Length, Settings,
};

pub fn run_gui() -> iced::Result {
    Menu::run(Settings::default())
}

struct Menu {
    panes_state: pane_grid::State<Content>,
    panes_created: usize,
    data_pane: pane_grid::Pane,
    log_pane: pane_grid::Pane,
    task_pane: pane_grid::Pane,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    StatPressed,
    RecordPressed,
    ReportPressed,
    AnnotatePressed,
    TopPressed,
    BenchPressed,
    Resized(pane_grid::ResizeEvent),
}

impl Application for Menu {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Menu, Command<Self::Message>) {
        let (mut panes_state, task_pane) = pane_grid::State::new(Content::new(PaneType::Task, 0));

        let (data_pane, vert_split) = panes_state
            .split(
                pane_grid::Axis::Vertical,
                &task_pane,
                Content::new(PaneType::Data, 1),
            )
            .unwrap();

        let (log_pane, horz_split) = panes_state
            .split(
                pane_grid::Axis::Horizontal,
                &data_pane,
                Content::new(PaneType::Log, 2),
            )
            .unwrap();

        panes_state.resize(&vert_split, 0.17);
        panes_state.resize(&horz_split, 0.88);

        (
            Menu {
                panes_state,
                panes_created: 1,
                data_pane,
                task_pane,
                log_pane,
            },
            Command::none(),
        )
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
            Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes_state.resize(&split, ratio);
            }

            Message::StatPressed => {
                let data_state = self.panes_state.get_mut(&self.data_pane).unwrap();
                data_state.data = "stat".to_string();
                println!("stat pressed");
            }
            Message::RecordPressed => {
                let data_state = self.panes_state.get_mut(&self.data_pane).unwrap();
                data_state.data = "record".to_string();
                println!("record pressed")
            }
            Message::ReportPressed => {
                let data_state = self.panes_state.get_mut(&self.data_pane).unwrap();
                data_state.data = "record".to_string();
                println!("report pressed")
            }
            Message::AnnotatePressed => {
                let data_state = self.panes_state.get_mut(&self.data_pane).unwrap();
                data_state.data = "annotate".to_string();
                println!("annotate pressed")
            }
            Message::TopPressed => {
                let data_state = self.panes_state.get_mut(&self.data_pane).unwrap();
                data_state.data = "top".to_string();
                println!("top pressed")
            }
            Message::BenchPressed => {
                let data_state = self.panes_state.get_mut(&self.data_pane).unwrap();
                data_state.data = "bench".to_string();
                println!("bench pressed")
            }

            _ => {
                println!("other")
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let panes = PaneGrid::new(&mut self.panes_state, |pane, content| {
            let title =
                Row::with_children(vec![Text::new(content.id.to_string()).into()]).spacing(5);

            let title_bar = pane_grid::TitleBar::new(title).padding(10);

            pane_grid::Content::new(match content.pane_type {
                PaneType::Task => Container::new(
                    Column::new()
                        .spacing(5)
                        .padding(5)
                        .width(Length::Fill)
                        .align_items(Align::Center)
                        .push(
                            Button::new(&mut content.stat_button, Text::new("stat"))
                                .on_press(Message::StatPressed)
                                .width(Length::FillPortion(100)),
                        )
                        .push(
                            Button::new(&mut content.record_button, Text::new("record"))
                                .on_press(Message::RecordPressed)
                                .width(Length::FillPortion(100)),
                        )
                        .push(
                            Button::new(&mut content.report_button, Text::new("report"))
                                .on_press(Message::ReportPressed)
                                .width(Length::FillPortion(100)),
                        )
                        .push(
                            Button::new(&mut content.annotate_button, Text::new("annotate"))
                                .on_press(Message::AnnotatePressed)
                                .width(Length::FillPortion(100)),
                        )
                        .push(
                            Button::new(&mut content.top_button, Text::new("top"))
                                .on_press(Message::TopPressed)
                                .width(Length::FillPortion(100)),
                        )
                        .push(
                            Button::new(&mut content.bench_button, Text::new("bench"))
                                .on_press(Message::BenchPressed)
                                .width(Length::FillPortion(100)),
                        ),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(5)
                .center_x()
                .center_y(),

                PaneType::Data => Container::new(
                    Column::new()
                        .spacing(5)
                        .padding(5)
                        .width(Length::Fill)
                        .align_items(Align::Center)
                        .push(Text::new(&content.data)),
                ),

                PaneType::Log => Container::new(
                    Column::new()
                        .spacing(5)
                        .padding(5)
                        .width(Length::Fill)
                        .align_items(Align::Center)
                        .push(Text::new("Logs")),
                ),

                _ => Container::new(
                    Column::new()
                        .spacing(5)
                        .padding(5)
                        .width(Length::Fill)
                        .align_items(Align::Center)
                        .push(Text::new("Other")),
                ),
            })
            .title_bar(title_bar)
            .style(style::Pane { is_focused: true })
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .on_resize(10, Message::Resized)
        .spacing(10);

        let content = Column::new()
            .spacing(5)
            .padding(5)
            .width(Length::Fill)
            .align_items(Align::Center)
            .push(panes);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

mod style {
    use iced::{container, Background, Color};

    const SURFACE: Color = Color::from_rgb(
        0xF2 as f32 / 255.0,
        0xF3 as f32 / 255.0,
        0xF5 as f32 / 255.0,
    );

    const ACTIVE: Color = Color::from_rgb(
        0x72 as f32 / 255.0,
        0x89 as f32 / 255.0,
        0xDA as f32 / 255.0,
    );

    const HOVERED: Color = Color::from_rgb(
        0x67 as f32 / 255.0,
        0x7B as f32 / 255.0,
        0xC4 as f32 / 255.0,
    );

    pub struct Pane {
        pub is_focused: bool,
    }

    impl container::StyleSheet for Pane {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(SURFACE)),
                border_width: 2.0,
                border_color: if self.is_focused {
                    Color::BLACK
                } else {
                    Color::from_rgb(0.7, 0.7, 0.7)
                },
                ..Default::default()
            }
        }
    }
}

struct Content {
    id: usize,
    data: String,
    pane_type: PaneType,
    stat_button: button::State,
    record_button: button::State,
    report_button: button::State,
    annotate_button: button::State,
    top_button: button::State,
    bench_button: button::State,
}

enum PaneType {
    Task,
    Data,
    Log,
}

impl Content {
    fn new(pane_type: PaneType, id: usize) -> Self {
        Content {
            pane_type,
            id,
            data: "".to_string(),
            stat_button: button::State::new(),
            record_button: button::State::new(),
            report_button: button::State::new(),
            annotate_button: button::State::new(),
            top_button: button::State::new(),
            bench_button: button::State::new(),
        }
    }
}
