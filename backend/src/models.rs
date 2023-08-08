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
pub struct LocationResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

impl From<&Location> for LocationResponse {
    fn from(l: &Location) -> Self {
        Self {
            id: l.id,
            name: l.name.clone(),
            description: l.description.clone(),
        }
    }
}
