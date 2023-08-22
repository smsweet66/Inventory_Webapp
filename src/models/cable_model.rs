use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub struct Cable {
    pub id: i32,
    pub end_a: i32,
    pub end_b: i32,
    pub cable_length: f32
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Validate, Default)]
pub struct NewCable {
    pub end_a: i32,
    pub end_b: i32,
    #[validate(range(min = 0.0, max = 1000.0, message = "Cable length must be between 0 and 1000 feet"))]
    pub cable_length: f32
}