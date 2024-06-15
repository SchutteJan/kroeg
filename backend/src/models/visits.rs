use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use schemars::JsonSchema;

use crate::schema::visits;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Visit {
    pub id: i32,
    pub user_id: i32,
    pub location_id: i32,
    pub visited_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = visits)]
pub struct NewVisit {
    pub user_id: i32,
    pub location_id: i32,
    pub visited_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct VisitStats {
    pub distinct_bar_visits: i64,
}
