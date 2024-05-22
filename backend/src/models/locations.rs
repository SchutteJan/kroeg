use diesel::prelude::*;
use diesel::Selectable;
use postgis_diesel::types::Point;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use schemars::JsonSchema;

use crate::models::Coordinate;
use crate::schema::locations;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Location {
    pub id: i32,
    pub name: String,
    pub coordinates: Point,
    pub published: bool,
    pub description: Option<String>,
    pub osm_node_id: Option<String>,
    pub google_place_id: Option<String>,
    pub imageurl: Option<String>,
}

#[derive(Serialize, JsonSchema)]
pub struct LocationResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub coordinates: Coordinate,
    pub imageurl: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = locations)]
pub struct NewLocation {
    pub name: String,
    pub coordinates: Point,
    pub published: bool,
    pub description: Option<String>,
    pub osm_node_id: Option<String>,
    pub google_place_id: Option<String>,
    pub imageurl: Option<String>,
}

impl NewLocation {
    pub fn from_json(bar: &Json<NewLocation>) -> Self {
        Self {
            name: bar.name.clone(),
            coordinates: Point {
                x: bar.coordinates.x,
                y: bar.coordinates.y,
                srid: Some(4326),
            },
            published: bar.published,
            description: bar.description.clone(),
            osm_node_id: bar.osm_node_id.clone(),
            google_place_id: bar.google_place_id.clone(),
            imageurl: bar.imageurl.clone(),
        }
    }
}

// TODO: Implement traits for Serde and JsonSchema for Point type
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
            imageurl: l.imageurl.clone(),
        }
    }
}
