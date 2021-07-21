use iced::pane_grid;

use crate::gui::*;

/// State for Gui
pub struct State {
    pub tasks: Vec<Task>,
    pub panes_state: pane_grid::State<Content>,
    pub panes_created: usize,
    pub data_pane: pane_grid::Pane,
    pub log_pane: pane_grid::Pane,
    pub task_pane: pane_grid::Pane,
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
