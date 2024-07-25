use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use schemars::JsonSchema;

use crate::schema::visits;

// TODO: Check diesel associations can make queries easier
//       #[diesel(belongs_to(User))]
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
    pub total_bars_by_area: Vec<(String, i64)>,
    pub bar_visits_by_area: Vec<(String, i64)>,
}
