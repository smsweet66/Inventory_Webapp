use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug, Validate, Default)]
pub struct Cable {
    pub id: i32,
    #[validate(range(min = 1, max = 1000000, message = "End A must be between 1 and 1000000"))]
    pub end_a: i32,
    #[validate(range(min = 1, max = 1000000, message = "End B must be between 1 and 1000000"))]
    pub end_b: i32,
    #[validate(range(min = 0.0, max = 1000.0, message = "Cable length must be between 0 and 1000 feet"))]
    pub cable_length: f32
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Validate, Default)]
pub struct NewCable {
    #[validate(range(min = 1, max = 1000000, message = "End A must be between 1 and 1000000"))]
    pub end_a: i32,
    #[validate(range(min = 1, max = 1000000, message = "End B must be between 1 and 1000000"))]
    pub end_b: i32,
    #[validate(range(min = 0.0, max = 1000.0, message = "Cable length must be between 0 and 1000 feet"))]
    pub cable_length: f32
}