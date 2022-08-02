use crate::models::{Pilot, RatingsData, Status, V3ResponseData};
use anyhow::{anyhow, Result};
use rand::seq::SliceRandom;
use reqwest::blocking::{Client, ClientBuilder};

/// Initial VATSIM API requests are made to this endpoint.
const STATUS_URL: &str = "https://status.vatsim.net/status.json";

/// API struct.
pub struct Vatsim {
    client: Client,
    v3_url: String,
}

impl Vatsim {
    /// New API struct instance.
    ///
    /// Makes the API call to the status endpoint to get the endpoint
    /// to make V3 API calls.
    pub fn new() -> Result<Self> {
        let client = ClientBuilder::new()
            .user_agent("github.com/celeo/vatsim_online")
            .build()?;
        let url = Vatsim::get_v3_url(&client)?;
        Ok(Self {
            client,
            v3_url: url,
        })
    }

    /// Get the V3 URL by querying the status endpoint.
    fn get_v3_url(client: &Client) -> Result<String> {
        let response = client.get(STATUS_URL).send()?;
        if !response.status().is_success() {
            return Err(anyhow!(
                "Got status {} from status endpoint",
                response.status().as_u16()
            ));
        }
        let data: Status = response.json()?;
        let url = data
            .data
            .v3
            .choose(&mut rand::thread_rng())
            .ok_or_else(|| anyhow!("No V3 URLs returned"))?
            .clone();
        Ok(url)
    }

    /// Query the stored V3 endpoint.
    pub fn get_online_pilots(&self) -> Result<Vec<Pilot>> {
        let response = self.client.get(&self.v3_url).send()?;
        if !response.status().is_success() {
            return Err(anyhow!(
                "Got status {} from status endpoint",
                response.status().as_u16()
            ));
        }
        let data: V3ResponseData = response.json()?;
        Ok(data.pilots)
    }

    /// Get the amount of time the user has spent as a pilot on the network.
    pub fn get_pilot_time(&self, cid: u64) -> Result<f64> {
        let response = self
            .client
            .get(format!(
                "https://api.vatsim.net/api/ratings/{}/rating_times",
                cid
            ))
            .send()?;
        if !response.status().is_success() {
            return Err(anyhow!(
                "Got status {} from ratings endpoint",
                response.status().as_u16()
            ));
        }
        let data: RatingsData = response.json()?;
        Ok(data.pilot)
    }
}
