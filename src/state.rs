use crate::models::RatingsData;
use std::collections::HashMap;
use tui::widgets::TableState;

/// State of the interface.
pub struct App {
    pub tab_index: usize,
    pub table_state: TableState,
    time_cache: HashMap<u64, RatingsData>,
}

impl App {
    /// Create a new interface state from the VATSIM pilot data.
    pub fn new() -> Self {
        let mut state = TableState::default();
        state.select(Some(0));
        Self {
            tab_index: 0,
            table_state: state,
            time_cache: HashMap::new(),
        }
    }

    /// Scroll down the table. Wrap-around supported.
    pub fn down(&mut self, row_count: usize) {
        let sel = self.table_state.selected().unwrap_or(0);
        let next = if sel >= row_count - 1 { 0 } else { sel + 1 };
        self.table_state.select(Some(next));
    }

    /// Scroll up the table. Wrap-around supported.
    pub fn up(&mut self, row_count: usize) {
        let sel = self.table_state.selected().unwrap_or(0);
        let next = if sel == 0 { row_count - 1 } else { sel - 1 };
        self.table_state.select(Some(next));
    }

    /// Get an item from the pilot time cache.
    pub fn pilot_time_cached(&self, cid: u64) -> Option<RatingsData> {
        self.time_cache
            .get(&cid)
            .map(std::borrow::ToOwned::to_owned)
    }

    /// Update the pilot time cache.
    pub fn update_pilot_time_cache(&mut self, cid: u64, ratings_data: &RatingsData) {
        let _ = self.time_cache.insert(cid, ratings_data.clone());
    }
}
