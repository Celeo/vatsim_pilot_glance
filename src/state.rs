use std::collections::HashMap;
use tui::widgets::TableState;
use vatsim_utils::models::RatingsTimeData;

/// State of the interface.
pub struct App {
    pub table_state: TableState,
    time_cache: HashMap<u64, RatingsTimeData>,
}

impl App {
    /// Create a new interface state from the VATSIM pilot data.
    pub fn new() -> Self {
        Self {
            table_state: TableState::default(),
            time_cache: HashMap::new(),
        }
    }

    /// Scroll down the table. Wrap-around supported.
    pub fn down(&mut self, row_count: usize) {
        if let Some(sel) = self.table_state.selected() {
            let next = if sel >= row_count - 1 { 0 } else { sel + 1 };
            self.table_state.select(Some(next));
        } else {
            self.table_state.select(Some(0));
        }
    }

    /// Scroll up the table. Wrap-around supported.
    pub fn up(&mut self, row_count: usize) {
        if let Some(sel) = self.table_state.selected() {
            let next = if sel == 0 { row_count - 1 } else { sel - 1 };
            self.table_state.select(Some(next));
        } else {
            self.table_state.select(Some(row_count - 1));
        }
    }

    /// Get an item from the pilot time cache.
    pub fn pilot_time_cached(&self, cid: u64) -> Option<RatingsTimeData> {
        self.time_cache.get(&cid).map(ToOwned::to_owned)
    }

    /// Update the pilot time cache.
    pub fn update_pilot_time_cache(&mut self, cid: u64, ratings_data: &RatingsTimeData) {
        let _ = self.time_cache.insert(cid, ratings_data.clone());
    }
}
