use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug, Default)]
pub enum Gender {
    #[default]
    Male,
    Female
}

impl Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct CableType {
    pub id: i32,
    pub name: String,
    pub cable_gender: Gender,
    pub image: Vec<u8>
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct NewCableType {
    pub name: String,
    pub cable_gender: Gender,
    pub image: Vec<u8>
}