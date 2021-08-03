//! # GUI driver.
//! <p> Usage: <em> ruperf gui [COMMAND] </em>
//! where COMMAND is one of: </p>
//! <ul>
//! <li>test</li>
//! <li>stat</li>
//! </ul>

extern crate structopt;
use structopt::StructOpt;

mod pane_content;
mod perf_event;
mod save_state;
mod state;
mod style;

use iced::{
    executor, pane_grid,
    widget::{
        Button, Column, Container, PaneGrid, PickList, Row, Rule, Scrollable, Text, TextInput,
    },
    Align, Application, Clipboard, Command, Element, Length, Settings,
};
use pane_content::*;
use perf_event::*;
use save_state::*;
use state::*;

/// Run the Gui Launcher
pub fn run_gui(options: &GuiOptions) -> iced::Result {
    Gui::run(Settings::default())
}

/// Main States for all Gui elements
enum Gui {
    Loading,
    Loaded(State),
}

/// Configuration settings for running the GUI
#[derive(Debug, StructOpt)]
pub struct GuiOptions {}

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
                    .style(style::widget::Pane { is_focused: true })
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

/// Message to display while Gui is loading
fn loading_message<'a>() -> Element<'a, Message> {
    Container::new(Text::new("Loading...").size(50))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .into()
}
