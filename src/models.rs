//! JSON API models.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StatusData {
    pub v3: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Status {
    pub data: StatusData,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pilot {
    pub cid: u64,
    pub name: String,
    pub callsign: String,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: i64,
    pub transponder: String,
    pub logon_time: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct V3ResponseData {
    pub pilots: Vec<Pilot>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RatingsData {
    pub pilot: f64,
}
