use rocket_sync_db_pools::database;

pub mod areas;
pub mod locations;
pub mod pgcrypto;
pub mod sql_types;
pub mod users;
pub mod visits;

#[database("webapp")]
pub struct DbConn(diesel::PgConnection);
