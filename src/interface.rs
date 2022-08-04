use crate::{
    api::Vatsim,
    models::{Pilot, RatingsData},
    state::App,
    static_data,
};
use anyhow::Result;
use chrono::{SecondsFormat, Utc};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use num_format::{Locale, ToFormattedString};
use once_cell::sync::Lazy;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Terminal,
};

const INSTRUCTIONS: &str =
    "   Refreshes every 15 seconds. Up down to select a row, then 'O' to view the pilot's online stats page.";
/// Style applied to the table header row.
static NORMAL_STYLE: Lazy<Style> = Lazy::new(|| Style::default().bg(Color::Blue));
/// Style applied to non-header table rows.
static SELECTED_STYLE: Lazy<Style> =
    Lazy::new(|| Style::default().add_modifier(Modifier::REVERSED));

/// Update the app's data.
fn update_data(
    vatsim: &Vatsim,
    app: &mut App,
    airport: &str,
    view_distance: f64,
) -> Result<Vec<(Pilot, RatingsData)>> {
    let online_pilots = vatsim.get_online_pilots()?;
    let pilots_in_range =
        static_data::filter_pilot_distance(&online_pilots, airport, view_distance)?;
    let pilot_times: Vec<(Pilot, RatingsData)> = pilots_in_range
        .par_iter()
        .map(|&pilot| {
            if let Some(time) = app.pilot_time_cached(pilot.cid) {
                (pilot.clone(), time)
            } else {
                let time = vatsim
                    .get_ratings_times(pilot.cid)
                    .expect("Could not get pilot time");
                (pilot.clone(), time)
            }
        })
        .collect();
    for (pilot, time) in &pilot_times {
        app.update_pilot_time_cache(pilot.cid, time);
    }
    Ok(pilot_times)
}

/// Run the TUI.
#[allow(clippy::too_many_lines)]
pub fn run(vatsim: &Vatsim, airport: &str, view_distance: f64) -> Result<()> {
    // configure terminal
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    enable_raw_mode()?;
    terminal.hide_cursor()?;
    let mut app = App::new();

    let mut last_updated_time = Utc::now();
    let mut last_updated = Instant::now();
    let mut pilots = update_data(vatsim, &mut app, airport, view_distance)?;
    pilots.sort_unstable_by(|(_, a), (_, b)| a.pilot.partial_cmp(&b.pilot).unwrap());

    loop {
        if last_updated.elapsed() >= Duration::from_secs(15) {
            pilots = update_data(vatsim, &mut app, airport, view_distance)?;
            pilots.sort_unstable_by(|(_, a), (_, b)| a.pilot.partial_cmp(&b.pilot).unwrap());
            last_updated = Instant::now();
            last_updated_time = Utc::now();
            app.table_state.select(None);
        }

        let _ = terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .horizontal_margin(1)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(f.size());

            let title_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length((INSTRUCTIONS.len() + 5).try_into().unwrap()),
                    Constraint::Min(1),
                    Constraint::Length(28),
                ])
                .split(chunks[0]);

            f.render_widget(
                Paragraph::new(Text::from(INSTRUCTIONS))
                    .block(Block::default().borders(Borders::ALL).title("Instructions")),
                title_chunks[0],
            );
            f.render_widget(
                Paragraph::new(Text::from(format!(
                    "{: ^26}",
                    last_updated_time.to_rfc3339_opts(SecondsFormat::Secs, true)
                )))
                .block(Block::default().borders(Borders::ALL).title("Last updated")),
                title_chunks[2],
            );

            let rows = pilots.iter().map(|(pilot, ratings_data)| {
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
                    #[allow(clippy::cast_possible_truncation)]
                    Cell::from(
                        (ratings_data.pilot.round() as i64).to_formatted_string(&Locale::en),
                    ),
                    #[allow(clippy::cast_possible_truncation)]
                    Cell::from((ratings_data.atc.round() as i64).to_formatted_string(&Locale::en)),
                ])
            });

            let table = Table::new(rows)
                .header(
                    Row::new([
                        Cell::from("Pilot callsign"),
                        Cell::from("Aircraft"),
                        Cell::from("Time piloting (hours)"),
                        Cell::from("Time controlling (hours)"),
                    ])
                    .style(*NORMAL_STYLE)
                    .height(1),
                )
                .widths(&[
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                    Constraint::Percentage(35),
                    Constraint::Percentage(35),
                ])
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("Pilots near {}", airport)),
                )
                .highlight_style(*SELECTED_STYLE);
            f.render_stateful_widget(table, chunks[1], &mut app.table_state);
        })?;

        if event::poll(Duration::from_secs(15))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => app.up(pilots.len()),
                    KeyCode::Down => app.down(pilots.len()),
                    KeyCode::Char('o') => {
                        let cid = pilots.get(app.tab_index).unwrap().0.cid;
                        webbrowser::open(&format!("https://stats.vatsim.net/stats/{}", cid))
                            .expect("Could not open web browser");
                    }
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
