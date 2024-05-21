pub mod locations;
use diesel::prelude::*;
pub use postgis_diesel::types::Point;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

#[derive(Deserialize)]
pub struct DeleteRequest {
    pub id: i32,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub password: String,
}
