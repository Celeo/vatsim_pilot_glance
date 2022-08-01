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

use crate::api::Vatsim;
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    /// Airport to monitor the area around
    airport: Option<String>,
    /// Show supported airports
    #[clap(long)]
    show_airports: bool,
}

fn main() {
    let args = Args::parse();
    let vatsim = Vatsim::new().expect("Could not set up access to VATSIM API");
    if args.airport.is_none() {
        eprintln!("No specified airport");
        return;
    }
    interface::run(&vatsim, &args.airport.unwrap()).expect("Could not set up interface");
}
