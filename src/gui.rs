use iced::{
    button, executor, pane_grid, pick_list, scrollable, text_input,
    widget::{Button, Column, Container, PaneGrid, PickList, Row, Scrollable, Text, Rule, TextInput},
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
    CommandSelected(PerfCommand),
    LaunchCommand,
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

                let data_state = state.panes_state.get_mut(&state.data_pane).unwrap();

                match message {
                    Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                        state.panes_state.resize(&split, ratio);
                    }

                    Message::NewAppPressed => {
                        data_state.context = Context::NewProgram;
                        println!("new app pressed");
                    }

                    Message::CommandSelected(PerfCommand::Stat) => {
                        data_state.selected_command = PerfCommand::Stat;
                        println!("stat selected")
                    }
                    Message::CommandSelected(PerfCommand::Record) => {
                        data_state.selected_command = PerfCommand::Record;
                        println!("record selected")
                    }
                    Message::CommandSelected(PerfCommand::Report) => {
                        data_state.selected_command = PerfCommand::Report;
                        println!("report selected")
                    }
                    Message::CommandSelected(PerfCommand::Annotate) => {
                        data_state.selected_command = PerfCommand::Annotate;
                        println!("annotate selected")
                    }
                    Message::CommandSelected(PerfCommand::Top) => {
                        data_state.selected_command = PerfCommand::Top;
                        println!("top selected")
                    }
                    Message::CommandSelected(PerfCommand::Bench) => {
                        data_state.selected_command = PerfCommand::Bench;
                        println!("bench selected")
                    }
                    Message::CommandSelected(PerfCommand::Test) => {
                        data_state.selected_command = PerfCommand::Test;
                        println!("test selected")
                    }

                    Message::InputChanged(value) => {
                        data_state.input_value = value;
                    }

                    Message::LaunchCommand => {

                        match data_state.selected_command {
                            PerfCommand::Stat => {
                                //TODO: Add program here
                        data_state.data = format!("Stat");
                            }
                            PerfCommand::Record => {
                                //TODO: Add program here
                        data_state.data = format!("record");
                            }
                            PerfCommand::Report => {
                                //TODO: Add program here
                        data_state.data = format!("report");
                            }
                            PerfCommand::Annotate => {
                                //TODO: Add program here
                        data_state.data = format!("Annotate");
                            }
                            PerfCommand::Top => {
                                //TODO: Add program here
                        data_state.data = format!("Top");
                            }
                            PerfCommand::Bench => {
                                //TODO: Add program here
                        data_state.data = format!("Bench");
                            }
                            PerfCommand::Test => {
                        data_state.data = format!("Test");
                                //TODO: Add program here
                            }
                        }

                        data_state.context = Context::Main;
                        
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
                panes_state,
                panes_created,
                ..
            }) => {
                let panes = PaneGrid::new(panes_state, |pane, content| {
                    let title = Row::with_children(vec![Text::new(content.id.to_string()).into()])
                        .spacing(5);

                    let title_bar = pane_grid::TitleBar::new(title).padding(10);

                    let pick_list = PickList::new(
                        &mut content.pick_list,
                        &PerfCommand::ALL[..],
                        Some(content.selected_command),
                        Message::CommandSelected,
                    );

                    let list = Scrollable::new(&mut content.scroll)
                        .width(Length::Fill)
                        .align_items(Align::Start)
                        .spacing(10)
                        .push(Text::new("Select a program to run"))
                        .push(pick_list);

                    let input = TextInput::new(
                        &mut content.input , 
                        "", 
                        &mut content.input_value, 
                        Message::InputChanged
                    );

                    pane_grid::Content::new(match content.pane_type {
                        PaneType::Task => Container::new(
                            Column::new()
                                .spacing(5)
                                .padding(5)
                                .width(Length::Fill)
                                .align_items(Align::Start)
                                .push(
                                    Button::new(&mut content.create_button, Text::new("new"))
                                        .on_press(Message::NewAppPressed)
                                        .width(Length::FillPortion(100)),
                                ),
                        )
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(5),

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
                                    .push(Column::with_children(vec![
                                        list.into(),
                                        Rule::horizontal(100).into(),
                                        Text::new("Program to run:").into(),
                                        input.into(),
                                        Rule::horizontal(100).into(),
                                        Text::new("Options:").into(),
                                        Rule::horizontal(100).into(),
                                        Button::new(
                                            &mut content.launch_button,
                                            Text::new("Launch"),
                                        )
                                        .on_press(Message::LaunchCommand)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerfCommand {
    Stat,
    Record,
    Report,
    Annotate,
    Top,
    Bench,
    Test,
}

impl PerfCommand {
    const ALL: [PerfCommand; 7] = [
        PerfCommand::Annotate,
        PerfCommand::Bench,
        PerfCommand::Record,
        PerfCommand::Report,
        PerfCommand::Stat,
        PerfCommand::Test,
        PerfCommand::Top,
    ];
}

impl Default for PerfCommand {
    fn default() -> PerfCommand {
        PerfCommand::Test
    }
}

impl std::fmt::Display for PerfCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PerfCommand::Annotate => "Annotate",
                PerfCommand::Bench => "Bench",
                PerfCommand::Record => "Record",
                PerfCommand::Report => "Report",
                PerfCommand::Stat => "Stat",
                PerfCommand::Test => "Test",
                PerfCommand::Top => "Top",
            }
        )
    }
}

mod style {
    use iced::{container, Background, Color};

    const SURFACE: Color = Color::from_rgb(
        0xF2 as f32 / 255.0,
        0xF3 as f32 / 255.0,
        0xF5 as f32 / 255.0,
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

enum Context {
    Main,
    NewProgram,
}

enum PaneType {
    Task,
    Data,
    Log,
}

struct Content {
    input_value: String,
    input: text_input::State,
    selected_command: PerfCommand,
    scroll: scrollable::State,
    pick_list: pick_list::State<PerfCommand>,
    id: usize,
    data: String,
    application: String,
    pane_type: PaneType,
    create_button: button::State,
    launch_button: button::State,
    context: Context,
}

impl Content {
    fn new(pane_type: PaneType, id: usize) -> Self {
        Content {
            input_value: String::new(),
            input: text_input::State::new(),
            selected_command: PerfCommand::default(),
            scroll: scrollable::State::new(),
            pick_list: pick_list::State::default(),
            pane_type,
            id,
            data: String::new(),
            create_button: button::State::new(),
            launch_button: button::State::new(),
            application: String::new(),
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
