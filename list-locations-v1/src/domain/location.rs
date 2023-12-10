use crate::utils::dynamodb::{as_float, as_string, as_unsigned_int};
use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Coordinate {
    pub(crate) lat: f64,
    pub(crate) lon: f64,
}

impl From<&HashMap<String, AttributeValue>> for Coordinate {
    fn from(value: &HashMap<String, AttributeValue>) -> Self {
        let lat = as_float(value.get("lat")).unwrap();
        let lon = as_float(value.get("lon")).unwrap();

        Coordinate { lat, lon }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Toilet {
    pub code: u32,
}

impl From<&HashMap<String, AttributeValue>> for Toilet {
    fn from(value: &HashMap<String, AttributeValue>) -> Self {
        let code = as_unsigned_int(value.get("code")).unwrap();

        Toilet { code }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wifi {
    pub code: Option<String>,
    pub is_open_network: bool,
}

impl From<&HashMap<String, AttributeValue>> for Wifi {
    fn from(value: &HashMap<String, AttributeValue>) -> Self {
        let code = as_string(value.get("code"));
        let is_open_network = value
            .get("is_open_network")
            .unwrap()
            .as_bool()
            .copied()
            .unwrap_or(false);

        Wifi {
            code,
            is_open_network,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) location: Coordinate,
    pub(crate) toilet: Option<Toilet>,
    pub(crate) wifi: Option<Wifi>,
}

impl From<&HashMap<String, AttributeValue>> for Location {
    fn from(value: &HashMap<String, AttributeValue>) -> Self {
        let id: String = as_string(value.get("id")).unwrap();
        let name: String = as_string(value.get("name")).unwrap();

        let location = value.get("location").unwrap().as_m().unwrap();

        return Location {
            id,
            name,
            location: location.into(),
            toilet: value.get("toilet").map(|it| it.as_m().unwrap().into()),
            wifi: None,
        };
    }
}
