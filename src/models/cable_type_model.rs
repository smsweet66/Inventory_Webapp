use std::fmt::Display;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
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
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    #[validate(length(min = 1, message = "Image is required"))]
    pub image: Vec<u8>
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate)]
pub struct NewCableType {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    pub cable_gender: Gender,
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    #[validate(length(min = 1, message = "Image is required"))]
    pub image: Vec<u8>
}

fn as_base64<T: AsRef<[u8]>, S: Serializer>(val: &T, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&STANDARD.encode(val))
}

fn from_base64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
    use serde::de;

    <&str>::deserialize(deserializer).and_then(|s| {
        STANDARD.decode(s)
            .map_err(|e| de::Error::custom(format!("invalid base64 string: {}, {}", s, e)))
    })
}