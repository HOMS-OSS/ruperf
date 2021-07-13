use iced::{
    button, executor, pane_grid,
    widget::{Button, Column, Container, PaneGrid, Row, Scrollable, Text},
    Align, Application, Clipboard, Command, Element, Length, Settings,
};

pub fn run_gui() -> iced::Result {
    Menu::run(Settings::default())
}

struct Content {
    id: usize,
    stat_button: button::State,
    record_button: button::State,
    report_button: button::State,
    annotate_button: button::State,
    top_button: button::State,
    bench_button: button::State,
}

impl Content {
    fn new(id: usize) -> Self {
        Content {
            id,
            stat_button: button::State::new(),
            record_button: button::State::new(),
            report_button: button::State::new(),
            annotate_button: button::State::new(),
            top_button: button::State::new(),
            bench_button: button::State::new(),
        }
    }

    fn view(&mut self, pane: pane_grid::Pane, total_panes: usize) -> Element<Message> {
        let content = Column::new()
            .spacing(5)
            .padding(5)
            .width(Length::Fill)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.stat_button, Text::new("stat"))
                    .on_press(Message::stat_pressed)
                    .width(Length::FillPortion(100)),
            )
            .push(
                Button::new(&mut self.record_button, Text::new("record"))
                    .on_press(Message::record_pressed)
                    .width(Length::FillPortion(100)),
            )
            .push(
                Button::new(&mut self.report_button, Text::new("report"))
                    .on_press(Message::report_pressed)
                    .width(Length::FillPortion(100)),
            )
            .push(
                Button::new(&mut self.annotate_button, Text::new("annotate"))
                    .on_press(Message::annotate_pressed)
                    .width(Length::FillPortion(100)),
            )
            .push(
                Button::new(&mut self.top_button, Text::new("top"))
                    .on_press(Message::top_pressed)
                    .width(Length::FillPortion(100)),
            )
            .push(
                Button::new(&mut self.bench_button, Text::new("bench"))
                    .on_press(Message::bench_pressed)
                    .width(Length::FillPortion(100)),
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .center_x()
            .center_y()
            .into()
    }
}

struct Menu {
    panes_state: pane_grid::State<Content>,
    panes_created: usize,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    stat_pressed,
    record_pressed,
    report_pressed,
    annotate_pressed,
    top_pressed,
    bench_pressed,
    Resized(pane_grid::ResizeEvent),
}

impl Application for Menu {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Menu, Command<Self::Message>) {
        //state list
        let (mut panes_state, first_pane) = pane_grid::State::new(Content::new(0));

        let (second_pane, vert_split) = panes_state
            .split(pane_grid::Axis::Vertical, &first_pane, Content::new(1))
            .unwrap();

        let (third_pane, horz_split) = panes_state
            .split(pane_grid::Axis::Horizontal, &second_pane, Content::new(1))
            .unwrap();

        panes_state.resize(&vert_split, 0.17);
        panes_state.resize(&horz_split, 0.88);

        (
            Menu {
                panes_state,
                panes_created: 1,
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
                println!("ratio: {}", ratio);
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

            pane_grid::Content::new(content.view(pane, 1))
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
    use iced::{button, container, Background, Color, Vector};

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
