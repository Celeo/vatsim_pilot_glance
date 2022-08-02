use crate::{api::Vatsim, models::Pilot, state::App, static_data};
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use once_cell::sync::Lazy;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Terminal,
};

/// Maximum distance to look at users.
const MAXIMUM_DISTANCE: f64 = 20.0;
/// Style applied to the table header row.
static NORMAL_STYLE: Lazy<Style> = Lazy::new(|| Style::default().bg(Color::Blue));
/// Style applied to non-header table rows.
static SELECTED_STYLE: Lazy<Style> =
    Lazy::new(|| Style::default().add_modifier(Modifier::REVERSED));

/// Update the app's data.
fn update_data(vatsim: &Vatsim, app: &mut App, airport: &str) -> Result<Vec<(Pilot, f64)>> {
    let online_pilots = vatsim.get_online_pilots()?;
    let pilots_in_range =
        static_data::filter_pilot_distance(&online_pilots, airport, MAXIMUM_DISTANCE)?;
    let pilot_times = pilots_in_range
        .par_iter()
        .map(|&pilot| match app.pilot_time_cached(pilot.cid) {
            Some(time) => (pilot.clone(), time),
            None => {
                let time = vatsim
                    .get_pilot_time(pilot.cid)
                    .expect("Could not get pilot time");
                (pilot.clone(), time)
            }
        })
        .collect();
    Ok(pilot_times)
}

/// Run the TUI.
pub fn run(vatsim: &Vatsim, airport: &str) -> Result<()> {
    // configure terminal
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    enable_raw_mode()?;
    terminal.hide_cursor()?;
    let mut app = App::new();

    let mut last_updated = Instant::now();
    let mut pilots = update_data(vatsim, &mut app, airport)?;

    loop {
        if last_updated.elapsed() >= Duration::from_secs(15) {
            pilots = update_data(vatsim, &mut app, airport)?;
            last_updated = Instant::now();
        }

        let _ = terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .horizontal_margin(1)
                .constraints([Constraint::Length(2), Constraint::Min(0)].as_ref())
                .split(f.size());

            let rows = pilots.iter().map(|(pilot, time)| {
                let aircraft = pilot.flight_plan.as_ref().map_or_else(
                    || "???",
                    |fp| {
                        if !fp.aircraft_faa.is_empty() {
                            &fp.aircraft_faa
                        } else if !fp.aircraft_short.is_empty() {
                            &fp.aircraft_short
                        } else {
                            "???"
                        }
                    },
                );
                Row::new([
                    Cell::from(pilot.callsign.clone()),
                    Cell::from(aircraft),
                    Cell::from(time.to_string()),
                ])
            });
            // TODO sort rows

            let table = Table::new(rows)
                .header(
                    Row::new([
                        Cell::from("Pilot callsign"),
                        Cell::from("Aircraft"),
                        Cell::from("Time piloting"),
                    ])
                    .style(*NORMAL_STYLE)
                    .height(1),
                )
                .widths(&[
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ])
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("Pilots near {}", airport)),
                )
                .highlight_style(*SELECTED_STYLE)
                .highlight_symbol(">> ");
            f.render_stateful_widget(table, chunks[1], &mut app.table_state);
        })?;

        if event::poll(Duration::from_secs(15))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => app.up(pilots.len()),
                    KeyCode::Down => app.down(pilots.len()),
                    _ => {}
                }
            }
        }
    }

    // exit, restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
