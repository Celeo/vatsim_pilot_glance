#![deny(
    clippy::all,
    clippy::pedantic,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

mod api;
mod interface;
mod models;
mod state;
mod static_data;

use crate::{api::Vatsim, static_data::AIRPORTS};
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    /// Airport to monitor the area around
    airport: Option<String>,
    /// Show supported airports
    #[clap(long)]
    show_airports: bool,
    /// View distance
    #[clap(short = 'd', long, default_value_t = 20.0)]
    view_distance: f64,
}

/// Entry points.
fn main() {
    let args = Args::parse();
    let vatsim = Vatsim::new().expect("Could not set up access to VATSIM API");
    if args.show_airports {
        println!("Supported airports: {}", AIRPORTS.join(", "));
        return;
    }
    let airport = if let Some(a) = args.airport {
        a
    } else {
        eprintln!("No specified airport");
        return;
    };
    if !AIRPORTS.contains(&airport.as_str()) {
        eprintln!(
            "Airport \"{}\" not found in supported list: {}",
            airport,
            AIRPORTS.join(", ")
        );
        return;
    }
    interface::run(&vatsim, &airport, args.view_distance).expect("Could not set up interface");
}
