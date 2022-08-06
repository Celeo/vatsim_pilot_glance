use crate::state::App;
use anyhow::Result;
use chrono::{SecondsFormat, Utc};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use num_format::{Locale, ToFormattedString};
use once_cell::sync::Lazy;
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Terminal,
};
use vatsim_utils::{
    distance::{haversine, Airport},
    live_api::Vatsim,
    models::{Pilot, RatingsTimeData},
    rest_api::{get_ratings_times, stats_url},
};

const INSTRUCTIONS: &str =
    "   Auto-refreshes every 15 sec. Up/down to select row & 'O' to view stats.";
/// Style applied to the table header row.
static NORMAL_STYLE: Lazy<Style> = Lazy::new(|| Style::default().bg(Color::Blue));
/// Style applied to non-header table rows.
static SELECTED_STYLE: Lazy<Style> =
    Lazy::new(|| Style::default().add_modifier(Modifier::REVERSED));

/// Update the app's data.
async fn update_data(
    vatsim: &Vatsim,
    app: &mut App,
    airport: &Airport,
    view_distance: f64,
) -> Result<Vec<(Pilot, RatingsTimeData)>> {
    // get current online pilots, filter to those in range of the airport
    let v3_data = vatsim.get_v3_data().await.unwrap();
    let pilots_in_range: Vec<_> = v3_data
        .pilots
        .iter()
        .filter(|&pilot| {
            haversine(
                pilot.latitude,
                pilot.longitude,
                airport.latitude,
                airport.longitude,
            ) < view_distance
        })
        .cloned()
        .collect();

    // retrieve position times from cache or REST API
    let mut pilot_times = Vec::new();
    for pilot in pilots_in_range {
        if let Some(time) = app.pilot_time_cached(pilot.cid) {
            pilot_times.push((pilot, time));
        } else {
            let time = get_ratings_times(pilot.cid).await.unwrap();
            pilot_times.push((pilot, time));
        }
    }

    // update cache
    for (pilot, time) in &pilot_times {
        app.update_pilot_time_cache(pilot.cid, time);
    }

    Ok(pilot_times)
}

/// Run the TUI.
#[allow(clippy::too_many_lines)]
pub async fn run(vatsim: &Vatsim, airport: &Airport, view_distance: f64) -> Result<()> {
    // configure terminal
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    enable_raw_mode()?;
    terminal.hide_cursor()?;

    // data and timestamps
    let mut app = App::new();
    let mut last_updated = Instant::now() - Duration::from_secs(20);
    let mut pilots = Vec::new();
    let mut last_updated_time = Utc::now();

    loop {
        // update data every 15 seconds (API refresh rate)
        if last_updated.elapsed() >= Duration::from_secs(15) {
            pilots = update_data(vatsim, &mut app, airport, view_distance).await?;
            pilots.sort_unstable_by(|(_, a), (_, b)| a.pilot.partial_cmp(&b.pilot).unwrap());
            last_updated = Instant::now();
            last_updated_time = Utc::now();
            app.table_state.select(None);
        }

        // draw the UI
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
                .block(Block::default().borders(Borders::ALL).title(format!(
                    "Pilots within {} nm of {}",
                    view_distance, airport.identifier
                )))
                .highlight_style(*SELECTED_STYLE);
            f.render_stateful_widget(table, chunks[1], &mut app.table_state);
        })?;

        // keyboard interaction
        if event::poll(Duration::from_secs(15))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Esc => app.table_state.select(None),
                    KeyCode::Up => app.up(pilots.len()),
                    KeyCode::Down => app.down(pilots.len()),
                    KeyCode::Char('o') => {
                        if let Some(index) = app.table_state.selected() {
                            let cid = pilots.get(index).unwrap().0.cid;
                            webbrowser::open(&stats_url(cid)).expect("Could not open web browser");
                        }
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
