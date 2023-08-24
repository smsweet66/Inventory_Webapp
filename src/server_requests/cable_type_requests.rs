use reqwasm::{Error, http::Request};

use crate::models::cable_type_model::*;

const CABLE_TYPE_URL: &str = "http://192.168.2.51:8080/cable_type";

pub async fn get_cable_types() -> Result<Vec<CableType>, Error> {
    let res = Request::get(CABLE_TYPE_URL).send().await?;
    
    res.json().await
}

pub async fn create_cable_type(cable_type: &NewCableType) -> Result<CableType, Error> {
    let res = Request::post(CABLE_TYPE_URL)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(cable_type)?)
        .send()
        .await?;

    res.json().await
}

pub async fn update_cable_type(cable_type: &CableType) -> Result<CableType, Error> {
    let res = Request::put(CABLE_TYPE_URL)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(cable_type)?)
        .send()
        .await?;

    res.json().await
}

pub async fn delete_cable_type(cable_type_id: i32) -> Result<(), Error> {
    let url = format!("{}/{}", CABLE_TYPE_URL, cable_type_id);
    match Request::delete(&url).send().await {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}