use reqwest::blocking::Client;
use std::error::Error;
use crate::models::cable_model::*;

const cable_url: &str = "http://localhost:8080/cable";

pub fn get_cables(client: Client) -> Result<Vec<Cable>, Box<dyn Error>> {
    let cables = client.get(cable_url)
        .send()?
        .json::<Vec<Cable>>()?;
    Ok(cables)
}

pub fn create_cable(client: Client, new_cable: NewCable) -> Result<Cable, Box<dyn Error>> {
    let cable = client.post(cable_url)
        .json(&new_cable)
        .send()?
        .json::<Cable>()?;
    Ok(cable)
}

pub fn update_cable(client: Client, cable: Cable) -> Result<Cable, Box<dyn Error>> {
    let cable = client.put(cable_url)
        .json(&cable)
        .send()?
        .json::<Cable>()?;
    Ok(cable)
}

pub fn delete_cable(client: Client, cable_id: i32) -> Result<(), Box<dyn Error>> {
    client.delete(format!("{}/{}", cable_url, cable_id))
        .send()?;
    Ok(())
}