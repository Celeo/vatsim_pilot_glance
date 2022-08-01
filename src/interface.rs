use crate::{api::Vatsim, state::App};
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use once_cell::sync::Lazy;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Spans, Text},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, Wrap},
    Terminal,
};

/// Style applied to the table header row.
static NORMAL_STYLE: Lazy<Style> = Lazy::new(|| Style::default().bg(Color::Blue));
/// Style applied to non-header table rows.
static SELECTED_STYLE: Lazy<Style> =
    Lazy::new(|| Style::default().add_modifier(Modifier::REVERSED));

/// Run the TUI.
pub fn run(vatsim: &Vatsim, airport: &str) -> Result<()> {
    // configure terminal
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    enable_raw_mode()?;
    terminal.hide_cursor()?;
    let mut app = App::new(vatsim.get_online_pilots()?);

    todo!()
}
