use diesel::{Queryable, Selectable};
use postgis_diesel::types::{Point, Polygon};
use rocket::serde::Serialize;
use schemars::JsonSchema;

use crate::db::sql_types::AreaTypeEnum;
use crate::schema::areas;

#[derive(Serialize, JsonSchema, Queryable, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Area {
    pub id: i32,
    pub name: String,
    pub area: Polygon<Point>,
    pub area_type: AreaTypeEnum,
}
