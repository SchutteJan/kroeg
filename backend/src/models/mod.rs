pub mod locations;
pub mod users;
mod visits;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

#[derive(Deserialize)]
pub struct DeleteRequest {
    pub id: i32,
}
