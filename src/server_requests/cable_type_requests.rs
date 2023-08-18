use reqwest::{Client, Error};
use crate::models::cable_type_model::*;

const CABLE_TYPE_URL: &str = "http://localhost:8080/cable_type";

pub async fn get_cable_types(client: &Client) -> Result<Vec<CableType>, Error> {
    let resp = client.get(CABLE_TYPE_URL).send().await?;

    resp.json::<Vec<CableType>>().await
}

pub async fn create_cable_type(client: &Client, cable_type: &NewCableType) -> Result<CableType, Error> {
    let resp = client.post(CABLE_TYPE_URL).json(&cable_type).send().await?;

    resp.json::<CableType>().await
}

pub async fn update_cable_type(client: &Client, cable_type: &CableType) -> Result<CableType, Error> {
    let resp = client.put(CABLE_TYPE_URL).json(&cable_type).send().await?;

    resp.json::<CableType>().await
}

pub async fn delete_cable_type(client: &Client, cable_type_id: i32) -> Result<(), Error> {
    let url = format!("{}/{}", CABLE_TYPE_URL, cable_type_id);
    let _resp = client.delete(&url).send().await?;

    Ok(())
}