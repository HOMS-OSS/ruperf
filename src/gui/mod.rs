//! Gui driver
use iced::{
    button, executor, pane_grid, pick_list, scrollable, text_input,
    widget::{
        Button, Column, Container, PaneGrid, PickList, Row, Rule, Scrollable, Text, TextInput,
    },
    Align, Application, Clipboard, Command, Element, Length, Settings,
};
use serde::{Deserialize, Serialize};

/// Run the Gui Launcher
pub fn run_gui() -> iced::Result {
    Gui::run(Settings::default())
}

/// Main States for all Gui elements
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

/// Default state for Gui
impl Default for State {
    fn default() -> Self {
        // First pane and first state is created here:
        // task Pane, panes_state
        let (mut panes_state, task_pane) = pane_grid::State::new(Content::new(PaneType::Task, 0));

        // Second pane and first split is created here:
        // data_pane, vert_split
        let (data_pane, vert_split) = panes_state
            .split(
                pane_grid::Axis::Vertical,
                &task_pane,
                Content::new(PaneType::Main, 1),
            )
            .unwrap();

        // Third plane and second split is created here:
        // log_pane, horz_split
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
/// Messages to be sent to the parent widget from
/// other child widgets, and consumed on update
#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
    InputChanged(String),
    NewAppPressed,
    Resized(pane_grid::ResizeEvent),
    CommandSelected(PerfEvent),
    LaunchCommand,
}
/// Provide methods for Gui renderer
impl Application for Gui {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    /// Initialize state of Gui parent element
    fn new(_flags: ()) -> (Gui, Command<Self::Message>) {
        (
            Gui::Loading,
            Command::perform(SavedState::load(), Message::Loaded),
        )
    }
    /// Set title for Gui parent element
    fn title(&self) -> String {
        String::from("Ruperf")
    }
    /// Update Gui based on recieved Message flags
    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match self {
            // Update Loading consumed for Gui
            // then changed to loaded based on
            // Loading function
            Gui::Loading => match message {
                Message::Loaded(Ok(state)) => {
                    *self = Gui::Loaded(State {
                        tasks: state.tasks,
                        ..State::default()
                    })
                }
                // When load file is not found
                // set state to default
                Message::Loaded(Err(_)) => {
                    *self = Gui::Loaded(State::default());
                }

                _ => {
                    println!("error")
                }
            },

            // When Gui is loaded prepare to recieve message
            // callbacks from children widgets
            Gui::Loaded(state) => {
                let mut saved = false;

                let data_state = state.panes_state.get_mut(&state.data_pane).unwrap();

                match message {
                    Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                        state.panes_state.resize(&split, ratio);
                    }

                    Message::NewAppPressed => {
                        data_state.context = Context::NewEvent;
                        println!("new app pressed");
                    }

                    Message::CommandSelected(PerfEvent::Stat) => {
                        data_state.selected_command = PerfEvent::Stat;
                        println!("stat selected")
                    }
                    Message::CommandSelected(PerfEvent::Record) => {
                        data_state.selected_command = PerfEvent::Record;
                        println!("record selected")
                    }
                    Message::CommandSelected(PerfEvent::Report) => {
                        data_state.selected_command = PerfEvent::Report;
                        println!("report selected")
                    }
                    Message::CommandSelected(PerfEvent::Annotate) => {
                        data_state.selected_command = PerfEvent::Annotate;
                        println!("annotate selected")
                    }
                    Message::CommandSelected(PerfEvent::Top) => {
                        data_state.selected_command = PerfEvent::Top;
                        println!("top selected")
                    }
                    Message::CommandSelected(PerfEvent::Bench) => {
                        data_state.selected_command = PerfEvent::Bench;
                        println!("bench selected")
                    }
                    Message::CommandSelected(PerfEvent::Test) => {
                        data_state.selected_command = PerfEvent::Test;
                        println!("test selected")
                    }

                    Message::InputChanged(value) => {
                        data_state.input_value = value;
                    }

                    Message::LaunchCommand => {
                        match data_state.selected_command {
                            PerfEvent::Stat => {
                                //TODO: Add program here
                                data_state.data = format!("Stat output:");
                            }
                            PerfEvent::Record => {
                                //TODO: Add program here
                                data_state.data = format!("Record output:");
                            }
                            PerfEvent::Report => {
                                //TODO: Add program here
                                data_state.data = format!("Report output:");
                            }
                            PerfEvent::Annotate => {
                                //TODO: Add program here
                                data_state.data = format!("Annotate output:");
                            }
                            PerfEvent::Top => {
                                //TODO: Add program here
                                data_state.data = format!("Top output:");
                            }
                            PerfEvent::Bench => {
                                //TODO: Add program here
                                data_state.data = format!("Bench output:");
                            }
                            PerfEvent::Test => {
                                data_state.data = format!("Test output:");
                                //TODO: Add program here
                            }
                        }

                        // Switch data panel to main view,
                        // and PerfEvent output
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
    /// Display Graphics to screen
    fn view(&mut self) -> Element<Self::Message> {
        match self {
            Gui::Loading => loading_message(),
            Gui::Loaded(State {
                panes_state,
                panes_created,
                ..
            }) => {
                // Iterate entire pane grid and display each
                // with thier own content
                let panes = PaneGrid::new(panes_state, |pane, content| {
                    let title = Row::with_children(vec![Text::new(content.id.to_string()).into()])
                        .spacing(5);

                    // Title of pane
                    let title_bar = pane_grid::TitleBar::new(title).padding(10);

                    // Initialize list of elements
                    let list = PickList::new(
                        &mut content.pick_list,
                        &PerfEvent::ALL[..],
                        Some(content.selected_command),
                        Message::CommandSelected,
                    );

                    // Initialize scrollable list of elements
                    let scrollable_list = Scrollable::new(&mut content.scroll)
                        .width(Length::Fill)
                        .align_items(Align::Start)
                        .spacing(10)
                        .push(Text::new("Select a program to run"))
                        .push(list);

                    // Initialize Input field
                    let input = TextInput::new(
                        &mut content.input,
                        "",
                        &mut content.input_value,
                        Message::InputChanged,
                    )
                    .width(Length::from(200));

                    // Pane main container dependant on the given PaneType:
                    //--------------------------------------------------------------------
                    // Task: previously ran events, or creating new event
                    // Main: main input for event creation, viewing output from ran events
                    // Log:  viewing logs for debug purposes
                    //---------------------------------------------------------------------
                    pane_grid::Content::new(match content.pane_type {
                        // Task pane
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

                        // data_pane will switch visual context based on outside events:
                        // Main: view data of running event (default)
                        // NewEvent: generate menu for creating events
                        PaneType::Main => match content.context {
                            Context::Main => Container::new(
                                Column::new()
                                    .spacing(5)
                                    .padding(5)
                                    .width(Length::Fill)
                                    .align_items(Align::Center)
                                    .push(Text::new(&content.data)),
                            ),

                            Context::NewEvent => Container::new(
                                Column::new()
                                    .spacing(5)
                                    .padding(5)
                                    .width(Length::Fill)
                                    .align_items(Align::Center)
                                    .push(Column::with_children(vec![
                                        scrollable_list.into(),
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
                        // Log pane
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

                // Collect all panes and add them to main Gui element
                let content = Column::new()
                    .spacing(5)
                    .padding(5)
                    .width(Length::Fill)
                    .align_items(Align::Center)
                    .push(panes);

                // Display all widget elements
                Container::new(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }
        }
    }
}

/// Perf Commands to be used
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerfEvent {
    Stat,
    Record,
    Report,
    Annotate,
    Top,
    Bench,
    Test,
}

/// Holds an enumerated array of PerfEvents
impl PerfEvent {
    const ALL: [PerfEvent; 7] = [
        PerfEvent::Annotate,
        PerfEvent::Bench,
        PerfEvent::Record,
        PerfEvent::Report,
        PerfEvent::Stat,
        PerfEvent::Test,
        PerfEvent::Top,
    ];
}

/// Default PerfEvent
impl Default for PerfEvent {
    fn default() -> PerfEvent {
        PerfEvent::Test
    }
}

/// Provide PerfEvents as String data types
impl std::fmt::Display for PerfEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PerfEvent::Annotate => "Annotate",
                PerfEvent::Bench => "Bench",
                PerfEvent::Record => "Record",
                PerfEvent::Report => "Report",
                PerfEvent::Stat => "Stat",
                PerfEvent::Test => "Test",
                PerfEvent::Top => "Top",
            }
        )
    }
}

/// Widget Style
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

/// Message to display while Gui is loading
fn loading_message<'a>() -> Element<'a, Message> {
    Container::new(Text::new("Loading...").size(50))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .into()
}

/// Main pane Contexts
enum Context {
    Main,
    NewEvent,
}

/// Pane Type
enum PaneType {
    Task,
    Main,
    Log,
}

/// States of all panes within the pane grid
// every pane state must be held here
struct Content {
    input_value: String,
    input: text_input::State,
    selected_command: PerfEvent,
    scroll: scrollable::State,
    pick_list: pick_list::State<PerfEvent>,
    id: usize,
    data: String,
    application: String,
    pane_type: PaneType,
    create_button: button::State,
    launch_button: button::State,
    context: Context,
}

/// Initialize pane states to default values
impl Content {
    fn new(pane_type: PaneType, id: usize) -> Self {
        Content {
            input_value: String::new(),
            input: text_input::State::new(),
            selected_command: PerfEvent::default(),
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
// Currently running or previously ran events
struct Task {
    name: String,
    application: String,
    options: Vec<String>,
}

//customized from iced todo example.
// source: https://github.com/hecrj/iced/blob/0.3/examples/todos/src/main.rs

//Persistance
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedState {
    tasks: Vec<Task>,
}

#[derive(Debug, Clone)]
/// Error type for load function
enum LoadError {
    FileError,
    FormatError,
}

#[derive(Debug, Clone)]
/// Error type for save function
enum SaveError {
    FileError,
    WriteError,
    FormatError,
}

#[cfg(not(target_arch = "wasm32"))]
/// Saved state for Gui
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
// Saved state for Gui (wasm32)
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
