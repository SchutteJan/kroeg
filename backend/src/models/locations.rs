use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Selectable;
use postgis_diesel::types::Point;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use schemars::JsonSchema;

use crate::schema::locations;

#[derive(Queryable, Selectable, Serialize, Deserialize, Identifiable)]
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
    pub address_line: String,
    pub gem_ams_id: Option<i64>,
}

#[derive(Serialize, JsonSchema, Queryable)]
pub struct LocationResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub google_place_id: Option<String>,
    pub coordinates: Point,
    pub imageurl: Option<String>,
    pub address_line: String,
    pub visited_at: Option<NaiveDateTime>,
    pub area_name: Option<String>,
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
    pub address_line: Option<String>,
    pub gem_ams_id: Option<i64>,
}

#[derive(Deserialize, Queryable, AsChangeset)]
#[diesel(table_name = locations)]
pub struct UpdateLocation {
    pub name: Option<String>,
    pub coordinates: Option<Point>,
    pub published: Option<bool>,
    pub description: Option<Option<String>>,
    pub osm_node_id: Option<Option<String>>,
    pub google_place_id: Option<Option<String>>,
    pub imageurl: Option<Option<String>>,
    pub address_line: Option<String>,
    pub gem_ams_id: Option<Option<i64>>,
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
            address_line: bar.address_line.clone(),
            gem_ams_id: bar.gem_ams_id,
        }
    }
}

impl From<&Location> for LocationResponse {
    fn from(l: &Location) -> Self {
        Self {
            id: l.id,
            name: l.name.clone(),
            description: l.description.clone(),
            google_place_id: l.google_place_id.clone(),
            coordinates: l.coordinates,
            imageurl: l.imageurl.clone(),
            visited_at: None,
            address_line: l.address_line.clone(),
            area_name: None,
        }
    }
}
