use iced::{
    button, executor, pane_grid,
    widget::{Button, Column, Container, PaneGrid, Row, Text},
    Align, Application, Clipboard, Command, Element, Length, Settings,
};

use serde::{Deserialize, Serialize};

pub fn run_gui() -> iced::Result {
    Gui::run(Settings::default())
}

enum Gui {
    Loading,
    Loaded(State),
}

struct State {
    tasks: Vec<Task>,
    panes_state: pane_grid::State<Content>,
    panes_created: usize,
    data_pane: pane_grid::Pane,
    log_pane: pane_grid::Pane,
    task_pane: pane_grid::Pane,
}

//default state for menu
impl Default for State {
    fn default() -> Self {
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

        let tasks = Vec::new();

        State {
            tasks,
            panes_state,
            panes_created: 3,
            data_pane,
            task_pane,
            log_pane,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
    InputChanged(String),
    NewAppPressed,
    Resized(pane_grid::ResizeEvent),
}

impl Application for Gui {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Gui, Command<Self::Message>) {
        (
            Gui::Loading,
            Command::perform(SavedState::load(), Message::Loaded),
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
        match self {
            Gui::Loading => match message {
                Message::Loaded(Ok(state)) => {
                    *self = Gui::Loaded(State {
                        tasks: state.tasks,
                        ..State::default()
                    })
                }
                Message::Loaded(Err(_)) => {
                    *self = Gui::Loaded(State::default());
                }

                _ => {
                    println!("other")
                }
            },

            Gui::Loaded(state) => {
                let mut saved = false;

                match message {
                    Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                        state.panes_state.resize(&split, ratio);
                    }

                    Message::NewAppPressed => {
                        let data_state = state.panes_state.get_mut(&state.data_pane).unwrap();
                        data_state.context = Context::NewProgram;
                        println!("new app pressed");
                    }

                    _ => {
                        println!("other")
                    }
                }
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        match self {
            Gui::Loading => loading_message(),
            Gui::Loaded(State {
                tasks,
                panes_state,
                panes_created,
                data_pane,
                task_pane,
                log_pane,
                ..
            }) => {
                let panes = PaneGrid::new(panes_state, |pane, content| {
                    let title = Row::with_children(vec![Text::new(content.id.to_string()).into()])
                        .spacing(5);

                    let title_bar = pane_grid::TitleBar::new(title).padding(10);

                    pane_grid::Content::new(match content.pane_type {
                        PaneType::Task => Container::new(
                            Column::new()
                                .spacing(5)
                                .padding(5)
                                .width(Length::Fill)
                                .align_items(Align::Center)
                                .push(
                                    Button::new(&mut content.stat_button, Text::new("new"))
                                        .on_press(Message::NewAppPressed)
                                        .width(Length::FillPortion(100)),
                                ),
                        )
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(5)
                        .center_x()
                        .center_y(),

                        // context for the data panel
                        PaneType::Data => match content.context {
                            Context::Main => Container::new(
                                Column::new()
                                    .spacing(5)
                                    .padding(5)
                                    .width(Length::Fill)
                                    .align_items(Align::Center)
                                    .push(Text::new(&content.data)),
                            ),

                            //menu for running a program
                            Context::NewProgram => Container::new(
                                Column::new()
                                    .spacing(5)
                                    .padding(5)
                                    .width(Length::Fill)
                                    .align_items(Align::Center)
                                    .push(Row::with_children(vec![
                                        Text::new(&content.data).into(),
                                        Button::new(&mut content.stat_button, Text::new("Launch"))
                                            .into(),
                                    ])),
                            ),
                        },

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

fn loading_message<'a>() -> Element<'a, Message> {
    Container::new(Text::new("Loading...").size(50))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .into()
}

struct Content {
    id: usize,
    data: String,
    application: String,
    pane_type: PaneType,
    stat_button: button::State,
    context: Context,
}

enum Context {
    Main,
    NewProgram,
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
            application: "".to_string(),
            context: Context::Main,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    name: String,
    application: String,
    options: Vec<String>,
}

//from iced todo example.
// source: https://github.com/hecrj/iced/blob/0.3/examples/todos/src/main.rs

//Persistance
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedState {
    tasks: Vec<Task>,
}

#[derive(Debug, Clone)]
enum LoadError {
    FileError,
    FormatError,
}

#[derive(Debug, Clone)]
enum SaveError {
    FileError,
    WriteError,
    FormatError,
}

#[cfg(not(target_arch = "wasm32"))]
impl SavedState {
    fn path() -> std::path::PathBuf {
        let mut path = if let Some(project_dirs) =
            directories_next::ProjectDirs::from("rs", "ruperf", "Tasks")
        {
            project_dirs.data_dir().into()
        } else {
            std::env::current_dir().unwrap_or(std::path::PathBuf::new())
        };

        path.push("tasks.json");

        path
    }

    async fn load() -> Result<SavedState, LoadError> {
        use async_std::prelude::*;

        let mut contents = String::new();

        let mut file = async_std::fs::File::open(Self::path())
            .await
            .map_err(|_| LoadError::FileError)?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::FileError)?;

        serde_json::from_str(&contents).map_err(|_| LoadError::FormatError)
    }

    async fn save(self) -> Result<(), SaveError> {
        use async_std::prelude::*;

        let json = serde_json::to_string_pretty(&self).map_err(|_| SaveError::FormatError)?;

        let path = Self::path();

        if let Some(dir) = path.parent() {
            async_std::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveError::FileError)?;
        }

        {
            let mut file = async_std::fs::File::create(path)
                .await
                .map_err(|_| SaveError::FileError)?;

            file.write_all(json.as_bytes())
                .await
                .map_err(|_| SaveError::WriteError)?;
        }

        // This is a simple way to save at most once every couple seconds
        async_std::task::sleep(std::time::Duration::from_secs(2)).await;

        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
impl SavedState {
    fn storage() -> Option<web_sys::Storage> {
        let window = web_sys::window()?;

        window.local_storage().ok()?
    }

    async fn load() -> Result<SavedState, LoadError> {
        let storage = Self::storage().ok_or(LoadError::FileError)?;

        let contents = storage
            .get_item("state")
            .map_err(|_| LoadError::FileError)?
            .ok_or(LoadError::FileError)?;

        serde_json::from_str(&contents).map_err(|_| LoadError::FormatError)
    }

    async fn save(self) -> Result<(), SaveError> {
        let storage = Self::storage().ok_or(SaveError::FileError)?;

        let json = serde_json::to_string_pretty(&self).map_err(|_| SaveError::FormatError)?;

        storage
            .set_item("state", &json)
            .map_err(|_| SaveError::WriteError)?;

        let _ = wasm_timer::Delay::new(std::time::Duration::from_secs(2)).await;

        Ok(())
    }
}
