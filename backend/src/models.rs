use diesel::prelude::*;
use postgis_diesel::types::Point;
use serde::{Serialize, Deserialize};

#[derive(Queryable,Serialize, Deserialize)]
pub struct Location {
    pub id: i32,
    pub name: String,
    pub coordinates: Point,
    pub published: bool,
    pub description: Option<String>,
    pub osm_node_id: Option<String>,
    pub google_place_id: Option<String>
}