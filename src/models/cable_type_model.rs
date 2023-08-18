use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum Gender {
    Male,
    Female
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CableType {
    pub id: i32,
    pub name: String,
    pub cable_gender: Gender,
    pub image: Vec<u8>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewCableType {
    pub name: String,
    pub cable_gender: Gender,
    pub image: Vec<u8>
}