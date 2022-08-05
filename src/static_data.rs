use crate::models::Pilot;
use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use std::{collections::HashMap, f64::consts::PI};

/// List of supported airports.
pub static AIRPORTS: Lazy<Vec<&'static str>> = Lazy::new(|| vec!["KSAN", "KLAX", "KSNA", "KLAS"]);

/// Mapping of airport locations for use in calculating distance.
static AIRPORT_LOCATIONS: Lazy<HashMap<&'static str, (f64, f64)>> = Lazy::new(|| {
    let mut m = HashMap::new();
    let _ = m.insert("KSAN", (32.7338, -117.1933));
    let _ = m.insert("KLAX", (33.9416, -118.4085));
    let _ = m.insert("KSNA", (33.6762, -117.8675));
    let _ = m.insert("KLAS", (36.084, -115.1537));
    m
});

/// Calculate the Haversine Distance between two (lat & long) points.
///
/// Originally from <https://www.movable-type.co.uk/scripts/latlong.html>.
pub fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371e3;
    let φ1 = (lat1 * PI) / 180_f64;
    let φ2 = (lat2 * PI) / 180_f64;
    #[allow(non_snake_case)]
    let Δφ = ((lat2 - lat1) * PI) / 180_f64;
    #[allow(non_snake_case)]
    let Δλ = ((lon2 - lon1) * PI) / 180_f64;
    let a = f64::sin(Δφ / 2_f64) * f64::sin(Δφ / 2_f64)
        + f64::cos(φ1) * f64::cos(φ2) * f64::sin(Δλ / 2_f64) * f64::sin(Δλ / 2_f64);
    let c = 2_f64 * f64::atan2(f64::sqrt(a), f64::sqrt(1_f64 - a));
    let d = r * c;
    f64::round(d * 0.00054)
}

/// Filter the list of pilots down to those in the specified range of the airport.
pub fn filter_pilot_distance<'a>(
    pilots: &'a [Pilot],
    airport: &str,
    distance: f64,
) -> Result<Vec<&'a Pilot>> {
    let [airport_lat, airport_lon] = match AIRPORT_LOCATIONS.get(airport) {
        Some(loc) => [loc.0, loc.1],
        None => return Err(anyhow!("Unsupported airport {}", airport)),
    };
    Ok(pilots
        .iter()
        .filter(|&pilot| {
            haversine_distance(airport_lat, airport_lon, pilot.latitude, pilot.longitude) < distance
        })
        .collect())
}
