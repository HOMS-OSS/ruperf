use crate::gui::PerfEvent;
/// States of all panes within the pane grid
// every pane state must be held here
use iced::{button, pick_list, scrollable, text_input};
use serde::{Deserialize, Serialize};

pub struct Content {
    pub input_value: String,
    pub input: text_input::State,
    pub selected_command: PerfEvent,
    pub scroll: scrollable::State,
    pub pick_list: pick_list::State<PerfEvent>,
    pub id: usize,
    pub data: String,
    pub application: String,
    pub pane_type: PaneType,
    pub create_button: button::State,
    pub launch_button: button::State,
    pub context: Context,
}

/// Initialize pane states to default values
impl Content {
    pub fn new(pane_type: PaneType, id: usize) -> Self {
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

/// Main pane Contexts
pub enum Context {
    Main,
    NewEvent,
}

/// Pane Type
pub enum PaneType {
    Task,
    Main,
    Log,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Currently running or previously ran events
pub struct Task {
    name: String,
    application: String,
    options: Vec<String>,
}
