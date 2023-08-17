use reqwest::blocking::Client;
use std::error::Error;
use crate::models::cable_type_model::*;

const cable_type_url: &str = "http://localhost:8080/cable_type";

pub fn get_cable_types(client: Client) -> Result<Vec<CableType>, Box<dyn Error>> {
    let cable_types = client.get(cable_type_url)
        .send()?
        .json::<Vec<CableType>>()?;
    Ok(cable_types)
}

pub fn create_cable_type(client: Client, new_cable_type: NewCableType) -> Result<CableType, Box<dyn std::error::Error>> {
    let cable_type = client.post(cable_type_url)
        .json(&new_cable_type)
        .send()?
        .json::<CableType>()?;
    Ok(cable_type)
}

pub fn update_cable_type(client: Client, cable_type: CableType) -> Result<CableType, Box<dyn std::error::Error>> {
    let cable_type = client.put(cable_type_url)
        .json(&cable_type)
        .send()?
        .json::<CableType>()?;
    Ok(cable_type)
}

pub fn delete_cable_type(client: Client, cable_type_id: i32) -> Result<(), Box<dyn std::error::Error>> {
    client.delete(format!("{}/{}", cable_type_url, cable_type_id))
        .send()?;
    Ok(())
}