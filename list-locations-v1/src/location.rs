use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Coordinate {
    pub(crate) lat: f64,
    pub(crate) lon: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Toilet {
    pub code: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wifi {
    pub code: Option<String>,
    pub is_open_network: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) location: Coordinate,
    pub(crate) toilet: Option<Toilet>,
    pub(crate) wifi: Option<Wifi>,
}