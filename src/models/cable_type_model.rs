use std::fmt::Display;

use serde::{Serialize, Deserialize};
use validator::Validate;

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

impl Gender {
    pub fn from_string(value: &str) -> Self {
        match value {
            "Female" => Gender::Female,
            _ => Gender::Male
        }
    }

    pub fn to_string_vec() -> Vec<String> {
        vec!["Male".to_owned(), "Female".to_owned()]
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default, Validate)]
pub struct CableType {
    pub id: i32,
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    pub cable_gender: Gender,
    pub image: Vec<u8>
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate)]
pub struct NewCableType {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    pub cable_gender: Gender,
    pub image: Vec<u8>
}