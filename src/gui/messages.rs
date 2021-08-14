//! SPDX-License-Identifier: GPL-2.0
pub mod main {
    use super::task::TaskMessage;
    use crate::gui::events::perf::PerfEvent;
    use crate::gui::state::*;
    /// Messages to be sent to the parent widget from
    /// other child widgets, and consumed on update
    #[derive(Debug, Clone)]
    pub enum Message {
        Loaded(Result<save_load::SavedState, save_load::LoadError>),
        Saved(Result<(), save_load::SaveError>),
        InputChanged(String),
        NewAppPressed,
        CommandSelected(PerfEvent),
        CyclesToggled(bool),
        InstructionsToggled(bool),
        JsonToggled(bool),
        ListToggled(bool),
        VerboseToggled(bool),
        LaunchCommand,
        RecieveTask(usize, TaskMessage),
    }
}

pub mod task {
    #[derive(Debug, Clone)]
    pub enum TaskMessage {
        Run,
    }
}
