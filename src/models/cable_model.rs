use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub struct Cable {
    pub id: i32,
    pub end_a: i32,
    pub end_b: i32,
    pub cable_length: f32
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct NewCable {
    pub end_a: i32,
    pub end_b: i32,
    pub cable_length: f32
}