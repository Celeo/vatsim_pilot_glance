#![deny(clippy::all)]
#![deny(unsafe_code)]

mod interface;
mod state;

use clap::Parser;
use vatsim_utils::{distance::AIRPORTS, live_api::Vatsim};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    /// Airport to monitor the area around
    airport: Option<String>,
    /// View distance
    #[clap(short = 'd', long, default_value_t = 20.0)]
    view_distance: f64,
}

/// Entry points.
#[tokio::main]
async fn main() {
    let args = Args::parse();
    let airport = if let Some(airport_str) = args.airport {
        if let Some(airport_ref) = AIRPORTS.iter().find(|&ap| ap.identifier == airport_str) {
            airport_ref
        } else {
            eprintln!("Unknown airport \"{airport_str}\"");
            return;
        }
    } else {
        eprintln!("No specified airport");
        return;
    };
    let vatsim = Vatsim::new()
        .await
        .expect("Could not set up access to VATSIM API");
    interface::run(&vatsim, airport, args.view_distance)
        .await
        .expect("Could not set up interface");
}
