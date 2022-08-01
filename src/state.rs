use crate::models::Pilot;
use tui::widgets::TableState;

/// State of the interface.
pub struct App {
    tab_index: usize,
    table_state: TableState,
    pilots: Vec<Pilot>,
}

impl App {
    /// Create a new interface state from the VATSIM pilot data.
    pub fn new(pilots: Vec<Pilot>) -> Self {
        let mut state = TableState::default();
        state.select(Some(0));
        Self {
            tab_index: 0,
            table_state: state,
            pilots,
        }
    }

    /// Update the stored pilots.
    pub fn update(&mut self, pilots: Vec<Pilot>) {
        self.pilots = pilots;
    }

    /// Scroll down the table. Wrap-around supported.
    pub fn down(&mut self) {
        let sel = self.table_state.selected().unwrap_or(0);
        let length = self.pilots.len();
        let next = if sel >= length - 1 { 0 } else { sel + 1 };
        self.table_state.select(Some(next));
    }

    /// Scroll up the table. Wrap-around supported.
    pub fn up(&mut self) {
        let sel = self.table_state.selected().unwrap_or(0);
        let length = self.pilots.len();
        let next = if sel == 0 { length - 1 } else { sel - 1 };
        self.table_state.select(Some(next));
    }
}
