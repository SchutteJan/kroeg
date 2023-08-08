use diesel::prelude::*;
use postgis_diesel::types::Point;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Location {
    pub id: i32,
    pub name: String,
    pub coordinates: Point,
    pub published: bool,
    pub description: Option<String>,
    pub osm_node_id: Option<String>,
    pub google_place_id: Option<String>,
}

#[derive(Serialize, JsonSchema)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, JsonSchema)]
pub struct LocationResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub coordinates: Coordinate,
}

// TODO(jans): Implement traits for Serde and JsonSchema for Point type
impl From<Point> for Coordinate {
    fn from(value: Point) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<&Location> for LocationResponse {
    fn from(l: &Location) -> Self {
        Self {
            id: l.id,
            name: l.name.clone(),
            description: l.description.clone(),
            coordinates: Coordinate::from(l.coordinates),
        }
    }
}
