use reqwest::{Client, Error};
use crate::models::cable_model::*;

const CABLE_URL: &str = "http://localhost:8080/cable";

pub async fn get_cables(client: &Client) -> Result<Vec<Cable>, Error> {
    let resp = client.get(CABLE_URL).send().await?;

    resp.json::<Vec<Cable>>().await
}

pub async fn create_cable(client: &Client, cable: &NewCable) -> Result<Cable, Error> {
    let resp = client.post(CABLE_URL).json(&cable).send().await?;

    resp.json::<Cable>().await
}

pub async fn update_cable(client: &Client, cable: &Cable) -> Result<Cable, Error> {
    let resp = client.put(CABLE_URL).json(&cable).send().await?;

    resp.json::<Cable>().await
}

pub async fn delete_cable(client: &Client, cable_id: i32) -> Result<(), Error> {
    let url = format!("{}/{}", CABLE_URL, cable_id);
    let _resp = client.delete(&url).send().await?;

    Ok(())
}