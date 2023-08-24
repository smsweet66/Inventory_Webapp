use reqwasm::{Error, http::Request};
use crate::models::cable_model::*;

const CABLE_URL: &str = "http://192.168.2.51:8080/cable";

pub async fn get_cables() -> Result<Vec<Cable>, Error> {
    let res = Request::get(CABLE_URL).send().await?;
    
    res.json().await
}

pub async fn create_cable(cable: &NewCable) -> Result<Cable, Error> {
    log::info!("request: {}", serde_json::to_string(cable)?);
    let res = Request::post(CABLE_URL)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(cable)?)
        .send()
        .await?;

    res.json().await
}

pub async fn update_cable(cable: &Cable) -> Result<Cable, Error> {
    let res = Request::put(CABLE_URL)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(cable)?)
        .send()
        .await?;

    res.json().await
}

pub async fn delete_cable(cable_id: i32) -> Result<(), Error> {
    let url = format!("{}/{}", CABLE_URL, cable_id);
    match Request::delete(&url).send().await {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}